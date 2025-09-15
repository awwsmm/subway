mod db;
mod model;

use crate::db::tables::posts_by_id::PostsByIdTableRow;
use crate::db::Database;
use crate::model::post::Post;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use uuid::Uuid;

#[endpoint]
async fn hello(res: &mut Response) {
    let filename = "resources/hello.html";
    let contents = fs::read_to_string(filename).unwrap();
    res.render(Text::Html(contents))
}

#[handler]
async fn not_found(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        // TODO performance improvement possible if we `include_str!` to embed this HTML file
        //   directly in the binary, rather than reading it from the filesystem each time
        let filename = "resources/404.html";
        let contents = fs::read_to_string(filename).unwrap();
        res.render(Text::Html(contents));

        // Skip remaining error handlers
        ctrl.skip_rest();
    }
}

static DB: LazyLock<Mutex<Database>> = LazyLock::new(|| Mutex::new(Database::new()));

#[endpoint]
async fn create_post(res: &mut Response) {
    let mut lock = DB.lock().await;
    let table = &mut lock.posts_by_id;

    match table.insert(Post::new(String::from("default title")).into()) {
        Ok(key) => res.render(format!("added new Post to table with id: {}", key)),
        Err(_) => res.render("error creating Post"),
    }
}

#[endpoint]
async fn get_post(req: &mut Request, res: &mut Response) {
    let lock = DB.lock().await;
    let table = &lock.posts_by_id;

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

/// Returns all Posts up to the specified limit.
///
/// Posts are returned as a JSON-formatted list.
#[endpoint(
    parameters(
        ("limit" = u32, Path, description = "maximum number of Posts to return!")
    ),
    responses(
        (status_code = 200, description = "success response")
    )
)]
async fn list_posts(req: &mut Request, res: &mut Response) {
    let lock = DB.lock().await;
    let table = &lock.posts_by_id;

    let limit = req.param::<u32>("limit").unwrap_or(10);

    match table.list(limit) {
        Err(e) => res.render(format!("error listing Posts: {}", e)),
        Ok(posts) => res.render(Json(posts)),
    }
}

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("0.0.0.0:7878").bind().await;

    // TODO import regex package and enable this
    // PathFilter::register_wisp_regex(
    //     "guid",
    //     Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap(),
    // );

    let router = Router::new()
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("post/create").post(create_post))
        .push(Router::with_path("post/get/{id}").get(get_post))
        .push(Router::with_path("post/list/{limit}").get(list_posts))
        ;

    // TODO consider replacing env!("CARGO_PKG_VERSION") with clap's crate_version macro
    let doc = OpenApi::new("test api", env!("CARGO_PKG_VERSION")).merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    let catcher = Catcher::default().hoop(not_found);

    println!("Subway is running at http://localhost:7878");

    Server::new(acceptor).serve(Service::new(router).catcher(catcher)).await;
}