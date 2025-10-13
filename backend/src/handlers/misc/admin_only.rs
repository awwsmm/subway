use salvo::oapi::endpoint;
use salvo::prelude::Text;
use salvo::Response;

/// Endpoint which can only be called by an authenticated admin.
#[endpoint]
pub(crate) async fn admin_only(res: &mut Response) {
    res.render(Text::Plain("welcome, administrator"))
}