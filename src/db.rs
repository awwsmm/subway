#[cfg(feature = "postgres")]
use diesel::dsl::insert_into;

#[cfg(feature = "postgres")]
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

#[cfg(feature = "postgres")]
use diesel::{table, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable};

#[cfg(feature = "postgres")]
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

#[cfg(not(feature = "postgres"))]
use std::collections::HashMap;

#[cfg(not(feature = "postgres"))]
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use serde::Serialize;

#[cfg(feature = "postgres")]
#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = users)]
pub(crate) struct User {
    id: i32,
    name: String,
}

#[cfg(not(feature = "postgres"))]
#[derive(Clone, Serialize)]
pub(crate) struct User {
    id: i32,
    name: String,
}

pub(crate) struct Database;

// below copied from https://crates.io/crates/diesel-async

#[cfg(feature = "postgres")]
table! {
    users(id) {
        id -> Integer,
        name -> Text,
    }
}

#[cfg(feature = "postgres")]
type PgPool = Pool<ConnectionManager<PgConnection>>;

#[cfg(feature = "postgres")]
pub(crate) static DB_POOL: OnceCell<PgPool> = OnceCell::new();

#[cfg(feature = "postgres")]
pub(crate) fn connect() -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
    DB_POOL.get().unwrap().get()
}

#[cfg(feature = "postgres")]
pub(crate) fn build_pool(database_url: &str, size: u32) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(size)
        .min_idle(Some(size))
        .test_on_check_out(false)
        .idle_timeout(None)
        .max_lifetime(None)
        .build(manager)
}

#[cfg(feature = "postgres")]
pub(crate) const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[cfg(not(feature = "postgres"))]
pub(crate) static IN_MEMORY_DB: OnceCell<Mutex<HashMap<i32, User>>> = OnceCell::new();

impl Database {

    pub(crate) fn connect() -> Result<(), String> {

        #[cfg(feature = "postgres")]
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");

        #[cfg(feature = "postgres")]
        DB_POOL
            .set(build_pool(&db_url, 10).expect(&format!("Error connecting to {}", &db_url)))
            .ok();

        #[cfg(not(feature = "postgres"))]
        IN_MEMORY_DB
            .set(Mutex::new(HashMap::new()))
            .ok();

        #[cfg(feature = "postgres")]
        match connect() {
            Ok(mut connection) => {
                connection.run_pending_migrations(MIGRATIONS).unwrap();
            }

            Err(_) => {}
        }

        Ok(())
    }

    pub(crate) async fn add_user(&mut self, id: i32, name: String) -> Result<(), String> {

        #[cfg(feature = "postgres")]
        match connect() {
            Ok(mut connection) => {
                let user = User { id, name };
                match insert_into(users::table).values(&user).execute(&mut connection) {
                    Ok(_user) => Ok(()),
                    Err(e) => Err(format!("Unable to insert User: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }

        #[cfg(not(feature = "postgres"))]
        {
            let user = User { id, name };

            // set
            IN_MEMORY_DB.get().unwrap().lock().unwrap().insert(id, user);

            // get
            match IN_MEMORY_DB.get().unwrap().lock().unwrap().get(&id) {
                None => Err("unable to add user to in-memory db".to_string()),
                Some(_) => Ok(())
            }
        }
    }

    pub(crate) async fn get_user(&self, id: i32) -> Result<User, String> {

        #[cfg(feature = "postgres")]
        match connect() {
            Ok(mut connection) => {
                match users::table.find(id).first::<User>(&mut connection) {
                    Ok(user) => Ok(user),
                    Err(e) => Err(format!("Unable to find User: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }

        #[cfg(not(feature = "postgres"))]
        {

            match IN_MEMORY_DB.get().unwrap().lock().unwrap().get(&id) {
                None => Err("unable to find user in in-memory db".to_string()),
                Some(user) => Ok(user.clone()),
            }
        }
    }
}



