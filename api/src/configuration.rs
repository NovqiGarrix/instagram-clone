use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub rust_env: String
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String
}

#[derive(Deserialize, Clone)]
pub struct JwtSettings {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub test_database: Option<DatabaseSettings>,
    pub jwt: JwtSettings
}

pub enum Environment {
    Development,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Development => "development",
            Self::Production => "production"
        }
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        match s.as_str() {
            "development" => Self::Development,
            "production" => Self::Production,
            other => panic!("{} is not a supported environment. Try development or production", other)
        }
    }

}

impl Settings {
    pub fn load_dotenv() {
        let env = std::env::var("APP_APPLICATION_RUST_ENV").unwrap_or_else(|_| "development".into());
        if env == "development" || env == "testing" {
            dotenv::dotenv().ok();
        }
    }

    pub fn get_configuration() -> Self {
        Self::load_dotenv();

        let base_path = std::env::current_dir()
            .expect("Failed to determine the current directory");
        let config_path = base_path.join("config");

        let environment: Environment = std::env::var("RUST_ENV")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .unwrap();
        let environment_filename = format!("{}.yaml", environment.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(config_path.join("base.yaml")))
            .add_source(config::File::from(config_path.join(environment_filename.as_str())))
            .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        if self.password.is_empty() {
            format!(
                "mysql://{}@{}:{}/{}",
                self.username,
                self.host,
                self.port,
                self.database_name
            )
        } else {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username,
                self.password,
                self.host,
                self.port,
                self.database_name
            )
        }

    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_configuration() {
        let settings = Settings::get_configuration();
        assert_eq!(settings.application.rust_env, "development".to_owned());
        assert!(!settings.application.host.is_empty());
        assert!(!settings.application.port.to_string().is_empty());
        assert!(!settings.jwt.private_key.is_empty());
    }

}