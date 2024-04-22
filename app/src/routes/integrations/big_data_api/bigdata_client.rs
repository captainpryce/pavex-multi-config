// This file is responsible for creating the HTTP client that will be used to make requests to the big data API.
use reqwest::Client;
//use crate::configuration::ApplicationConfig;
use crate::routes::integrations::big_data_api::bigdata_configuration::BigDataConfig;


#[derive(Clone)]
pub struct BigDataClient {
    pub(crate) client: Client,
    pub (crate) config: BigDataConfig,
}

impl BigDataClient {
    pub fn new(config: BigDataConfig) -> Self {
        Self {
            client: Client::new(),
            config, // we return the entire config and isolate the exact configurations we want inside the bigdata.rs submodule
        }
    }
}