use crustchan::dynamodb::{get_post_by_id, list_boards};
use crustchan::response::{GenericResponse, WebResult};
use crustchan::models::Board;
use tracing::info;

#[utoipa::path(get, path = "/{board_id}/{post_id}",
    params(
        ("board_id" = u64, Path, description = "The board to get a post in"),
        ("post_id" = u64, Path, description = "The post to fetch"),
    )
)]
pub async fn get_post_by_id_handler(_board_id: String, post_id: String) -> WebResult {
    info!("get_post_by_id_handler:");
    let post = get_post_by_id(post_id).await.unwrap();


    let response = GenericResponse::new(warp::http::StatusCode::OK, post);
    Ok(response)
}

#[utoipa::path(
    get, 
    path = "/" ,
    responses(
        (status = 200, description = "Boards found successfully", body = Vec<Board>)
    )
)]

pub async fn get_boards_handler() -> WebResult {
    info!("get_boards_handler:");
    let boards = list_boards().await.unwrap();

    let response = GenericResponse::new(warp::http::StatusCode::OK, boards);
    Ok(response)
}
