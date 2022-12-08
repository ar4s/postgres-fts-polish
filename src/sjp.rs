use bzip2::read::BzDecoder;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, path::Path};
use tar::Archive;

const BASE: &str = "https://sjp.pl/sl/ort/";
const STOP_WORDS_URL: &str =
    "https://raw.githubusercontent.com/bieli/stopwords/master/polish.stopwords.txt";

pub fn download_and_save_stop_words(dir_dest: &Path) {
    let content = get(STOP_WORDS_URL)
        .expect("Download stop words")
        .text()
        .expect("Text unpack");
    let mut file = File::create(dir_dest.join("polish.stop")).expect("File polish.stop");
    file.write_all(&content.as_bytes())
        .expect("Write polish.stop");
}

pub fn download_and_unpack(date: String, dir_dest: &Path) {
    let file_mapper = HashMap::from([
        ("polish.aff", "polish.affix"),
        ("polish.all", "polish.dict"),
    ]);
    let target = format!("{BASE}sjp-ispell-pl-{date}-src.tar.bz2");
    println!("{}", target);
    let content = get(target).expect("Get request failed");
    let bytes = content.bytes().expect("Read bytes failed");
    let slices = bytes.to_vec();
    let bz = BzDecoder::new(&*slices);
    let mut archive = Archive::new(bz);
    let entries = archive.entries();

    for file in entries.unwrap() {
        let mut file = file.unwrap();

        let path = file.header().path().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let check = file_mapper.get(&file_name);
        match check {
            Some(f) => {
                println!("Found! {}", f);
                file.unpack(dir_dest.join(f)).expect("unpack error");
            }
            None => println!("Skipping {file_name}..."),
        }
    }
}
