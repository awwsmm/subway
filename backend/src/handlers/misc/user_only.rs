use salvo::oapi::endpoint;
use salvo::prelude::Text;
use salvo::{Depot, Response};

/// Endpoint which can only be called by an authenticated user.
#[endpoint]
pub(crate) async fn user_only(depot: &mut Depot, res: &mut Response) {
    let username = depot.get::<String>("token_user_name").cloned().unwrap_or(String::from("friend"));
    res.render(Text::Plain(format!("welcome, {}!", username)))
}