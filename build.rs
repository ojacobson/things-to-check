use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;
use std::fs::File;
use std::io;
use thiserror::Error;

include!("src/thing.struct.rs");

impl From<String> for Thing {
    fn from(markdown: String) -> Self {
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown, options);

        let mut html = String::new();
        html::push_html(&mut html, parser);

        Thing { markdown, html }
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error("Unable to read file: {0}")]
    IOError(#[from] io::Error),
    #[error("Unable to parse file: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Unable to write generated source: {0}")]
    UnparseError(#[from] uneval::error::UnevalError),
}

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=things-to-check.yml");
    let f = File::open("things-to-check.yml")?;
    let things_raw: Vec<String> = serde_yaml::from_reader(f)?;
    let things: Vec<Thing> = things_raw.into_iter().map(Thing::from).collect();

    uneval::to_out_dir(things, "things_to_check.partial.rs")?;

    Ok(())
}
