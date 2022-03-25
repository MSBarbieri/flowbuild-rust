use diesel::{
    r2d2::{ConnectionManager, Pool},
};
#[cfg(feature = "sqlite")]
use diesel::SqliteConnection;
#[cfg(feature = "postgres")]
use diesel::PgConnection;

#[derive(Clone)]
pub(crate) struct PersistorProvider {
    #[cfg(feature = "postgres")]
    pub(crate) pool: Pool<ConnectionManager<PgConnection>>,
    #[cfg(feature = "sqlite")]
    pub(crate) pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[cfg(feature = "sqlite")]
lazy_static! {
    pub(crate) static ref PROVIDER: PersistorProvider = {
        let db_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(db_env);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        PersistorProvider { pool }
    };
}

#[cfg(feature = "postgres")]
lazy_static! {
    pub(crate) static ref PROVIDER: PersistorProvider = {
        let db_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_env);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        PersistorProvider { pool }
    };
}

impl PersistorProvider {
    pub fn get_provider() -> PersistorProvider {
        PROVIDER.clone()
    }

    #[cfg(feature = "postgres")]
    pub fn get_pool(&mut self) -> Pool<ConnectionManager<PgConnection>> {
        self.pool.clone()
    }

    #[cfg(feature = "sqlite")]
    pub fn get_pool(&mut self) -> Pool<ConnectionManager<SqliteConnection>> {
        self.pool.clone()
    }
}
