use crate::{routes, telemetry};
use pavex::blueprint::Blueprint;
use pavex::kit::ApiKit;
use crate::configuration::ApplicationConfig;
use crate::routes::integrations::integrations_bp;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);
    telemetry::register(&mut bp);
    ApplicationConfig::register(&mut bp); // This registers the ApplicationConfig as a constructor that can be injected throughout the entire application starting from the current directory.
    routes::register(&mut bp); // This line registers the router. Within the routes directory, the register method can be used to register any routes by calling them with the bp.routes method. This line can be removed if you are not using the                              //default routes and decoupling your routes inside of blueprints for modules.
    bp.nest(integrations_bp()); // This line tells the parent blueprint that we have nested a blueprint called integrations_bp. Pavex will search through starting from the root directory to locate this method.
    bp
}
