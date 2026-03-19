use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::model::subscriber::Subscriber;
use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::service::notification::NotificationService;
use crate::service::product::ProductService;

pub struct NotificationController;

impl NotificationController {
    pub fn notify(product_type: String, notification: Notification) {
        NotificationService::notify(product_type, notification);
    }
}

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

#[post("/publish/<id>")]
pub fn publish(id: usize) -> Result<status::Custom<Json<Product>>, bambangshop::Error> {
    let result = ProductService::publish(id)?;
    Ok(status::Custom(Status::Ok, Json(result)))
}