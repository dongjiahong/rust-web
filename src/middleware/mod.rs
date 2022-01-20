pub mod auth;
pub mod jwt;

use poem::{http::Method, middleware::Cors};

pub fn new_cors() -> Cors {
    Cors::new().allow_methods(vec![Method::POST, Method::DELETE, Method::GET, Method::PUT])
}
