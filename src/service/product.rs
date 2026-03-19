use rocket::http::Status;
use rocket::serde::json::Json;
use bambangshop::{Result, compose_error_response};
use crate::model::product::Product;
use crate::model::notification::Notification;
use crate::repository::product::ProductRepository;
use crate::controller::notification::NotificationController;

pub struct ProductService;

impl ProductService {
    pub fn create(mut product: Product) -> Result<Product> {
        product.product_type = product.product_type.to_uppercase();
        let product_result: Product = ProductRepository::add(product);
        NotificationController::notify(
            product_result.product_type.clone(),
            Notification {
                product_type: product_result.product_type.clone(),
                product_title: product_result.title.clone(),
                subscriber_name: String::from(""),
                subscriber_url: String::from(""),
                status: String::from("CREATED"),
                message: String::from("New product created."),
            }
        );
        return Ok(product_result);
    }

    pub fn list() -> Result<Vec<Product>> {
        return Ok(ProductRepository::list_all());
    }

    pub fn read(id: usize) -> Result<Product> {
        let product_opt: Option<Product> = ProductRepository::get_by_id(id);
        if product_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Product not found.")
            ));
        }
        return Ok(product_opt.unwrap());
    }

    pub fn delete(id: usize) -> Result<Json<Product>> {
        let product_opt: Option<Product> = ProductRepository::delete(id);
        if product_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Product not found.")
            ));
        }
        let product: Product = product_opt.unwrap();
        NotificationController::notify(
            product.product_type.clone(),
            Notification {
                product_type: product.product_type.clone(),
                product_title: product.title.clone(),
                subscriber_name: String::from(""),
                subscriber_url: String::from(""),
                status: String::from("DELETED"),
                message: String::from("Product deleted."),
            }
        );
        return Ok(Json::from(product));
    }

    pub fn publish(id: usize) -> Result<Product> {
        let product_opt: Option<Product> = ProductRepository::get_by_id(id);
        if product_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Product not found.")
            ));
        }
        let product: Product = product_opt.unwrap();
        NotificationController::notify(
            product.product_type.clone(),
            Notification {
                product_type: product.product_type.clone(),
                product_title: product.title.clone(),
                subscriber_name: String::from(""),
                subscriber_url: String::from(""),
                status: String::from("PUBLISHED"),
                message: String::from("Product published."),
            }
        );
        return Ok(product);
    }
}
