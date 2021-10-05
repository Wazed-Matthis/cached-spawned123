use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use std::sync::Arc;

lazy_static!{
    pub static ref RUNTIME : Arc<Runtime> = Arc::new(Runtime::new().unwrap());
}