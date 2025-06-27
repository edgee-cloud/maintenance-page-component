mod helpers;
mod world;

use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};
use world::bindings::Component;

impl Guest for Component {
    fn handle(_req: IncomingRequest, resp: ResponseOutparam) {
        let body = include_str!("maintenance.html");
        let response = helpers::build_response_html(body, 200);
        response.send(resp);
    }
}
