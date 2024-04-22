// This file is responsible for creating the HTTP client that will be used to make requests to the big data API.
use reqwest::Client;
use crate::configuration::ApplicationConfig;
//use crate::routes::integrations::big_data_api::bigdata_configuration::BigDataConfig;
// use crate::routes::integrations::big_data_api::big_data_bp;
// use crate::routes::integrations::big_data_api::bigdata_configuration::{BigDataConfig, BigDataVariables};


// Add any specific data needed for the API into the BigDataClient struct
#[derive(Clone)]
pub struct BigDataClient {
    pub(crate) client: Client,
    pub (crate) config: ApplicationConfig, // this will return the entire ApplicationConfig implementation, containing any methods from within.
    // pub(crate) config: ApplicationConfig, // this will return the entire ApplicationConfig implementation, containing any methods from within.
}
// TODO:: To fix this compile time error we need to properly convert this method into a constructor
// the new() method for the BigDataClient struct is properly registered as a constructor, but the way we are accessing the Config values is the
// reason why the code is failing
impl BigDataClient {
    // constructor function accepts a base_url as a parameter. If given, it will initialize the BigDataClient struct
    // Calling the new method creates a new instance of the BigDataClient struct, accepting a base_url as a parameter
    // Calling the new method will therefore return a new instance of the struct with the changed parameters, hence why we are returning Self in this.
    pub fn new(config: ApplicationConfig) -> Self {

        Self {
            client: Client::new(),
            config, // we return the entire config and isolate the exact configurations we want inside the bigdata.rs submodule
        }
    }
}