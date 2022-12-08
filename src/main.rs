use clap::Parser;
use std::fs::create_dir;
use std::path::Path;

pub mod db;
pub mod sjp;
use db::DbOperations;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "polish")]
    dict_name: String,

    #[arg(long, default_value = "20221201")]
    date: String,

    #[arg(long)]
    dest_dir: String,

    #[arg(long)]
    force: bool,

    #[arg(long)]
    hostname: String,

    #[arg(long)]
    username: String,

    #[arg(long)]
    password: String,

    #[arg(long)]
    database: String
}

fn main() {
    let args = Args::parse();
    let dest_path = Path::new(&args.dest_dir);
    if !dest_path.exists() {
        create_dir(&args.dest_dir).expect("Create directory");
    }
    sjp::download_and_save_stop_words(&dest_path);
    sjp::download_and_unpack(args.date, &dest_path);

    let mut db = DbOperations::new(args.dict_name, &args.hostname, &args.username, &args.password, &args.database);

    if args.force {
      db.drop();
    }
    db.create();
    db.check();
}
