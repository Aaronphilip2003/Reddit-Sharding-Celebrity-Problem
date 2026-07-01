use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::hello_handler::hello,
        crate::handlers::user_handler::create_user,
        crate::handlers::post_handler::create_post,
        crate::handlers::post_handler::list_posts
    ),
    components(schemas(
        crate::models::responses::hello_response::HelloResponse,
        crate::models::requests::user_requests::CreateUserRequest,
        crate::models::responses::user_response::UserResponse,
        crate::models::requests::post_requests::CreatePostRequest,
        crate::models::responses::post_response::PostResponse
    ))
)]
pub struct ApiDoc;
