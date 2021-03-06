#![warn(clippy::all, rust_2018_idioms)]

use cargo_registry::{db, models::Crate, schema::crates};

use clap::Clap;
use diesel::prelude::*;

mod dialoguer;

#[derive(Clap, Debug)]
#[clap(
    name = "delete-crate",
    about = "Purge all references to a crate from the database.\n\nPlease be super sure you want to do this before running this."
)]
struct Opts {
    /// Name of the crate
    crate_name: String,
}

fn main() {
    let conn = db::connect_now().unwrap();
    conn.transaction::<_, diesel::result::Error, _>(|| {
        delete(&conn);
        Ok(())
    })
    .unwrap()
}

fn delete(conn: &PgConnection) {
    let opts: Opts = Opts::parse();

    let krate: Crate = Crate::by_name(&opts.crate_name).first(conn).unwrap();

    let prompt = format!(
        "Are you sure you want to delete {} ({})?",
        opts.crate_name, krate.id
    );
    if !dialoguer::confirm(&prompt) {
        return;
    }

    println!("deleting the crate");
    let n = diesel::delete(crates::table.find(krate.id))
        .execute(conn)
        .unwrap();
    println!("  {} deleted", n);

    if !dialoguer::confirm("commit?") {
        panic!("aborting transaction");
    }
}
