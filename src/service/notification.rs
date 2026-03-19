use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
        pub fn subscribe(product_type: String, subscriber: Subscriber) -> Subscriber {
            SubscriberRepository::add(&product_type, subscriber)
        }
        pub fn unsubscribe(product_type: String, subscriber_url: String) -> Option<Subscriber> {
                SubscriberRepository::delete(&product_type, &subscriber_url)
            }


}