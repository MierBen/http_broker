use actix_web::{put, web, HttpResponse};

use std::collections::VecDeque;
use std::sync::Mutex;

use crate::config::QUEUE_MAX_SIZE;
use crate::models::Queue;

#[put("/{queue}")]
pub async fn put_handle(
    data: web::Data<Mutex<Queue>>,
    path: web::Path<String>,
    body: web::Bytes,
) -> HttpResponse {
    let mut queue = data.lock().unwrap();

    let name: String = path.into_inner();

    if let None = queue.get(name.as_str()) {
        let mut vec = VecDeque::new();
        vec.push_back(body.to_vec());
        queue.insert(name, vec);
        return HttpResponse::Ok().finish();
    }

    let value = queue.get_mut(name.as_str()).unwrap();
    if value.len() >= QUEUE_MAX_SIZE {
        return HttpResponse::TooManyRequests().finish();
    }
    value.push_back(body.to_vec());

    HttpResponse::Ok().finish()
}
