use crate::db::tables::authors_by_id::AuthorsByIdTableLike;
use crate::db::tables::posts_by_id::PostsByIdTableLike;
use crate::db::tables::{in_memory, postgres};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

// defines what a 'Table' is
pub(crate) mod table;

// gives a list of Tables
pub(crate) mod tables;

// list the Tables we want to use here
pub(crate) struct Database {
    pub(crate) posts_by_id: Box<dyn PostsByIdTableLike>,
    pub(crate) authors_by_id: Box<dyn AuthorsByIdTableLike>,
}

impl Database {
    pub(crate) fn new() -> Self {
        if std::env::var("SUBWAY_DB_MODE").is_ok_and(|env| env == "docker") {
            let db_url = std::env::var("SUBWAY_DB_URL").expect("SUBWAY_DB_URL not defined");
            let manager = ConnectionManager::<PgConnection>::new(db_url);
            let n_connections = 10;

            let pool = Pool::builder()
                .max_size(n_connections)
                .min_idle(Some(n_connections))
                .test_on_check_out(false)
                .idle_timeout(None)
                .max_lifetime(None)
                .build(manager);

            match pool {
                Err(_) => panic!("Database Pool Creation failed"),
                Ok(pool) => {
                    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

                    match pool.get() {
                        Ok(mut connection) => connection.run_pending_migrations(MIGRATIONS).unwrap(),
                        Err(_) => panic!("Unable to run database migrations!"),
                    };

                    let arc_pool = Arc::new(pool);

                    Database {
                        posts_by_id: Box::new(postgres::posts_by_id::Impl { connection_pool: Arc::clone(&arc_pool) }),
                        authors_by_id: Box::new(postgres::authors_by_id::Impl { connection_pool: Arc::clone(&arc_pool) }),
                    }
                }
            }

        } else {
            Database {
                posts_by_id: Box::new(in_memory::posts_by_id::Impl::new()),
                authors_by_id: Box::new(in_memory::authors_by_id::Impl::new()),
            }
        }
    }
}