use std::io;

use inkwell::{context::Context, OptimizationLevel};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(& mut input).expect("failed to read line");
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::from(1u16));

    let printf_fn_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
    let printf_fn = module.add_function("printf", printf_fn_type, None);

    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, None);

    let entry_basic_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry_basic_block);

    let input_string_ptr = builder.build_global_string_ptr(&input, "input");
    builder.build_call(printf_fn, &[input_string_ptr.as_pointer_value().into()], "call");
    builder.build_return(Some(&i32_type.const_int(0, false)));

    let exec_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    unsafe {
        exec_engine.get_function::<unsafe extern "C" fn()>("main").unwrap().call();
    }
}
