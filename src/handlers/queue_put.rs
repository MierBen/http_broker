use actix_web::{put, web, HttpResponse};
use async_std::sync::Mutex;

use std::collections::VecDeque;

use crate::config::QUEUE_MAX_SIZE;
use crate::models::Queue;

/// Put value to queue
///
/// # Arguments
///
/// * `data` - A queue shared object
/// * `path` - Path with queue name
/// * `data` - Data to put
#[put("/{queue}")]
pub async fn put_handle(
    data: web::Data<Mutex<Queue>>,
    path: web::Path<String>,
    body: web::Bytes,
) -> HttpResponse {
    let name: String = path.into_inner();

    if body.is_empty() {
        return HttpResponse::BadRequest().body("Data cann`t be empty");
    }

    let mut queue = data.lock().await;
    let value = queue.entry(name).or_insert_with(VecDeque::new);
    if value.len() >= QUEUE_MAX_SIZE {
        return HttpResponse::TooManyRequests().finish();
    }
    value.push_back(body.to_vec());
    HttpResponse::Ok().finish()
}
