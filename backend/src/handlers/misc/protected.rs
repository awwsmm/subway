use salvo::oapi::endpoint;
use salvo::prelude::Text;
use salvo::{Request, Response};

/// Endpoint which can only be called by an authenticated user.
#[endpoint]
pub(crate) async fn protected(req: &mut Request, res: &mut Response) {
    // // Assuming a successful authentication, the claims will be in the request extensions
    // let claims = req.ext::<KeycloakToken<Role>>().unwrap();
    // let username = claims.subject.as_ref().unwrap();

    res.render(Text::Plain("welcome, authenticated user"))
}