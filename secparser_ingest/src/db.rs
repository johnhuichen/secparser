use std::env;

use postgres::{Client, NoTls};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum ConstructorError {
    #[snafu(display("Failed to get db params"))]
    EnvVar { source: env::VarError },

    #[snafu(display("Failed to get db client"))]
    Postgres { source: postgres::Error },
}

pub struct PostgresDB {
    pub client: Client,
}

const SECPARSER_DB_USER: &str = "SECPARSER_DB_USER";
const SECPARSER_DB_PASSWORD: &str = "SECPARSER_DB_PASSWORD";
const SECPARSER_DB_HOST: &str = "SECPARSER_DB_HOST";
const SECPARSER_DB_DATABASE: &str = "SECPARSER_DB_DATABASE";

impl PostgresDB {
    pub fn new() -> Result<Self, ConstructorError> {
        let db_params = Self::get_db_params().context(EnvVarSnafu)?;
        let client = Client::connect(&db_params, NoTls).context(PostgresSnafu)?;

        Ok(PostgresDB { client })
    }

    fn get_db_params() -> Result<String, env::VarError> {
        let db_user = env::var(SECPARSER_DB_USER)?;
        let db_password = env::var(SECPARSER_DB_PASSWORD)?;
        let db_host = env::var(SECPARSER_DB_HOST)?;
        let db_database = env::var(SECPARSER_DB_DATABASE)?;

        Ok(format!(
            "postgresql://{}:{}@{}/{}",
            db_user, db_password, db_host, db_database
        ))
    }
}
