pub mod status;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/api/ping", f!(self::status::ping)); // this route is a default route given when running pavex new. This can effectively be deleted, as well as the
}

pub mod integrations;


