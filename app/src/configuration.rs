use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::{linter::Lint, Blueprint};
use pavex::f;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use serde_aux::field_attributes::deserialize_number_from_string;
use secrecy::{ExposeSecret, Secret};

// Attribute applied to implement traits (derive) from serde::Derserialize, Debug and Clone. This is done to deserialize data,
// print helpful debug information and clone the data respectively.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    // here we are defining the bigdata field, returning the BigDataConfig struct as its type
    pub database: DatabaseConfig,
}

impl ApplicationConfig {
    // When the ApplicationConfig is implemented, we can call the big_data_config method, which takes a reference to the
    pub fn database_config(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn register(bp: &mut Blueprint) {
        bp.singleton(f!(self::ApplicationConfig::database_config));
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseConfig {
    /// Return the database connection options.
    pub fn connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }

    /// Return a database connection pool.
    pub async fn get_pool(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        let pool = sqlx::PgPool::connect_with(self.connection_options()).await?;
        Ok(pool)
    }
}
