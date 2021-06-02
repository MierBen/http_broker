use actix_web::{get, rt::time::interval, web, HttpResponse};
use async_std::sync::Mutex;

use std::time::Duration;

use crate::models::{GetQueue, Queue};

/// Getting value from queue with timeout
///
/// # Arguments
///
/// * `data` - A queue shared object
/// * `name` - Name of queue
/// * `time` - Timeout
///
async fn get_from_queue_by_timeout(
    data: &web::Data<Mutex<Queue>>,
    name: &str,
    time: u64,
) -> Option<Vec<u8>> {
    let mut interval = interval(Duration::from_secs(1));
    for _i in 0..time {
        interval.tick().await;
        if let Some(value) = get_from_queue(data, name).await {
            return Some(value);
        }
    }
    None
}

/// Getting value from queue
///
/// # Arguments
///
/// * `data` - A queue shared object
/// * `name` - Name of queue
///
async fn get_from_queue(data: &web::Data<Mutex<Queue>>, name: &str) -> Option<Vec<u8>> {
    data.lock()
        .await
        .get_mut(name)
        .map(|v| v.pop_front())
        .flatten()
}

#[get("/{queue}")]
pub async fn get_handle(
    data: web::Data<Mutex<Queue>>,
    path: web::Path<String>,
    params: web::Query<GetQueue>,
) -> HttpResponse {
    let name: String = path.into_inner();
    let time = params.into_inner().timeout;

    if let Some(time) = time {
        if let Some(value) = get_from_queue_by_timeout(&data, &name, time).await {
            return HttpResponse::Ok().body(value);
        }
    } else {
        if let Some(value) = get_from_queue(&data, &name).await {
            return HttpResponse::Ok().body(value);
        }
    }

    HttpResponse::NotFound().finish()
}
