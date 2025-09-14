use crate::db::tables::posts_by_id::{InMemoryPostsByIdTable, PostgresPostsByIdTable, PostsByIdTableLike};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

pub(crate) mod tables;
pub(crate) mod table;

pub(crate) struct Database {
    pub(crate) posts_by_id: Box<dyn PostsByIdTableLike>,
}

impl Database {
    pub(crate) fn new() -> Self {
        if cfg!(feature = "postgres") {
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");

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
                Ok(pool) => {
                    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

                    match pool.get() {
                        Ok(mut connection) => connection.run_pending_migrations(MIGRATIONS).unwrap(),
                        Err(_) => panic!("Unable to run database migrations!"),
                    };

                    let arc_pool = Arc::new(pool);
                    Database { posts_by_id: Box::new(PostgresPostsByIdTable { connection_pool: arc_pool }) }
                },
                Err(_) => panic!("Database Pool Creation failed"),
            }
        } else {
            Database { posts_by_id: Box::new(InMemoryPostsByIdTable::new()) }
        }
    }
}