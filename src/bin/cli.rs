#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("cffc")
        .version("v0.1.0")
        .author("gopher9527 <gopher9527@gmail.com>")
        .about("convert between json,yaml and toml")
        .arg(
            Arg::with_name("From")
                .short("-f")
                .long("-from")
                .help("original file format: json,yaml,toml")
                .required(true),
        )
        .arg(
            Arg::with_name("To")
                .short("-t")
                .long("-tp")
                .help("target file format: json,yaml,toml")
                .required(true),
        )
        .arg(
            Arg::with_name("Input")
                .short("-i")
                .long("-input")
                .help("original file path")
                .required(true),
        )
        .arg(
            Arg::with_name("Output")
                .short("-o")
                .long("-output")
                .help("target file path")
                .required(true),
        )
        .help(
            r#"
        Usage: cffc OPTIONS
        
        Options:
            -f  original file format: json,yaml,toml
            -t  target file format: json,yaml,toml
            -i  original file path
            -o  target file path
        "#,
        );

    let matches = app.get_matches();
    let fv = matches.value_of("-f");
    println!("{:?}", fv);
}
