use std::sync::atomic::{AtomicUsize, Ordering};

use sqlx::{postgres::PgPool, Executor};
use thiserror::Error;
use tracing::info;

struct Migration {
    name: String,
    sql: String,
    seq: usize,
}

impl Migration {
    fn new(name: String, sql: String, seq: AtomicUsize) -> Self {
        Self {
            name,
            sql,
            seq: seq.fetch_add(1, Ordering::Relaxed),
        }
    }

    async fn execute(&self, pool: &PgPool) -> Result<(), SchemaError> {
        let mut tx = pool.begin().await?;

        let query = sqlx::raw_sql(&self.sql);
        tx.execute(query).await?;

        tx.commit().await?;

        info!("completed migration: {:04} {}", self.seq, self.name);
        Ok(())
    }
}

macro_rules! migration {
    ($file:literal, $seq:expr) => {
        Migration::new($file.into(), include_str!($file).into(), $seq)
    };
}

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Sql error: {0}")]
    Sql(#[from] sqlx::Error),
}

pub async fn migrate(pool: &PgPool) -> Result<(), SchemaError> {
    let seq = AtomicUsize::new(0);
    let mut migrations = vec![migration!("../sql/01_init.sql", seq)];
    migrations.sort_by_key(|m| m.seq);

    for migration in migrations {
        migration.execute(&pool).await?;
    }
    Ok(())
}
