// src/infrastructure/websocket/heartbeat.rs
use super::{ConnectionManager, WebSocketMessage};
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use leptos::logging;
use std::{sync::Arc, time::Duration};
use tokio::time::interval;
use uuid::Uuid;

#[allow(clippy::unused_async)]
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(connection_manager): State<Arc<ConnectionManager>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_websocket(socket, connection_manager))
}

async fn handle_websocket(socket: WebSocket, connection_manager: Arc<ConnectionManager>) {
    let connection_id = Uuid::new_v4();
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<WebSocketMessage>();

    // Add connection to manager
    connection_manager
        .add_connection(connection_id, tx, None)
        .await;

    // Spawn task to send messages to client
    let _connection_manager_clone = connection_manager.clone();
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let json_message = match serde_json::to_string(&message) {
                Ok(json) => json,
                Err(e) => {
                    logging::error!("Failed to serialize WebSocket message: {}", e);
                    continue;
                }
            };

            if sender
                .send(Message::Text(json_message.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let connection_manager_clone = connection_manager.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(message) = receiver.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Err(e) =
                        handle_client_message(&text, &connection_id, &connection_manager_clone)
                            .await
                    {
                        logging::warn!("Error handling client message: {}", e);
                    }
                }
                Ok(Message::Ping(_data)) => {
                    // Update last heartbeat time
                    connection_manager_clone
                        .update_last_heartbeat(&connection_id)
                        .await;
                }
                Ok(Message::Close(_)) => {
                    logging::log!("WebSocket connection closed by client: {}", connection_id);
                    break;
                }
                Err(e) => {
                    logging::error!("WebSocket error for connection {}: {}", connection_id, e);
                    break;
                }
                _ => {}
            }
        }

        connection_manager_clone
            .remove_connection(&connection_id)
            .await;
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = receive_task => {},
    }

    logging::log!("WebSocket connection {} terminated", connection_id);
}

async fn handle_client_message(
    text: &str,
    connection_id: &Uuid,
    connection_manager: &Arc<ConnectionManager>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let message: WebSocketMessage = serde_json::from_str(text)?;

    match message {
        WebSocketMessage::Pong => {
            // Client responded to ping - update heartbeat
            connection_manager
                .update_last_heartbeat(connection_id)
                .await;
        }
        _ => {
            // Handle other message types as needed
            logging::log!("Received message from {}: {:?}", connection_id, message);
        }
    }

    Ok(())
}

// Background task to send periodic heartbeats
pub async fn start_heartbeat_service(connection_manager: Arc<ConnectionManager>) {
    let mut heartbeat_interval = interval(Duration::from_secs(30)); // Send heartbeat every 30 seconds
    let mut cleanup_interval = interval(Duration::from_secs(60)); // Cleanup every minute

    loop {
        tokio::select! {
            _ = heartbeat_interval.tick() => {
                connection_manager.broadcast_heartbeat().await;
            }
            _ = cleanup_interval.tick() => {
                connection_manager.cleanup_stale_connections().await;
            }
        }
    }
}
