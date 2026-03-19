use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: String, subscriber: Json<Subscriber>)
    -> status::Custom<Json<Subscriber>>
{
    let result = NotificationService::subscribe(product_type, subscriber.into_inner());
    status::Custom(Status::Created, Json(result))
}