#[macro_use]
extern crate clap;

mod compiler;

use crate::compiler::Compiler;
use clap::{App, Arg};
use inkwell::context::Context;
use std::fs;

fn main() -> Result<(), String> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("source bf file to compile")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("output filename")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    Compiler::init_targets();

    let context = Context::create();
    let compiler = Compiler {
        context: &context,
        module: context.create_module("brainfrick-rust"),
        builder: context.create_builder(),
    };

    let source_filename = matches.value_of("INPUT").unwrap();
    let program = fs::read_to_string(source_filename).map_err(|e| format!("{:?}", e))?;

    compiler.compile(program)?;
    let output_filename = matches.value_of("output").unwrap();
    compiler.write_to_file(output_filename)
}
