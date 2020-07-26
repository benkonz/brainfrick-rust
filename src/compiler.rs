use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::{AddressSpace, OptimizationLevel};

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

struct Functions<'ctx> {
    calloc_fn: FunctionValue<'ctx>,
    getchar_fn: FunctionValue<'ctx>,
    putchar_fn: FunctionValue<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn init_targets() {
        Target::initialize_all(&InitializationConfig::default());
    }

    pub fn compile(&self, program: &[u8]) -> Result<(), String> {
        let functions = self.init_functions();
        let (data, ptr) = self.build_main();
        self.init_pointers(&functions, &data, &ptr)?;

        let mut pc = 0;
        while pc < program.len() {
            match program[pc] as char {
                '>' => self.build_add_ptr(1, &ptr),
                '<' => self.build_add_ptr(-1, &ptr),
                '+' => self.build_add(1, &ptr),
                '-' => self.build_add(-1, &ptr),
                '.' => self.build_put(&functions, &ptr),
                ',' => self.build_get(&functions, &ptr)?,
                '[' => self.build_while_start(),
                ']' => self.build_while_end(),
                _ => (),
            }
            pc += 1;
        }
        self.build_free(&data);
        self.return_zero();

        Ok(())
    }

    fn init_functions(&self) -> Functions {
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        let i8_type = self.context.i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::Generic);

        let calloc_fn_type = i8_ptr_type.fn_type(&[i64_type.into(), i64_type.into()], false);
        let calloc_fn = self
            .module
            .add_function("calloc", calloc_fn_type, Some(Linkage::External));

        let getchar_fn_type = i32_type.fn_type(&[], false);
        let getchar_fn =
            self.module
                .add_function("getchar", getchar_fn_type, Some(Linkage::External));

        let putchar_fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let putchar_fn =
            self.module
                .add_function("putchar", putchar_fn_type, Some(Linkage::External));
        Functions {
            calloc_fn,
            getchar_fn,
            putchar_fn,
        }
    }

    fn build_main(&self) -> (PointerValue, PointerValue) {
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn_val = self
            .module
            .add_function("main", main_fn_type, Some(Linkage::External));
        let basic_block = self.context.append_basic_block(main_fn_val, "entry");
        self.builder.position_at_end(basic_block);

        let i8_type = self.context.i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::Generic);

        let data = self.builder.build_alloca(i8_ptr_type, "data");
        let ptr = self.builder.build_alloca(i8_ptr_type, "ptr");

        (data, ptr)
    }

    fn init_pointers(
        &self,
        functions: &Functions,
        data: &PointerValue,
        ptr: &PointerValue,
    ) -> Result<(), String> {
        let i64_type = self.context.i64_type();
        let i64_memory_size = i64_type.const_int(30_000, false);
        let i64_element_size = i64_type.const_int(1, false);

        let data_ptr = self.builder.build_call(
            functions.calloc_fn,
            &[i64_memory_size.into(), i64_element_size.into()],
            "calloc_call",
        );
        let data_ptr_result: Result<_, _> = data_ptr.try_as_basic_value().flip().into();
        let data_ptr_basic_val =
            data_ptr_result.map_err(|_| "calloc returned void for some reason!")?;

        self.builder.build_store(*data, data_ptr_basic_val);
        self.builder.build_store(*ptr, data_ptr_basic_val);

        Ok(())
    }

    fn build_add_ptr(&self, amount: i32, ptr: &PointerValue) {
        let i32_type = self.context.i32_type();
        let i32_amount = i32_type.const_int(amount as u64, false);
        let ptr_load = self
            .builder
            .build_load(*ptr, "load ptr")
            .into_pointer_value();
        let result = self.builder.build_int_add(
            ptr_load.const_to_int(i32_type),
            i32_amount,
            "add to data ptr",
        );
        self.builder.build_store(*ptr, result);
    }

    fn build_add(&self, amount: i8, ptr: &PointerValue) {
        let i8_type = self.context.i8_type();
        let i8_amount = i8_type.const_int(amount as u64, false);
        let ptr_load = self
            .builder
            .build_load(*ptr, "load ptr")
            .into_pointer_value();
        let ptr_val = self.builder.build_load(ptr_load, "load ptr value");
        let result = self
            .builder
            .build_int_add(ptr_val.into_int_value(), i8_amount, "add to data ptr");
        self.builder.build_store(ptr_load, result);
    }

    fn build_get(&self, functions: &Functions, ptr: &PointerValue) -> Result<(), String> {
        let getchar_call = self
            .builder
            .build_call(functions.getchar_fn, &[], "getchar call");
        let getchar_result: Result<_, _> = getchar_call.try_as_basic_value().flip().into();
        let getchar_basicvalue =
            getchar_result.map_err(|_| "getchar returned void for some reason!")?;
        let i8_type = self.context.i8_type();
        let truncated = self.builder.build_int_truncate(
            getchar_basicvalue.into_int_value(),
            i8_type,
            "getchar truncate result",
        );
        let ptr_value = self
            .builder
            .build_load(*ptr, "load ptr value")
            .into_pointer_value();
        self.builder.build_store(ptr_value, truncated);

        Ok(())
    }

    fn build_put(&self, functions: &Functions, ptr: &PointerValue) {
        let char_to_put = self.builder.build_load(
            self.builder
                .build_load(*ptr, "load ptr value")
                .into_pointer_value(),
            "load ptr ptr value",
        );
        let s_ext = self.builder.build_int_s_extend(
            char_to_put.into_int_value(),
            self.context.i32_type(),
            "putchar sign extend",
        );
        self.builder
            .build_call(functions.putchar_fn, &[s_ext.into()], "putchar call");
    }

    fn build_while_start(&self) {}

    fn build_while_end(&self) {}

    fn build_free(&self, data: &PointerValue) {
        self.builder
            .build_free(self.builder.build_load(*data, "load").into_pointer_value());
    }

    fn return_zero(&self) {
        let i32_type = self.context.i32_type();
        let i32_zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&i32_zero));
    }

    pub fn write_to_file(&self, output_filename: &str) -> Result<(), String> {
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

        target_machine
            .write_to_file(&self.module, FileType::Object, output_filename.as_ref())
            .map_err(|e| format!("{:?}", e))
    }
}
