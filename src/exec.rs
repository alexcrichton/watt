use crate::data::Data;
use crate::{debug, import};
use proc_macro::TokenStream;

pub fn proc_macro(fun: &str, inputs: Vec<TokenStream>, wasm: &[u8]) -> TokenStream {
    _proc_macro(fun, inputs, wasm)
}

#[cfg(jit)]
fn _proc_macro(fun: &str, inputs: Vec<TokenStream>, wasm: &[u8]) -> TokenStream {
    let start = std::time::Instant::now();
    let prev = std::time::Instant::now();
    use crate::runtime::*;

    let engine = Engine::new();
    println!("engine {:?}", prev.elapsed());
    let prev = std::time::Instant::now();
    let mut store = Store::new(&engine);
    println!("store {:?}", prev.elapsed());
    let prev = std::time::Instant::now();
    let module = Module::new(&store, wasm);
    println!("module {:?}", prev.elapsed());
    let prev = std::time::Instant::now();
    if cfg!(watt_debug) {
        debug::print_module(&module);
    }
    let imports = import::extern_vals(&module, &mut store);
    println!("imports {:?}", prev.elapsed());
    let prev = std::time::Instant::now();
    let module_instance = Instance::new(&store, &module, &imports).unwrap();
    let main = module
        .exports()
        .iter()
        .position(|p| p.name() == fun)
        .unwrap();
    println!("instance {:?}", prev.elapsed());
    let prev = std::time::Instant::now();
    let exports = module_instance.exports();
    let main = exports[main].func().unwrap();
    let memory = exports.iter().filter_map(|e| e.memory()).next().unwrap();
    println!("find main {:?}", prev.elapsed());
    let prev = std::time::Instant::now();

    let _guard = Data::guard();
    let args = Data::with(|d| {
        inputs
            .into_iter()
            .map(|input| Val::i32(d.tokenstream.push(input) as i32))
            .collect::<Vec<_>>()
    });
    println!("set data {:?}", prev.elapsed());
    let prev = std::time::Instant::now();


    let ret = current_memory::set(&memory, || {
        let values = main.call(&args).unwrap();
        println!("call {:?}", prev.elapsed());
        println!("call in wasm {:?}", prev.elapsed() - IMPORT_TIME.with(|c| c.get()));
        let handle = values.into_iter().next().unwrap();
        let handle = handle.as_i32().unwrap() as u32;
        Data::with(|d| d.tokenstream[handle].clone())
    });
    println!("entire macro (JIT) {:?}", start.elapsed());
    println!("import call time {:?}", IMPORT_TIME.with(|c| c.get()));
    TIME_BY_IMPORT.with(|c| {
        let c = c.borrow();
        let mut v = c.iter().collect::<Vec<_>>();
        v.sort_by_key(|p| p.1);
        println!("import call times {:#?}", v);
    });
    return ret;
}

#[cfg(not(jit))]
fn _proc_macro(fun: &str, inputs: Vec<TokenStream>, wasm: &[u8]) -> TokenStream {
    use crate::runtime::{
        decode_module, get_export, init_store, instantiate_module, invoke_func, ExternVal, Value,
    };
    use std::io::Cursor;

    let start = std::time::Instant::now();
    let cursor = Cursor::new(wasm);
    let module = decode_module(cursor).unwrap();
    if cfg!(watt_debug) {
        debug::print_module(&module);
    }

    let mut store = init_store();
    let extern_vals = import::extern_vals(&module, &mut store);
    let module_instance = instantiate_module(&mut store, module, &extern_vals).unwrap();
    let main = match get_export(&module_instance, fun) {
        Ok(ExternVal::Func(main)) => main,
        _ => unimplemented!("unresolved macro: {:?}", fun),
    };

    let _guard = Data::guard();
    let args = Data::with(|d| {
        inputs
            .into_iter()
            .map(|input| Value::I32(d.tokenstream.push(input)))
            .collect()
    });

    let res = invoke_func(&mut store, main, args);
    let values = match res {
        Ok(values) => values,
        Err(err) => panic!("{:?}", err),
    };
    let handle = values.into_iter().next().unwrap();
    let handle = match handle {
        Value::I32(handle) => handle,
        _ => unimplemented!("unexpected macro return type"),
    };
    println!("entire macro (interpreter) {:?}", start.elapsed());
    Data::with(|d| d.tokenstream[handle].clone())
}
