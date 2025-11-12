use std::fs;
use salvo::{handler, FlowCtrl, Response};
use salvo::http::StatusCode;
use salvo::prelude::Text;

/// Handler that takes the user to a default 404.html page.
#[handler]
pub(crate) async fn not_found(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
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