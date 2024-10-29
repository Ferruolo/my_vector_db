use crate::vector_db::VectorDB;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use crate::db_interface::DbCalls::{FetchIndexData, FindIndexes, Insert, Null};
use crate::types::{InsertRequest, Response};

const NUM_INDEXES: usize = 10;

pub enum DbCalls {
    Insert(String, oneshot::Sender<Response>),
    FindIndexes(String, oneshot::Sender<Response>),
    FetchIndexData(Vec<usize>, oneshot::Sender<Response>),
    Kill,
    Null,
}

pub fn db_interface(llamafile_path: &str, dims: usize) -> (JoinHandle<()>, Sender<DbCalls>) {
    let (tx, mut rx): (Sender<DbCalls>, Receiver<DbCalls>) = mpsc::channel(10);

    let db_process: JoinHandle<()> = tokio::spawn(async move {
        let mut vector_db: VectorDB<String> = VectorDB::new(llamafile_path, dims);
        loop {
            match rx.recv().await.unwrap_or(Null) {
                Insert(x, return_sender) => {
                    vector_db.insert(x.clone(), x);
                    return_sender.send(Response::Success).unwrap();
                }
                FindIndexes(x, return_sender) => {
                    let indexes = vector_db.get_top_k_indexes(x, NUM_INDEXES);
                    return_sender.send(Response::Indexes(indexes)).unwrap();
                }
                FetchIndexData(indices, return_address) => {
                    let data = vector_db.get_indexes(indices);
                    return_address.send(Response::Data(data)).unwrap()
                }
                Kill => {
                    break
                }
            }
        }
    });

    (db_process, tx)
}

async fn handle_insert(request: &str, db_address: Sender<DbCalls>) -> Response {
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    if let Some(body) = request.split("\r\n\r\n").nth(1) {
        if let Ok(insert_req) = serde_json::from_str::<InsertRequest>(body) {
            db_address.send(Insert(insert_req.entry, tx)).await.unwrap();
            rx.await.unwrap()
        } else {
            Response::Error("Invalid JSON for insert".to_string())
        }
    } else {
        Response::Error("No body in insert request".to_string())
    }
}

async fn handle_get(request: &str, db_address: &mpsc::Sender<DbCalls>) -> Response {
    if let Some(id) = request.split("?id=").nth(1) {
        let id = id.split_whitespace().next().unwrap_or("");
        let (sender, receiver) = tokio::sync::oneshot::channel();
        db_address.send(FindIndexes(id.to_string(), sender)).await.unwrap();
        match receiver.await.unwrap() {
            Response::Indexes(indexes) => {
                db_address.send(DbCalls::FetchIndexData(indexes, sender)).await.unwrap();
                receiver.await.unwrap()
            }
            _ => Response::Error("Invalid Response from DB".to_string()),
        }

    } else {
        Response::Error("No id provided in get request".to_string())
    }
}