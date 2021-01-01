use rusqlite::Connection;
use refinery::embed_migrations;

embed_migrations!();

pub fn migrate_database(db: &str) {
    let mut conn = Connection::open(db).expect(&format!("Failed to open database {} for migration", db));
    migrations::runner().run(&mut conn).expect("Failed while trying to run migrations");
}
