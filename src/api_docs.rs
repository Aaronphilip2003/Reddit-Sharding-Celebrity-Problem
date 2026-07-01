use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::hello_handler::hello,
        crate::handlers::entry_handler::get_entries
    ),
    components(schemas(crate::models::responses::hello_response::HelloResponse))
)]
pub struct ApiDoc;
