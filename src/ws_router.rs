use warp::ws::{Message, WebSocket};

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};


pub async fn user_connected(ws: WebSocket) {
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    let msg = Message::text("welcome".to_string());

    let mut tx2 = user_ws_tx;
    let _result_welcome = tx2.send(msg).await;

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(): {}", e);
                break;
            }
        };

        user_message(msg, &mut tx2).await;
    }
    user_disconnected().await;
}

async fn user_message(msg: Message, tx2: &mut SplitSink<WebSocket, Message>) {
    let s = String::from_utf8(msg.into()).unwrap();

    let s1 = "reply: ".to_string();
    let s2 = s;
    let full_message = s1 + &s2;

    let msg = Message::text(full_message.to_string());
    let _result_msg = tx2.send(msg).await;
}

async fn user_disconnected() {
    eprintln!("good bye user: {}", 0);
}
