use std::fs;

use parsers::uml::parse_uml;

use crate::compilers::java::compile;
use crate::converters::java::convert;

mod models;
mod parsers;
mod helpers;
mod converters;
mod compilers;

fn main() {
    let contents = fs::read_to_string("test.puml").expect("No uml file");
    let uml = parse_uml(&contents).expect("Failed to parse uml");
    let java = convert(uml).expect("Failed to convert uml to java");
    compile(&java, "compiled/java/src/").expect("Failed to compile java code");
}