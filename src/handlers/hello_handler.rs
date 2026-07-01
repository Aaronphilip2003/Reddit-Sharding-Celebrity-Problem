use axum::Json;

use crate::models::responses::hello_response::HelloResponse;

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (
            status = 200,
            description = "Returns hello message",
            body = HelloResponse
        )
    )
)]
pub async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello World".to_string(),
    })
}
