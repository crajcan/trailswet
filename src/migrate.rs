use refinery::config::{Config, ConfigDbType};
use std::env;

pub mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("db/migrations");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_host(&env::var("DB_HOST")?)
        .set_db_port(&env::var("DB_PORT")?)
        .set_db_name(&env::var("DB_NAME")?)
        .set_db_user(&env::var("DB_USER")?)
        .set_db_pass(&env::var("DB_PASS")?);
    println!("Running migrations");
    embedded::migrations::runner().run(&mut conn)?;
    Ok(())
}
