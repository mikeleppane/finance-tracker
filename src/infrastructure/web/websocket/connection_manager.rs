// src/infrastructure/websocket/connection_manager.rs
use leptos::logging;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{sync::RwLock, time::Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    Heartbeat {
        timestamp: u64,
        server_time: String,
    },
    ConnectionStatus {
        connected_users: usize,
        uptime_seconds: u64,
    },
    Error {
        message: String,
    },
    Pong,
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: Uuid,
    pub connected_at: Instant,
    pub last_heartbeat: Instant,
    pub user_id: Option<String>, // For authenticated connections
}

pub type SharedConnectionManager = Arc<ConnectionManager>;

pub struct ConnectionManager {
    connections: RwLock<HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<WebSocketMessage>>>,
    connection_info: RwLock<HashMap<Uuid, ConnectionInfo>>,
    connected_count: AtomicUsize,
    server_start_time: Instant,
}

impl ConnectionManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
            connection_info: RwLock::new(HashMap::new()),
            connected_count: AtomicUsize::new(0),
            server_start_time: Instant::now(),
        }
    }

    pub async fn add_connection(
        &self,
        connection_id: Uuid,
        sender: tokio::sync::mpsc::UnboundedSender<WebSocketMessage>,
        user_id: Option<String>,
    ) {
        let connection_info = ConnectionInfo {
            id: connection_id,
            connected_at: Instant::now(),
            last_heartbeat: Instant::now(),
            user_id,
        };

        {
            let mut connections = self.connections.write().await;
            connections.insert(connection_id, sender);
        }

        {
            let mut info_map = self.connection_info.write().await;
            info_map.insert(connection_id, connection_info);
        }

        let count = self.connected_count.fetch_add(1, Ordering::Relaxed) + 1;
        logging::log!(
            "New WebSocket connection: {} (Total: {})",
            connection_id,
            count
        );

        // Broadcast updated connection count to all clients
        self.broadcast_connection_status().await;
    }

    pub async fn remove_connection(&self, connection_id: &Uuid) {
        self.remove_connection_internal(connection_id, true).await;
    }

    async fn remove_connection_internal(&self, connection_id: &Uuid, broadcast_status: bool) {
        {
            let mut connections = self.connections.write().await;
            connections.remove(connection_id);
        }

        {
            let mut info_map = self.connection_info.write().await;
            info_map.remove(connection_id);
        }

        let count = self.connected_count.fetch_sub(1, Ordering::Relaxed) - 1;
        logging::log!(
            "WebSocket connection closed: {} (Total: {})",
            connection_id,
            count
        );

        // Broadcast updated connection count to all clients
        if broadcast_status {
            self.broadcast_connection_status().await;
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub async fn broadcast_heartbeat(&self) {
        let message = WebSocketMessage::Heartbeat {
            timestamp: chrono::Utc::now().timestamp_millis().max(0) as u64,
            server_time: chrono::Utc::now().to_rfc3339(),
        };

        self.broadcast_message(message).await;
    }

    pub async fn broadcast_connection_status(&self) {
        let connected_users = self.connected_count.load(Ordering::Relaxed);
        let uptime_seconds = self.server_start_time.elapsed().as_secs();

        let message = WebSocketMessage::ConnectionStatus {
            connected_users,
            uptime_seconds,
        };

        self.broadcast_message(message).await;
    }

    async fn broadcast_message(&self, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        let mut failed_connections = Vec::new();

        for (connection_id, sender) in connections.iter() {
            if sender.send(message.clone()).is_err() {
                failed_connections.push(*connection_id);
            }
        }

        // Remove failed connections
        drop(connections);
        for connection_id in failed_connections {
            Box::pin(self.remove_connection_internal(&connection_id, false)).await;
        }
    }

    pub fn get_connection_count(&self) -> usize {
        self.connected_count.load(Ordering::Relaxed)
    }

    pub async fn update_last_heartbeat(&self, connection_id: &Uuid) {
        let mut info_map = self.connection_info.write().await;
        if let Some(info) = info_map.get_mut(connection_id) {
            info.last_heartbeat = Instant::now();
        }
    }

    // Clean up stale connections
    pub async fn cleanup_stale_connections(&self) {
        let stale_threshold = Duration::from_secs(60); // 1 minute
        let now = Instant::now();
        let mut stale_connections = Vec::new();

        {
            let info_map = self.connection_info.read().await;
            for (connection_id, info) in info_map.iter() {
                if now.duration_since(info.last_heartbeat) > stale_threshold {
                    stale_connections.push(*connection_id);
                }
            }
        }

        for connection_id in stale_connections {
            logging::warn!("Removing stale connection: {}", connection_id);
            self.remove_connection(&connection_id).await;
        }
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
