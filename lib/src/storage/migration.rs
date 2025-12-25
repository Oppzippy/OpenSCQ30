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
    let version = usize::try_from(tx.query_row(
        "SELECT user_version FROM pragma_user_version",
        [],
        |row| row.get::<_, u32>(0),
    )?)
    .expect("current migration number should fit within usize");
    debug!("on migration version {version}");

    if version < migrations.len() {
        for (i, migration) in migrations[version..].iter().enumerate() {
            debug!("starting migration {}", version + i);
            match migration {
                Migration::Sql(sql) => {
                    tx.execute_batch(sql)?;
                }
            }
        }
        tx.pragma_update(
            None,
            "user_version",
            u32::try_from(migrations.len()).expect("number of migrations should fit in u32"),
        )?;
        debug!("migrated to {}", migrations.len());
    }
    tx.commit()
}
