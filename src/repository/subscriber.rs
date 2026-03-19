use dashmap::DashMap;
use lazy_static::lazy_static;
use rocket::log::private::warn;
use crate::model::subscriber::Subscriber;

lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
        pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
            // Kalau product_type belum ada, buat DashMap baru untuk dia
            if !SUBSCRIBERS.contains_key(product_type) {
                SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
            }

            // Ambil map untuk product_type ini, lalu insert subscriber
            let mut product_subscribers = SUBSCRIBERS
                .get_mut(product_type)
                .unwrap();

            product_subscribers.insert(subscriber.url.clone(), subscriber.clone());
            subscriber
        }
        pub fn list_all(product_type: &str) -> Vec<Subscriber> {
                // Kalau product_type tidak ada, return vec kosong
                let subscribers = SUBSCRIBERS.get(product_type);
                match subscribers {
                    Some(map) => map.iter().map(|entry| entry.value().clone()).collect(),
                    None => Vec::new(),
                }
            }
        pub fn delete(product_type: &str, subscriber_url: &str) -> Option<Subscriber> {
                let product_subscribers = SUBSCRIBERS.get(product_type);
                match product_subscribers {
                    Some(map) => {
                        // remove() returns Option<(key, value)>, kita ambil value-nya aja
                        map.remove(subscriber_url).map(|(_, subscriber)| subscriber)
                    },
                    None => None,
                }
            }
}