use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::hello_handler::hello,
        crate::handlers::user_handler::create_user,
        crate::handlers::post_handler::create_post,
        crate::handlers::post_handler::list_posts,
        crate::handlers::comment_handler::create_comment,
        crate::handlers::comment_handler::list_comments_for_post,
        crate::handlers::vote_handler::cast_vote,
        crate::handlers::vote_handler::score_for_post
    ),
    components(schemas(
        crate::models::responses::hello_response::HelloResponse,
        crate::models::requests::user_requests::CreateUserRequest,
        crate::models::responses::user_response::UserResponse,
        crate::models::requests::post_requests::CreatePostRequest,
        crate::models::responses::post_response::PostResponse,
        crate::models::requests::comment_requests::CreateCommentRequest,
        crate::models::responses::comment_response::CommentResponse,
        crate::models::requests::vote_requests::CastVoteRequest,
        crate::models::responses::vote_response::VoteResponse,
        crate::models::responses::vote_response::ScoreResponse
    ))
)]
pub struct ApiDoc;
