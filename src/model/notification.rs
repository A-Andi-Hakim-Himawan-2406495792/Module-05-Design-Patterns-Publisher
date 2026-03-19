use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub product_type: String,
    pub product_title: String,
    pub subscriber_name: String,
    pub subscriber_url: String,
    pub status: String,
    pub message: String,
}