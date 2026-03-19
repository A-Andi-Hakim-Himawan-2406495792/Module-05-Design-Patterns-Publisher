use crate::model::subscriber::Subscriber;
use crate::model::notification::Notification;
use crate::repository::subscriber::SubscriberRepository;
use std::thread;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: String, subscriber: Subscriber) -> Subscriber {
        SubscriberRepository::add(&product_type, subscriber)
    }

    pub fn unsubscribe(product_type: String, subscriber_url: String) -> Option<Subscriber> {
        SubscriberRepository::delete(&product_type, &subscriber_url)
    }

    pub fn notify(product_type: String, notification: Notification) {
        let subscribers = SubscriberRepository::list_all(&product_type);

        for subscriber in subscribers {
            let notif_clone = notification.clone();
            thread::spawn(move || {
                subscriber.update(&notif_clone);
            });
        }
    }
}