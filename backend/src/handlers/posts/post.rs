use crate::model::post::Post;
use crate::DB;
use salvo::oapi::endpoint;
use salvo::Response;

/// Endpoint to POST many Posts (i.e. to create one or more Posts).
#[endpoint]
pub(crate) async fn many(res: &mut Response) {
    let mut lock = DB.lock().await;
    let table = &mut lock.posts_by_id;

    match table.insert(Post::new(String::from("default title")).into()) {
        Ok(key) => res.render(format!("added new Post to table with id: {}", key)),
        Err(_) => res.render("error creating Post"),
    }
}