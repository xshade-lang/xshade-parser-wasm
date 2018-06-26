#![feature(proc_macro, wasm_import_module, wasm_custom_section)]

extern crate xshade_parser;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(program: &str) -> String {
    let ast = xshade_parser::parse_str(program).unwrap();
    let serialized_ast = xshade_parser::serialize(&ast).unwrap();
    return serialized_ast;
}
