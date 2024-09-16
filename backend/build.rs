
fn main() {
    std::env::set_var("LIBTORCH", "/home/andrewf/libtorch/");
    std::env::set_var("LD_LIBRARY_PATH", "/home/andrewf/libtorch/lib:/usr/local/cuda-12.2/lib64");
    std::env::set_var("LIBTORCH_BYPASS_VERSION_CHECK", "true");
}