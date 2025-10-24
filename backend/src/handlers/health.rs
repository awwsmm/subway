use salvo::oapi::endpoint;
use salvo::prelude::StatusCode;
use salvo::Response;

/// Healthcheck endpoint
#[endpoint]
pub(crate) async fn check(res: &mut Response) {

    // TODO add checks for
    //  - DB connectivity and readiness
    //  - Auth connectivity and readiness

    res.status_code(StatusCode::OK);
}
