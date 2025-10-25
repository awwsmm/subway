use crate::db::tables::authors_by_id::AuthorsByIdTableRow;
use crate::db::Database;
use salvo::oapi::endpoint;
use salvo::prelude::Json;
use salvo::{Depot, Request, Response};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Endpoint to GET one single Author by id.
#[endpoint]
pub(crate) async fn one(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();
    let db = state.lock().unwrap();
    let table = &db.authors_by_id;

    let id: String = req.param::<String>("id").expect("request did not contain an 'id' param");

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
pub(crate) async fn many(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();
    let db = state.lock().unwrap();
    let table = &db.authors_by_id;

    let limit = req.query::<u32>("limit").unwrap_or(10);

    match table.list(limit) {
        Err(e) => res.render(format!("error listing Authors: {}", e)),
        Ok(authors) => res.render(Json(authors)),
    }
}