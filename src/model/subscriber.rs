use rocket::serde::{Deserialize, Serialize};
use crate::model::notification::Notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl Subscriber {
    pub fn update(&self, notification: &Notification) {
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post(&self.url)
            .json(notification)
            .send();
    }
}