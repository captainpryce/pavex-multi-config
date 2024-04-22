use pavex::blueprint::{router::GET, Blueprint};
use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::linter::Lint;
use pavex::f;
use pavex::time::format_description::well_known::iso8601::Config;

// mod.rs files signify that the directory is a module. Any other .rs files inside of a directory are considered to be
// submodules
pub(crate) fn big_data_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    //BigDataConfig::register(&mut bp);
    // //bp.singleton(f!(self::bigdata_configuration::BigDataConfig::big_data_config), &Config)
    //     .cloning(CloningStrategy::CloneIfNecessary)
    //     .ignore(Lint::Unused);
    bp.route(GET, "/product_info/:upc", f!(self::bigdata::get_product_data));
    bp.singleton(f!(self::bigdata_client::BigDataClient::new))
        .cloning(CloningStrategy::CloneIfNecessary)// allows the HTTP client to be cloned. This would allow it to be used in multiple threads and have unique configurations depending on where it is used. E.g. different headers, timeouts, etc or maintaining different states.
        .ignore(Lint::Unused); // prevents linter from throwing an error if the singleton is not used
    bp
}

// this makes the bigdata module available to the rest of the application
// recall that modules are .rs files that are not the mod.rs file in a directory
pub mod bigdata;
pub mod bigdata_client;
pub mod bigdata_configuration;

// this lets us use all the methods in the bigdata sub module

//pub use bigdata::*;
// pub use bigdata_client::*;


// You want to avoid doing this, as the Pavex framework is designed to explicitly define what type of method (singleton, request_scoped, etc.) the injected method should be.
// This will help prevent race conditions and inappropriately calling methods without constructing them correctly. All injected methods should be done in the blueprint


