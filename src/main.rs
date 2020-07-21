#[macro_use]
extern crate clap;

mod runtime;

use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine, InitializationConfig};
use inkwell::OptimizationLevel;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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

    let context = Context::create();
    let module = context.create_module("brainfrick-rust");
    let builder = context.create_builder();

    let source_filename = matches.value_of("INPUT").unwrap();
    let mut f = File::open(source_filename).map_err(|e| format!("{:?}", e))?;
    let mut program = Vec::new();
    f.read_to_end(&mut program)
        .map_err(|e| format!("{:?}", e))?;

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).map_err(|e| format!("{:?}", e))?;
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| "Unable to create target machine!".to_string())?;

    let output_filename = matches.value_of("output").unwrap();
    target_machine
        .write_to_file(&module, FileType::Assembly, output_filename.as_ref())
        .map_err(|e| format!("{:?}", e))?;

    runtime::execute(&program)
}
