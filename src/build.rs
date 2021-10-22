use man::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let path = "conchsh.1";
    let mut output = File::create(path)?;

    let msg = Manual::new("conchsh")
        .about("A shell in rust.")
        .arg(Arg::new("path"))
        .example(
            Example::new()
                .text("Running the program")
                .command("conchsh [FILE]...")
                .output("TODO"),
        )
        .author(Author::new("Takashi I").email("mail@takashiidobe.com"))
        .render();

    write!(output, "{}", msg)
}
