use crate::db::tables::posts_by_id::PostsByIdTableRow;
use crate::db::Database;
use salvo::oapi::endpoint;
use salvo::prelude::Json;
use salvo::{Depot, Request, Response};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Endpoint to GET one single Post by id.
#[endpoint]
pub(crate) async fn one(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();
    let db = state.lock().await;
    let table = &db.posts_by_id;

    let id: String = req.param::<String>("id").expect("request did not contain a 'id' param");

    match Uuid::from_str(&id) {
        Err(_) => res.render(format!("cannot parse {} as UUID\n", id)),
        Ok(key) => {
            match table.get(&key) {
                Err(e) => res.render(format!("error getting Post by id: {}", e)),
                Ok(post) => res.render(Json(Into::<PostsByIdTableRow>::into(post))),
            }
        }
    }
}

// See https://docs.rs/salvo-oapi/0.84.0/salvo_oapi/endpoint/index.html#tuples
//   for info on OpenAPI endpoint tuples for parameters

/// Returns all Posts up to the specified limit.
///
/// Posts are returned as a JSON-formatted list.
#[endpoint(
    parameters(
        ("limit" = u32, Query, description = "maximum number of Posts to return")
    ),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub(crate) async fn many(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();
    let db = state.lock().await;
    let table = &db.posts_by_id;

    let limit = req.query::<usize>("limit").unwrap_or(10);

    match table.list(limit) {
        Err(e) => res.render(format!("error listing Posts: {}", e)),
        Ok(posts) => res.render(Json(posts)),
    }
}