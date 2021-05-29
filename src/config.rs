use dotenv_codegen::dotenv;
use konst::{primitive::parse_usize, result::unwrap_ctx};

pub const SERVER_ADDR: &'static str = dotenv!("SERVER_ADDR");

pub const QUEUE_MAX_SIZE: usize = unwrap_ctx!(parse_usize(dotenv!("QUEUE_MAX_SIZE")));
