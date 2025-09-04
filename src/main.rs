mod db;

use crate::db::Database;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
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
        let filename = "resources/404.html";
        let contents = fs::read_to_string(filename).unwrap();
        res.render(Text::Html(contents));

        // Skip remaining error handlers
        ctrl.skip_rest();
    }
}

// TODO move model classes into src/model and add README.md there to explain ubiquitous language

#[derive(Deserialize, ToSchema, Extractible)]
struct PostConstructor {
    title: String,
}

// FIXME make id field a Uuid when Postgres is introduced
#[derive(Serialize, Deserialize)]
struct Post {
    id: String,
    title: String,
}

// TODO figure out how to document response codes (2xx, 5xx) in OpenAPI

#[endpoint]
async fn create_post(req: &mut Request, res: &mut Response) {
    match File::options().append(true).create(true).open("database.txt") {
        Ok(mut file) => {

            let id = Uuid::new_v4();
            let post_constructor: PostConstructor = req.extract().await.unwrap();
            let title = post_constructor.title.as_str();

            let post = Post { id: id.to_string(), title: title.to_string() };

            // TODO replace this writing to / reading from a file with a Postgres DB
            //   place this behind an interface, so we can still use a text file DB if we want to
            match file.write(format!("{},{}\r\n", id.to_string(), title).as_bytes()) {
                Ok(_) => {
                    res.render(Json(post));
                },
                Err(e) => res.render(salvo::Error::other(format!("Error writing to database.txt: {}", e))),
            }
        },
        Err(e) => res.render(salvo::Error::other(format!("Error opening database.txt: {}", e))),
    }
}

#[endpoint(parameters(("id", description = "Pet id")))]
async fn get_post(req: &mut Request, res: &mut Response) {
    match req.param::<String>("id") {
        Some(id) => {
            match File::options().read(true).write(false).open("database.txt") {
                Ok(mut file) => {

                    let mut str = String::new();
                    let mut maybe_post: Option<Post> = None;

                    match file.read_to_string(&mut str) {
                        Ok(_) => {
                            let lines = str.split("\n").collect::<Vec<&str>>();
                            for line in lines {
                                let fields = line.split(",").collect::<Vec<&str>>();

                                if id == fields[0] {
                                    match Uuid::parse_str(fields[0]) {
                                        Ok(uuid) => {
                                            let id = uuid;
                                            let title = fields[1].to_string();
                                            maybe_post = Some(Post { id: id.to_string(), title: title.clone() });
                                            break;
                                        },
                                        Err(e) => res.render(salvo::Error::other(format!("Error parsing uuid: {}", e))),
                                    }
                                }
                            }

                            match maybe_post {
                                Some(post) => res.render(Json(post)),
                                None => res.render(salvo::Error::other(format!("Unknown uuid: {}", id))),
                            }
                        },
                        Err(e) => res.render(salvo::Error::other(format!("Error reading database.txt: {}", e))),
                    }
                }
                Err(e) => res.render(salvo::Error::other(format!("Error opening database.txt: {}", e))),
            }
        },
        None => res.render(salvo::Error::other("Missing uuid in request"))
    }
}

#[handler]
async fn get_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id: i32 = req.param::<i32>("id").expect("request did not contain an 'id: i32' field");

    let db = depot.obtain::<db::Database>().expect("Couldn't obtain database");

    match db.get_user(id).await {
        Ok(user) => res.render(Json(user)),
        Err(e) => res.render(salvo::Error::other(format!("Error getting user: {}", e))),
    }
}

#[handler]
async fn add_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id: i32 = req.param::<i32>("id").expect("request did not contain an 'id: i32' field");
    let name: String = req.param::<String>("name").expect("request did not contain a 'name: String' field");

    let db = depot.obtain_mut::<db::Database>().expect("Couldn't obtain database");

    match db.add_user(id, name).await {
        Ok(_) => res.render(Text::Plain("user added successfully".to_string())),
        Err(e) => res.render(salvo::Error::other(format!("Error adding user: {}", e))),
    }
}

#[handler]
async fn set_db(depot: &mut Depot) {
    depot.inject(Database{});
}

#[tokio::main]
async fn main() {

    Database::connect().unwrap();

    let acceptor = TcpListener::new("0.0.0.0:7878").bind().await;

    // TODO import regex package and enable this
    // PathFilter::register_wisp_regex(
    //     "guid",
    //     Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap(),
    // );

    let router = Router::new()
        .hoop(set_db)
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("post/create").post(create_post))
        .push(Router::with_path("post/{id}").get(get_post))
        .push(Router::with_path("user/add/{id}/{name}").get(add_user))
        .push(Router::with_path("user/get/{id}").get(get_user))
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