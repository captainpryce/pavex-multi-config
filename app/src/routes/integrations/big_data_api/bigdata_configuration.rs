#[derive(serde::Deserialize, Debug, Clone)]
pub struct BigDataVariables {
    pub rapid_api_app: String,
    pub rapid_api_key: String,
    pub rapid_api_host: String,
    pub rapid_api_request_url: String,
}

impl BigDataVariables {
    pub fn new(&self) -> &Self {
        &self
    }
}
#[derive(serde::Deserialize, Debug, Clone)]
pub struct BigDataConfig {
    pub bigdata: BigDataVariables
}
impl BigDataConfig {
    pub fn big_data_config(&self) -> &BigDataVariables {
        &self.bigdata
    }
}