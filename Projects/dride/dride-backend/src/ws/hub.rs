use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use super::messages::WsMessage;

/// Channel sender for a connected user
type UserTx = mpsc::UnboundedSender<String>;

/// Tracks connected user's location for geo-broadcast
#[derive(Debug, Clone)]
pub struct ConnectedUser {
    pub tx: UserTx,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

/// Connection hub: registry of connected users by user_id
#[derive(Debug, Clone)]
pub struct Hub {
    connections: Arc<RwLock<HashMap<Uuid, ConnectedUser>>>,
}

impl Hub {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a user connection
    pub async fn register(&self, user_id: Uuid, tx: UserTx) {
        let mut conns = self.connections.write().await;
        conns.insert(
            user_id,
            ConnectedUser {
                tx,
                lat: None,
                lng: None,
            },
        );
        tracing::info!("WS hub: user {} connected (total: {})", user_id, conns.len());
    }

    /// Unregister a user connection
    pub async fn unregister(&self, user_id: &Uuid) {
        let mut conns = self.connections.write().await;
        conns.remove(user_id);
        tracing::info!("WS hub: user {} disconnected (total: {})", user_id, conns.len());
    }

    /// Update a user's location
    pub async fn update_location(&self, user_id: &Uuid, lat: f64, lng: f64) {
        let mut conns = self.connections.write().await;
        if let Some(user) = conns.get_mut(user_id) {
            user.lat = Some(lat);
            user.lng = Some(lng);
        }
    }

    /// Send a message to a specific user
    pub async fn send_to_user(&self, user_id: &Uuid, msg: &WsMessage) {
        let conns = self.connections.read().await;
        if let Some(user) = conns.get(user_id) {
            let _ = user.tx.send(msg.to_text());
        }
    }

    /// Broadcast to multiple users
    pub async fn send_to_users(&self, user_ids: &[Uuid], msg: &WsMessage) {
        let conns = self.connections.read().await;
        let text = msg.to_text();
        for uid in user_ids {
            if let Some(user) = conns.get(uid) {
                let _ = user.tx.send(text.clone());
            }
        }
    }

    /// Broadcast to all connected users within radius_km of a point
    pub async fn broadcast_nearby(
        &self,
        lat: f64,
        lng: f64,
        radius_km: f64,
        msg: &WsMessage,
        exclude: Option<&Uuid>,
    ) {
        let conns = self.connections.read().await;
        let text = msg.to_text();
        for (uid, user) in conns.iter() {
            if let Some(excl) = exclude {
                if uid == excl {
                    continue;
                }
            }
            if let (Some(ulat), Some(ulng)) = (user.lat, user.lng) {
                let dist = crate::services::pricing::haversine_km(lat, lng, ulat, ulng);
                if dist <= radius_km {
                    let _ = user.tx.send(text.clone());
                }
            }
        }
    }

    /// Check if a user is connected
    pub async fn is_connected(&self, user_id: &Uuid) -> bool {
        let conns = self.connections.read().await;
        conns.contains_key(user_id)
    }
}
