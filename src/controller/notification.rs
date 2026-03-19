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

#[delete("/unsubscribe/<product_type>?<subscriber_url>")]
pub fn unsubscribe(product_type: String, subscriber_url: String)
    -> status::Custom<Json<Option<Subscriber>>>
{
    let result = NotificationService::unsubscribe(product_type, subscriber_url);
    status::Custom(Status::Ok, Json(result))
}