#[macro_use]
extern crate clap;

mod runtime;

use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine
};
use inkwell::{OptimizationLevel, AddressSpace};
use std::env;
// use std::fs::File;
// use std::io::prelude::*;

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

    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn_val = module.add_function("main", main_fn_type, Some(Linkage::External));
    
    let basic_block = context.append_basic_block(main_fn_val, "entry");
    builder.position_at_end(basic_block);

    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(AddressSpace::Generic);

    let data = builder.build_alloca(i8_ptr_type, "data");
    let ptr = builder.build_alloca(i8_ptr_type, "ptr");

    let i64_type = context.i64_type();
    let i64_memory_size = i64_type.const_int(30_000, false);
    let i64_element_size = i64_type.const_int(1, false);

    let calloc_fn_type = i8_ptr_type.fn_type(&[i64_type.into(), i64_type.into()], false);
    let calloc_fn_val = module.add_function("calloc", calloc_fn_type, Some(Linkage::External));

    let data_ptr = builder.build_call(calloc_fn_val, &[i64_memory_size.into(), i64_element_size.into()], "call");
    let data_ptr_result: Result<_, _> = data_ptr.try_as_basic_value().flip().into();
    let data_ptr_basic_val = data_ptr_result.map_err(|_| "calloc returned void for some reason!")?;

    builder.build_store(data, data_ptr_basic_val);
    builder.build_store(ptr, data_ptr_basic_val);

    // let source_filename = matches.value_of("INPUT").unwrap();
    // let mut f = File::open(source_filename).map_err(|e| format!("{:?}", e))?;
    // let mut program = Vec::new();
    // f.read_to_end(&mut program)
    //     .map_err(|e| format!("{:?}", e))?;

    let i32_zero = i32_type.const_int(0, false);
    builder.build_return(Some(&i32_zero));

    Target::initialize_all(&InitializationConfig::default());

    let target_triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();

    let target = Target::from_triple(&target_triple).map_err(|e| format!("{:?}", e))?;
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &cpu,
            &features,
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| "Unable to create target machine!".to_string())?;

    let output_filename = matches.value_of("output").unwrap();
    target_machine
        .write_to_file(&module, FileType::Object, output_filename.as_ref())
        .map_err(|e| format!("{:?}", e))
}
