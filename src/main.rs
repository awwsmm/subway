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
static DB: LazyLock<Mutex<Database>> = LazyLock::new(|| Mutex::new(Database { posts_by_id: PostsByIdTable::new() }));

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
            Mutex::new(Database { posts_by_id: PostsByIdTable { connection_pool: arc_pool } })
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

// // example: curl -X POST localhost:7878/post/create -H 'Content-Type: application/json' -d '{"title":"Hello, World!"}'
// #[endpoint]
// async fn create_post(req: &mut Request, res: &mut Response) {
//     match File::options().append(true).create(true).open("database.txt") {
//         Ok(mut file) => {
//
//             let id = Uuid::new_v4();
//             let post_constructor: CreatePostRequest = req.parse_json().await.unwrap();
//             let title = post_constructor.title.as_str();
//
//             let post = Post { id: id.to_string(), title: title.to_string() };
//
//             // TODO replace this writing to / reading from a file with a Postgres DB
//             //   place this behind an interface, so we can still use a text file DB if we want to
//             match file.write(format!("{},{}\r\n", id.to_string(), title).as_bytes()) {
//                 Ok(_) => {
//                     res.render(Json(post));
//                 },
//                 Err(e) => res.render(salvo::Error::other(format!("Error writing to database.txt: {}", e))),
//             }
//         },
//         Err(e) => res.render(salvo::Error::other(format!("Error opening database.txt: {}", e))),
//     }
// }

// // example: curl localhost:7878/post/510d6709-4082-4c07-b79c-96d112cf1281
// #[endpoint(parameters(("id", description = "Pet id")))]
// async fn get_post(req: &mut Request, res: &mut Response) {
//     match req.param::<String>("id") {
//         Some(id) => {
//             if !Path::new("database.txt").exists() {
//                 res.render(salvo::Error::other("database.txt does not exist"));
//             } else {
//                 match File::options().read(true).write(false).open("database.txt") {
//                     Ok(mut file) => {
//                         let mut str = String::new();
//                         let mut maybe_post: Option<Post> = None;
//
//                         match file.read_to_string(&mut str) {
//                             Ok(_) => {
//                                 let lines = str.split("\n").collect::<Vec<&str>>();
//                                 for line in lines {
//                                     let fields = line.split(",").collect::<Vec<&str>>();
//
//                                     if id == fields[0] {
//                                         match Uuid::parse_str(fields[0]) {
//                                             Ok(uuid) => {
//                                                 let id = uuid;
//                                                 let title = fields[1].to_string();
//                                                 maybe_post = Some(Post { id: id.to_string(), title: title.clone() });
//                                                 break;
//                                             },
//                                             Err(e) => res.render(salvo::Error::other(format!("Error parsing uuid: {}", e))),
//                                         }
//                                     }
//                                 }
//
//                                 match maybe_post {
//                                     Some(post) => res.render(Json(post)),
//                                     None => res.render(salvo::Error::other(format!("Unknown uuid: {}", id))),
//                                 }
//                             },
//                             Err(e) => res.render(salvo::Error::other(format!("Error reading database.txt: {}", e))),
//                         }
//                     }
//                     Err(e) => res.render(salvo::Error::other(format!("Error opening database.txt: {}", e))),
//                 }
//             }
//         },
//         None => res.render(salvo::Error::other("Missing uuid in request"))
//     }
// }

// #[handler]
// async fn get_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
//     let id: i32 = req.param::<i32>("id").expect("request did not contain an 'id: i32' field");
//
//     let db = depot.obtain::<db::Database>().expect("Couldn't obtain database");
//
//     match db.get_user(id).await {
//         Ok(user) => res.render(Json(user)),
//         Err(e) => res.render(salvo::Error::other(format!("Error getting user: {}", e))),
//     }
// }
//
// #[handler]
// async fn add_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
//     let id: i32 = req.param::<i32>("id").expect("request did not contain an 'id: i32' field");
//     let name: String = req.param::<String>("name").expect("request did not contain a 'name: String' field");
//
//     let db = depot.obtain_mut::<db::Database>().expect("Couldn't obtain database");
//
//     match db.add_user(id, name).await {
//         Ok(_) => res.render(Text::Plain("user added successfully".to_string())),
//         Err(e) => res.render(salvo::Error::other(format!("Error adding user: {}", e))),
//     }
// }


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