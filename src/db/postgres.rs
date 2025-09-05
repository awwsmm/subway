use crate::db::{users, Database, DatabaseLike, User};
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use once_cell::sync::OnceCell;

// below copied from https://crates.io/crates/diesel-async

static CONNECTION_POOL: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();

fn build_pool(database_url: &str, size: u32) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(size)
        .min_idle(Some(size))
        .test_on_check_out(false)
        .idle_timeout(None)
        .max_lifetime(None)
        .build(manager)
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

impl DatabaseLike for Database {

    async fn connect() -> Result<(), String> {

        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");

        CONNECTION_POOL
            .set(build_pool(&db_url, 10).expect(&format!("Error connecting to {}", &db_url)))
            .ok();

        match CONNECTION_POOL.get().unwrap().get() {
            Ok(mut connection) => {
                connection.run_pending_migrations(MIGRATIONS).unwrap();
            }

            Err(_) => {}
        }

        Ok(())
    }

    async fn add_user(&mut self, id: i32, name: String) -> Result<(), String> {
        Self::connect().await?;

        match CONNECTION_POOL.get().unwrap().get() {
            Ok(mut connection) => {
                let user = User { id, name };
                match insert_into(users::table).values(&user).execute(&mut connection) {
                    Ok(_user) => Ok(()),
                    Err(e) => Err(format!("Unable to insert User: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }
    }

    async fn get_user(&self, id: i32) -> Result<User, String> {
        Self::connect().await?;

        match CONNECTION_POOL.get().unwrap().get() {
            Ok(mut connection) => {
                match users::table.find(id).first::<User>(&mut connection) {
                    Ok(user) => Ok(user),
                    Err(e) => Err(format!("Unable to find User: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }
    }
}



