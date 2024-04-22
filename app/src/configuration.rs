use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::{linter::Lint, Blueprint};
use pavex::f;

// Attribute applied to implement traits (derive) from serde::Derserialize, Debug and Clone. This is done to deserialize data,
// print helpful debug information and clone the data respectively.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    // here we are defining the bigdata field, returning the BigDataConfig struct as its type
    pub bigdata: BigDataConfig,
}

impl ApplicationConfig {
    // When the ApplicationConfig is implemented, we can call the big_data_config method, which takes a reference to the
    // ApplicationConfig as a parameter, and returns a reference to the BigDataConfig struct
    pub fn big_data_config (&self) -> &BigDataConfig {
        // here we are returning the entire BigDataConfig struct by first referencing the ApplicationConfig struct
        // and then isolating for the bigdata field, which as the BigDataConfig struct as its type
        &self.bigdata
    }

    pub fn register(bp: &mut Blueprint) {
        bp.singleton(f!(self::ApplicationConfig::big_data_config))
            .cloning(CloningStrategy::CloneIfNecessary)
            .ignore(Lint::Unused);
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BigDataConfig {
    // These are the fields in the BigDataConfig struct
    // rapid_api_app is considered the name and the String is considered its type
    pub rapid_api_app: String,
    pub rapid_api_key: String,
    pub rapid_api_host: String,
    pub rapid_api_request_url: String,
}

impl BigDataConfig {
    pub fn new(&self) -> &Self {
        // returns the entire BigDataConfig struct when the BigDataConfig struct is implemented.
        // if you wanted only the rapid_api_app or other keys in the struct, you would return those instead by first referencing self, and isolating
        // the variables needed.
        &self
    }
}
