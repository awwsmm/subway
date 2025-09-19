use salvo::oapi::endpoint;
use salvo::prelude::Text;
use salvo::Response;
use std::fs;

/// Endpoint that takes the user to a hello.html page.
#[endpoint]
pub(crate) async fn hello(res: &mut Response) {
    let filename = "resources/hello.html";
    let contents = fs::read_to_string(filename).unwrap();
    res.render(Text::Html(contents))
}