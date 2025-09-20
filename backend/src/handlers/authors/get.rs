use crate::db::tables::authors_by_id::AuthorsByIdTableRow;
use crate::DB;
use salvo::oapi::endpoint;
use salvo::prelude::Json;
use salvo::{Request, Response};
use std::str::FromStr;
use uuid::Uuid;

/// Endpoint to GET one single Author by id.
#[endpoint]
pub(crate) async fn one(req: &mut Request, res: &mut Response) {
    let lock = DB.lock().await;
    let table = &lock.authors_by_id;

    let id: String = req.param::<String>("id").expect("request did not contain a 'id' param");

    match Uuid::from_str(&id) {
        Err(_) => res.render(format!("cannot parse {} as UUID\n", id)),
        Ok(key) => {
            match table.get(&key) {
                Err(e) => res.render(format!("error getting Author by id: {}", e)),
                Ok(author) => res.render(Json(Into::<AuthorsByIdTableRow>::into(author))),
            }
        }
    }
}

// See https://docs.rs/salvo-oapi/0.84.0/salvo_oapi/endpoint/index.html#tuples
//   for info on OpenAPI endpoint tuples for parameters

/// Returns all Authors up to the specified limit.
///
/// Authors are returned as a JSON-formatted list.
#[endpoint(
    parameters(
        ("limit" = u32, Query, description = "maximum number of Authors to return!")
    ),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub(crate) async fn many(req: &mut Request, res: &mut Response) {
    let lock = DB.lock().await;
    let table = &lock.authors_by_id;

    let limit = req.query::<u32>("limit").unwrap_or(10);

    match table.list(limit) {
        Err(e) => res.render(format!("error listing Authors: {}", e)),
        Ok(authors) => res.render(Json(authors)),
    }
}