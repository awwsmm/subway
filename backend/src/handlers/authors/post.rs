use crate::db::tables::authors_by_id::AuthorsByIdTableRow;
use crate::db::Database;
use crate::model::author::Author;
use salvo::oapi::{endpoint, ToSchema};
use salvo::{Depot, Request, Response};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

/// Fields required to create a Author.
#[derive(Deserialize, ToSchema)]
struct ProtoAuthor {
    name: String,
}

/// Create one or more Authors.
#[endpoint(
    request_body(
        content = Vec<ProtoAuthor>,
        description = "A JSON list of objects, each containing a name",
        content_type = "text/json",
    ),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub(crate) async fn many(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Database>>>().unwrap();

    match req.parse_json::<Vec<ProtoAuthor>>().await {
        Ok(proto_authors) => {
            let mut db = state.lock().unwrap();
            let table = &mut db.authors_by_id;

            match table.insert(
                proto_authors.into_iter().map(|proto_author| {
                    <AuthorsByIdTableRow as From<Author>>::from(Author::new(proto_author.name))
                }).collect()
            ) {
                Ok(uuids) => res.render(format!("added new Author to table with ids: {:?}", uuids)),
                Err(e) => res.render(format!("error inserting new Authors into DB: {}", e)),
            }
        },
        Err(e) => res.render(format!("error parsing request body: {}", e)),
    }
}