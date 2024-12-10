use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::RwLock;
use warp::{
    filters::ws::{Message, WebSocket},
    reject::Rejection,
    reply::Reply,
    Filter,
};

type InstanceTable = Arc<RwLock<HashMap<uuid::Uuid, tokio::sync::mpsc::UnboundedSender<Message>>>>;

#[allow(unused)]
pub struct Server {}

impl Server {
    pub fn filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let instance_table = InstanceTable::default();
        let instance_table = warp::any().map(move || instance_table.clone());
        warp::path("chat").and(warp::ws()).and(instance_table).map(
            |ws: warp::ws::Ws, instance_table| {
                ws.on_upgrade(move |socket| Self::user_connected(socket, instance_table))
            },
        )
    }

    async fn user_connected(ws: WebSocket, instance_table: InstanceTable) {
        let (mut user_ws_tx, mut user_ws_rx) = ws.split();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

        let id = uuid::Uuid::new_v4();
        instance_table.write().await.insert(id, tx);
        println!("connected: {}", id);

        tokio::task::spawn(async move {
            while let Some(message) = rx.next().await {
                println!("Send");
                user_ws_tx.send(message).await.unwrap();
            }
        });

        while let Some(result) = user_ws_rx.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={}): {}", id, e);
                    break;
                }
            };
            Self::user_message(id, msg, &instance_table).await;
        }

        Self::user_disconnected(id, &instance_table).await;
    }

    async fn user_message(id: uuid::Uuid, _msg: Message, _instance_table: &InstanceTable) {
        println!("{}", id);
    }

    async fn user_disconnected(id: uuid::Uuid, instance_table: &InstanceTable) {
        eprintln!("good bye user: {}", id);

        // Stream closed up, so remove from the user list
        instance_table.write().await.remove(&id);
    }
}
