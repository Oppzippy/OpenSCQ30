use rusqlite::{Connection, TransactionBehavior};
use tracing::{debug, instrument};

pub enum Migration {
    Sql(&'static str),
}

macro_rules! migration_file {
    ($name:literal) => {
        Migration::Sql(include_str!(concat!("migrations/", $name)))
    };
}

pub const MIGRATIONS: &[Migration] = &[migration_file!("0.sql")];

#[instrument(skip(connection, migrations))]
pub fn migrate(
    connection: &mut Connection,
    migrations: &[Migration],
) -> Result<(), rusqlite::Error> {
    let tx = connection.transaction_with_behavior(TransactionBehavior::Exclusive)?;
    let version = tx.query_row("SELECT user_version FROM pragma_user_version", [], |row| {
        row.get(0)
    })?;
    debug!("on migration version {version}");
    if version < migrations.len() {
        for (i, migration) in migrations[version..].iter().enumerate() {
            debug!("starting migration {}", version + i);
            match migration {
                Migration::Sql(sql) => {
                    tx.execute(sql, ())?;
                }
            }
        }
        tx.pragma_update(None, "user_version", &migrations.len())?;
        debug!("migrated to {}", migrations.len());
    }
    tx.commit()
}
