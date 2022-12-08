use clap::Parser;
use std::fs::create_dir;
use std::path::Path;

pub mod db;
pub mod sjp;
use db::DbOperations;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    dict_name: String,

    #[arg(long, default_value = "20221201")]
    date: String,

    #[arg(long)]
    dest_dir: String,

    #[arg(long)]
    force: bool
}

fn main() {
    let args = Args::parse();
    let dest_path = Path::new(&args.dest_dir);
    if !dest_path.exists() {
        create_dir(&args.dest_dir).expect("Create directory");
    }
    sjp::download_and_save_stop_words(&dest_path);
    sjp::download_and_unpack(args.date, &dest_path);

    let mut db = DbOperations::new(args.dict_name, "localhost", "postgres", "postgres");

    if args.force {
      db.drop();
    }
    db.create();
    db.check();
}
