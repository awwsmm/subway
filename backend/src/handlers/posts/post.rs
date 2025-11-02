use crate::db::tables::posts_by_id::PostsByIdTableRow;
use crate::db::Database;
use crate::model::post;
use crate::model::post::Post;
use salvo::oapi::{endpoint, ToSchema};
use salvo::{Depot, Request, Response};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Fields required to create a Post.
#[derive(Deserialize, ToSchema)]
struct ProtoPost {
    title: String,
    body: String,
}

/// Create one or more Posts.
#[endpoint(
    request_body(
        content = Vec<ProtoPost>,
        description = "A JSON list of blog posts, each containing an author id, a title, and a body.",
        content_type = "application/json",
    ),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub(crate) async fn many(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();
    let user_id = depot.get::<Uuid>("token_user_id").expect("unknown user id");

    match req.parse_json::<Vec<ProtoPost>>().await {
        Ok(proto_posts) => {
            let mut db = state.lock().await;
            let table = &mut db.posts_by_id;

            match table.insert(
                proto_posts.into_iter().map(|proto_post| {
                    <PostsByIdTableRow as From<Post>>::from(
                        Post::new(
                            post::AuthorId::new(user_id.clone()),
                            post::Title::new(proto_post.title),
                            post::Body::new(proto_post.body),
                        )
                    )
                }).collect()
            ) {
                Ok(uuids) => res.render(format!("added new Post to table with ids: {:?}", uuids)),
                Err(e) => res.render(format!("error inserting new Posts into DB: {}", e)),
            }
        },
        Err(e) => res.render(format!("error parsing request body: {}", e)),
    }
}