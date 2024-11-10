#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views;

#[debug_handler]
pub async fn homepage(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::home::homepage(&v)
}

async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
    format::text("fuck off")
}

pub fn routes() -> Routes {
    // Routes::new().prefix("home/").add("/hello", get(hello))
    Routes::new().prefix("/").add("/", get(homepage))
}
