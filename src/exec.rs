use crate::data::Data;
use std::time::Instant;
use crate::import;
use proc_macro::TokenStream;

use watt_jit::*;

pub fn proc_macro(fun: &str, inputs: Vec<TokenStream>, wasm: &[u8]) -> TokenStream {
    let start = Instant::now();
    let engine = Engine::new();
    let store = Store::new(&engine);
    eprintln!("store at {:?}", start.elapsed());
    let module = Module::new(&store, wasm);
    eprintln!("module at {:?}", start.elapsed());
    let imports = import::extern_vals(&module, &store);
    eprintln!("imports at {:?}", start.elapsed());
    let module_instance = Instance::new(&store, &module, &imports).unwrap();
    eprintln!("instance at {:?}", start.elapsed());
    let main = module
        .exports()
        .iter()
        .position(|p| p.name() == fun)
        .expect(&format!("unresolved macro: {:?}", fun));
    let exports = module_instance.exports();
    let main = exports[main].func().unwrap();
    eprintln!("main at {:?}", start.elapsed());

    let _guard = Data::guard();
    let args = Data::with(|d| {
        inputs
            .into_iter()
            .map(|input| Val::i32(d.tokenstream.push(input)))
            .collect::<Vec<_>>()
    });

    let values = main.call(&args).unwrap();
    eprintln!("call at {:?}", start.elapsed());
    let handle = values.into_iter().next().unwrap();
    let handle = handle.as_i32().expect("unexpected macro return type");
    let ret = Data::with(|d| d.tokenstream[handle].clone());
    eprintln!("expanded in {:?}", start.elapsed());
    return ret;
}
