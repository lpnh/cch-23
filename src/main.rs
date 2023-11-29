use cch23_lpnh::app;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = app();

    Ok(app.into())
}
