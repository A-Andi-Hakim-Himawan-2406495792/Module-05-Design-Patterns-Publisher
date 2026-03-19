use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;