use serde::Deserialize;
use std::collections::{HashMap, VecDeque};

#[derive(Deserialize, Debug)]
pub struct GetQueue {
    pub timeout: Option<u64>,
}

pub type Queue = HashMap<String, VecDeque<Vec<u8>>>;
