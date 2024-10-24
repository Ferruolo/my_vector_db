use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

struct BlockDevice {
    file: File,
    block_size: usize,
    alignment_mask: u64,
}

#[cfg(target_os = "linux")]
fn get_block_size(file: &File) -> std::io::Result<usize> {
    use std::os::raw::c_ulong;
    println!("Called ME");
    const BLKSSZGET: c_ulong = 0x1268;
    let mut size: i32 = 0;

    // Safety: This ioctl is safe as we're passing a valid file descriptor
    // and a properly aligned integer to receive the block size
    unsafe {
        let result = libc::ioctl(file.as_raw_fd(), BLKSSZGET, &mut size);
        if result == -1 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(size as usize)
}

#[cfg(not(target_os = "linux"))]
fn get_block_size(_file: &File) -> std::io::Result<usize> {
    // Default to 512 bytes, which is common for many devices
    // In production, you'd want to implement proper detection for other OS
    Ok(512)
}

impl BlockDevice {
    /// Opens a file as a block device with specified block size
    pub fn new<P: AsRef<Path>>(path: P, block_size: Option<usize>) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        // If block_size is not provided, detect it from the device
        let block_size = block_size.unwrap_or_else(|| get_block_size(&file).unwrap_or(512));

        // Ensure block size is a power of 2
        if !block_size.is_power_of_two() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Block size must be a power of 2"
            ));
        }

        // Calculate alignment mask (one less than block size)
        let alignment_mask = (block_size as u64) - 1;

        Ok(BlockDevice {
            file,
            block_size,
            alignment_mask,
        })
    }

    /// Checks if an offset is aligned to block boundaries
    pub fn is_aligned(&self, offset: u64) -> bool {
        (offset & self.alignment_mask) == 0
    }

    /// Aligns an offset up to the next block boundary
    pub fn align_up(&self, offset: u64) -> u64 {
        (offset + self.alignment_mask) & !self.alignment_mask
    }

    /// Get the block size of the device
    pub fn get_block_size(&self) -> usize {
        self.block_size
    }

    /// Reads a block at the specified index with alignment verification
    pub fn read_block(&mut self, block_index: u64) -> std::io::Result<Vec<u8>> {
        let offset = block_index * self.block_size as u64;

        // Verify alignment
        if !self.is_aligned(offset) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Read offset not aligned to block boundary"
            ));
        }

        let mut buffer = vec![0; self.block_size];

        // Seek to the correct position
        self.file.seek(SeekFrom::Start(offset))?;

        // Read the block
        self.file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    /// Writes a block at the specified index with alignment verification
    pub fn write_block(&mut self, block_index: u64, data: &[u8]) -> std::io::Result<()> {
        let offset = block_index * self.block_size as u64;

        // Verify alignment
        if !self.is_aligned(offset) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Write offset not aligned to block boundary"
            ));
        }

        if data.len() != self.block_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Data size does not match block size"
            ));
        }

        // Seek to the correct position
        self.file.seek(SeekFrom::Start(offset))?;

        // Write the block
        self.file.write_all(data)?;
        self.file.sync_all()?;

        Ok(())
    }

    /// Reads potentially unaligned data with automatic alignment handling
    pub fn read_unaligned(&mut self, offset: u64, length: usize) -> std::io::Result<Vec<u8>> {
        // Calculate aligned read parameters
        let start_block = offset / self.block_size as u64;
        let end_offset = offset + length as u64;
        let end_block = (end_offset + self.block_size as u64 - 1) / self.block_size as u64;

        // Read all necessary blocks
        let mut all_data = Vec::new();
        for block_idx in start_block..end_block {
            all_data.extend_from_slice(&self.read_block(block_idx)?);
        }

        // Extract the requested portion
        let start_offset = (offset % self.block_size as u64) as usize;
        Ok(all_data[start_offset..start_offset + length].to_vec())
    }
}

fn main() -> std::io::Result<()> {
    // Create a new block device with auto-detected block size
    let mut device = BlockDevice::new("test.disk", None)?;
    println!("Detected block size: {} bytes", device.get_block_size());

    // Example: Write aligned data to block 0
    let data = vec![0x55; device.get_block_size()];
    device.write_block(0, &data)?;

    // Read back the data from block 0
    let read_data = device.read_block(0)?;
    println!("Read {} bytes from block 0", read_data.len());

    // Example of unaligned read (reads across block boundaries if necessary)
    let unaligned_data = device.read_unaligned(100, 1024)?;
    println!("Read {} bytes from unaligned offset", unaligned_data.len());

    // Verify alignment
    println!("Offset 512 aligned: {}", device.is_aligned(512));
    println!("Offset 1000 aligned: {}", device.is_aligned(1000));

    // Get next aligned offset
    println!("Next aligned offset after 1000: {}", device.align_up(1000));

    Ok(())
}