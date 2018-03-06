#![allow(dead_code)]


extern crate rls_analysis;
extern crate rls_span;
extern crate json;

mod resources;
mod analyses;


fn main() {
    let arguments = std::env::args();

    for arg in arguments {
        match &*arg {
            "hello" => analyses::check_hello(),
            "ogl" => analyses::check_ogl(),
            _ => {},
        }
    }
}
