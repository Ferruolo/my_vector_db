mod helpers;
mod llama_embedding;
mod node_interface;
mod types;
mod vector_db;
mod db_interface;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::db_interface::{db_interface, DbCalls};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::db_interface::DbCalls::Kill;
use crate::types::Response;

const LLAMAFILE_PATH: &str = "LLAMAFILE";
const NUM_DIMS: usize = 4096;



#[tokio::main]
async fn main()  {
    // Bind the listener to the address
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let (db_process, db_address) = db_interface(LLAMAFILE_PATH, NUM_DIMS);
    let loop_invariant = Arc::new(AtomicBool::new(true));
    while loop_invariant.load(Ordering::Relaxed) {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        let loop_invariant_clone = loop_invariant.clone();
        let db_address_copy = db_address.clone();


        tokio::spawn(async move {
            process(socket, loop_invariant_clone, db_address_copy).await;
        });
    }
    println!("Server Shutting Down");
    db_address.send(Kill).unwrap();
    db_process.await.unwrap();
}

async fn process(
    mut socket: TcpStream,
    loop_invariant: Arc<AtomicBool>,
    db_address: Sender<DbCalls>
) {
    let mut buf = [0; 8192];
    let n = socket.read(&mut buf).await.unwrap();
    let request = str::from_utf8(&buf[..n]).unwrap();

    let response = if request.starts_with("POST /insert") {

    } else if request.starts_with("GET /get") {

    } else if request.starts_with("POST /shutdown") {
        loop_invariant.store(false, Ordering::Relaxed);
        Response::Success
    } else {
        Response::Error("Request not found".to_string())
    };

    let response_json = serde_json::to_string(&response).unwrap();
    socket.write_all(response_json.as_bytes()).await.unwrap();
}

