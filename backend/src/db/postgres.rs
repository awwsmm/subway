use crate::db::tables::posts_by_id::PostsByIdTableLike;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

// gives a list of Tables
pub(in crate::db) mod tables;

// list the Tables we want to use here
pub(crate) struct Database {
    pub(in crate::db) posts_by_id: Box<dyn PostsByIdTableLike>,
}

impl Database {
    pub(crate) fn new(url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(url);
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
                    posts_by_id: Box::new(tables::posts_by_id::Impl { connection_pool: Arc::clone(&arc_pool) }),
                }
            }
        }
    }
}