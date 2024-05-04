use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

fn on_launch() -> Result<()> {
    let conn = Connection::open("theSplit.db")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}
