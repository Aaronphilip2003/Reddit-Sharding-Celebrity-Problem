use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[allow(dead_code)] // Placeholder for upcoming POST/PUT examples.
pub struct HelloRequest {
    pub name: String,
}
