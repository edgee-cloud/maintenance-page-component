mod helpers;
mod world;

use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};
use world::bindings::Component;

impl Guest for Component {
    fn handle(_req: IncomingRequest, resp: ResponseOutparam) {
        let body = include_str!("maintenance.html");
        let mut builder = helpers::ResponseBuilder::new();
        builder
            .set_header("content-type", "text/html")
            .set_status_code(200)
            .set_body(body);

        builder.build(resp);
    }
}
