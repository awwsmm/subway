mod db;
mod model;

use crate::db::table::Table;
use crate::db::tables::posts_by_id::PostsByIdTable;
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

#[cfg(feature = "postgres")]
use diesel::r2d2::{ConnectionManager, Pool};

#[cfg(feature = "postgres")]
use diesel::PgConnection;

#[cfg(feature = "postgres")]
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

#[cfg(feature = "postgres")]
use std::sync::Arc;

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

// TODO figure out how to document response codes (2xx, 5xx) in OpenAPI

#[cfg(not(feature = "postgres"))]
static DB: LazyLock<Mutex<Database>> = LazyLock::new(|| Mutex::new(Database { posts_by_id: Box::new(PostsByIdTable::new()) }));

#[cfg(feature = "postgres")]
static DB: LazyLock<Mutex<Database>> = LazyLock::new(|| {

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let n_connections = 10;

    let pool = Pool::builder()
        .max_size(n_connections)
        .min_idle(Some(n_connections))
        .test_on_check_out(false)
        .idle_timeout(None)
        .max_lifetime(None)
        .build(manager);

    match pool {
        Ok(pool) => {
            const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

            match pool.get() {
                Ok(mut connection) => connection.run_pending_migrations(MIGRATIONS).unwrap(),
                Err(_) => panic!("Unable to run database migrations!"),
            };

            let arc_pool = Arc::new(pool);
            Mutex::new(Database { posts_by_id: Box::new(PostsByIdTable { connection_pool: arc_pool }) })
        },
        Err(_) => panic!("Database Pool Creation failed"),
    }
});

#[endpoint]
async fn create_post(res: &mut Response) {
    let mut lock = DB.lock().await;
    let table = &mut lock.posts_by_id;

    match table.insert(Post::new(String::from("default title"))) {
        Ok(key) => res.render(format!("added new Post to table with id: {}\n\ntable: {:?}", key, table)),
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
                Ok(post) => res.render(Json(post)),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("0.0.0.0:7878").bind().await;

    // POSTS_BY_ID_TABLE
    //     .set(Mutex::new(PostsByIdTable::new()))
    //     .expect("Could not create PostsByIdTable");

    // TODO import regex package and enable this
    // PathFilter::register_wisp_regex(
    //     "guid",
    //     Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap(),
    // );

    let router = Router::new()
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("post/create").post(create_post))
        .push(Router::with_path("post/get/{id}").get(get_post))
        // .push(Router::with_path("user/add/{id}/{name}").get(add_user))
        // .push(Router::with_path("user/get/{id}").get(get_user))
        ;

    // TODO consider replacing env!("CARGO_PKG_VERSION") with clap's crate_version macro
    let doc = OpenApi::new("test api", env!("CARGO_PKG_VERSION")).merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    let catcher = Catcher::default().hoop(not_found);

    println!("Subway is running ...");

    Server::new(acceptor).serve(Service::new(router).catcher(catcher)).await;
}