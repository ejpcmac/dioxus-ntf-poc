//! A web service for notifications.

#![expect(
    clippy::missing_panics_doc,
    clippy::expect_used,
    reason = "that’s a PoC"
)]

use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
};
use eyre::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// The state of the web service.
#[derive(Debug, Default)]
pub struct AppState {
    /// The notifications.
    pub notifications: IndexMap<usize, Notification>,
}

/// A notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// The notification ID.
    pub id: usize,
    /// The message to show.
    pub message: String,
    /// Has the notification been acknowledged?
    pub ack: bool,
}

/// The request payload for `POST /notifications`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotificationPayload {
    /// The message to show.
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/status", get(status))
        .route("/notifications", get(list_notifications))
        .route("/notifications", post(create_notification))
        .route("/notifications/{id}", get(get_notification))
        .route("/notifications/{id}", put(ack_notification))
        .route("/notifications/{id}", delete(delete_notification))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Returns the status.
async fn status() -> &'static str {
    "ok"
}

/// Lists the notifications.
async fn list_notifications(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<Vec<Notification>> {
    Json(
        state
            .lock()
            .expect("cannot acquire lock on the state")
            .notifications
            .values()
            .cloned()
            .collect(),
    )
}

/// Gets a notification by its ID.
async fn create_notification(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateNotificationPayload>,
) -> Json<Notification> {
    let mut state = state.lock().expect("cannot acquire lock on the state");

    let id = state.notifications.keys().last().unwrap_or(&0) + 1;
    let notification = Notification {
        id,
        message: payload.message,
        ack: false,
    };
    state.notifications.insert(id, notification.clone());

    Json(notification)
}

/// Gets a notification by its ID.
async fn get_notification(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<usize>,
) -> Json<Notification> {
    Json(
        state
            .lock()
            .expect("cannot acquire lock on the state")
            .notifications
            .get(&id)
            .unwrap()
            .clone(),
    )
}

/// Acknowledges a notification.
async fn ack_notification(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<usize>,
) -> Json<Notification> {
    let mut state = state.lock().unwrap();
    let notification = state.notifications.get_mut(&id).unwrap();
    notification.ack = true;
    Json(notification.clone())
}

/// Delete a notification.
async fn delete_notification(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<usize>,
) -> Json<Notification> {
    Json(
        state
            .lock()
            .expect("cannot acquire lock on the state")
            .notifications
            .shift_remove(&id)
            .unwrap(),
    )
}
