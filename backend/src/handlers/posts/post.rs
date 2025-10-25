use crate::db::tables::posts_by_id::PostsByIdTableRow;
use crate::db::Database;
use crate::model::post::Post;
use salvo::oapi::{endpoint, ToSchema};
use salvo::{Depot, Request, Response};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

/// Fields required to create a Post.
#[derive(Deserialize, ToSchema)]
struct ProtoPost {
    title: String,
}

/// Create one or more Posts.
#[endpoint(
    request_body(
        content = Vec<ProtoPost>,
        description = "A JSON list of objects, each containing a title",
        content_type = "text/json",
    ),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub(crate) async fn many(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();

    match req.parse_json::<Vec<ProtoPost>>().await {
        Ok(proto_posts) => {
            let mut db = state.lock().unwrap();
            let table = &mut db.posts_by_id;

            match table.insert(
                proto_posts.into_iter().map(|proto_post| {
                    <PostsByIdTableRow as From<Post>>::from(Post::new(proto_post.title))
                }).collect()
            ) {
                Ok(uuids) => res.render(format!("added new Post to table with ids: {:?}", uuids)),
                Err(e) => res.render(format!("error inserting new Posts into DB: {}", e)),
            }
        },
        Err(e) => res.render(format!("error parsing request body: {}", e)),
    }
}