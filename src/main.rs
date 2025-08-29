use salvo::catcher::Catcher;
use salvo::prelude::*;
use std::fs;

#[handler]
async fn hello(res: &mut Response) {
    let filename = "resources/hello.html";
    let contents = fs::read_to_string(filename).unwrap();
    res.render(Text::Html(contents))
}

#[handler]
async fn not_found(&self, _req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        let filename = "resources/404.html";
        let contents = fs::read_to_string(filename).unwrap();
        res.render(Text::Html(contents));

        // Skip remaining error handlers
        ctrl.skip_rest();
    }
}

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("0.0.0.0:8989").bind().await;

    let router = Router::new()
        .push(
            Router::with_path("hello").get(hello)
        );

    let catcher = Catcher::default().hoop(not_found);

    Server::new(acceptor).serve(Service::new(router).catcher(catcher)).await;
}