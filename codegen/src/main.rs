// This crate crawls the Syn source directory to find all structs and enums that
// form the Syn syntax tree.
//
// A machine-readable representation of the syntax tree is saved to syn.json in
// the repo root for other code generation tools to consume. The syn-codegen
// crate (https://docs.rs/syn-codegen/) provides the data structures for parsing
// and making use of syn.json from Rust code.
//
// Finally this crate generates the Visit, VisitMut, and Fold traits in Syn
// programmatically from the syntax tree description.

#![recursion_limit = "128"]
#![allow(clippy::needless_pass_by_value)]

mod debug;
mod error;
mod file;
mod fold;
mod full;
mod gen;
mod json;
mod operand;
mod parse;
mod version;
mod visit;
mod visit_mut;

use crate::error::Result;
use std::process;

fn main() {
    if let Err(err) = do_main() {
        let _ = eprintln!("error: {}", err);
        process::exit(1);
    }
}

fn do_main() -> Result<()> {
    let defs = parse::parse()?;
    json::generate(&defs)?;
    fold::generate(&defs)?;
    visit::generate(&defs)?;
    visit_mut::generate(&defs)?;
    debug::generate(&defs)?;
    Ok(())
}
