use crate::{configuration, routes, telemetry};
use pavex::blueprint::Blueprint;
use pavex::f;
use pavex::kit::ApiKit;
use crate::routes::integrations::big_data_api::big_data_bp;

// Here we are registering the nested blueprint for the integrations module. Within here, I further nest another blueprint
// for the big_data_api module. This is a good example of how to nest blueprints within each other.
pub(crate) fn integrations_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.nest_at("/big_data_api", big_data_bp());
    bp
}
pub mod big_data_api;
