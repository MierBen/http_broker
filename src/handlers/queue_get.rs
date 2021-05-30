use actix_web::{get, rt::time::interval, web, HttpResponse};

use std::sync::Mutex;
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
    name: &String,
    time: u64,
) -> Result<Vec<u8>, &'static str> {
    let mut interval = interval(Duration::from_secs(1));
    for _i in 0..time {
        interval.tick().await;
        if let Ok(value) = get_from_queue(data, name).await {
            return Ok(value);
        }
    }
    Err("Can`t get result")
}

/// Getting value from queue
///
/// # Arguments
///
/// * `data` - A queue shared object
/// * `name` - Name of queue
///
async fn get_from_queue(
    data: &web::Data<Mutex<Queue>>,
    name: &String,
) -> Result<Vec<u8>, &'static str> {
    let mut queue = data.lock().unwrap();
    if let None = queue.get(name.as_str()) {
        return Err("Value not found!");
    }
    let value = queue.get_mut(name).unwrap();
    if value.is_empty() {
        queue.remove(name.as_str());
        return Err("Value not found!");
    }
    Ok(value.pop_front().unwrap())
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
        if let Ok(value) = get_from_queue_by_timeout(&data, &name, time).await {
            return HttpResponse::Ok().body(value);
        }
    } else {
        if let Ok(value) = get_from_queue(&data, &name).await {
            return HttpResponse::Ok().body(value);
        }
    }

    HttpResponse::NotFound().finish()
}
