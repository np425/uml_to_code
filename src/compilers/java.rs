use std::fs;
use std::path::{Path, PathBuf};

use crate::models::java::*;

pub fn compile(java: &SourceTree, path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)?;

    for pack in &java.packages {
        compile_package(pack, Path::new(path).to_path_buf())?;
    }

    Ok(())
}

pub fn compile_package(pack: &Package, path: PathBuf) -> std::io::Result<()> {
    let path = path.join(&pack.name);
    fs::create_dir_all(&path)?;

    for interface in &pack.interfaces {
        compile_interface(interface, pack, path.clone())?;
    }

    for cls in &pack.classes {
        compile_class(cls, pack, path.clone())?;
    }

    for pack in &pack.packages {
        compile_package(pack, path.clone())?;
    }

    Ok(())
}

pub fn compile_class(cls: &Class, pack: &Package, path: PathBuf) -> std::io::Result<()> {
    let mut contents = format!("package {};\nclass {} {{\n", pack.name, cls.name);

    for field in &cls.fields {
        compile_field(field, &mut contents);
        contents.push('\n');
    }

    for method in &cls.methods {
        compile_method(method, &mut contents, true);
        contents.push('\n');
    }

    contents.push('}');

    fs::write(path.join(cls.name.clone() + ".java"), contents)
}

pub fn compile_interface(interface: &Interface, pack: &Package, path: PathBuf) -> std::io::Result<()> {
    let mut contents = format!("package {};\ninterface {}", pack.name, interface.name);

    if !interface.extends.is_empty() {
        contents.push_str(" extends ");

        for extend in &interface.extends {
            contents.push_str(extend);
            contents.push(',');
        }

        contents.pop();
    }

    contents.push_str(" {\n");

    for method in &interface.methods {
        compile_method(method, &mut contents, false);
        contents.push('\n');
    }

    contents.push('}');

    fs::write(path.join(interface.name.clone() + ".java"), contents)
}

pub fn compile_field(field: &Field, text: &mut String) {
    compile_access(&field.access, text);
    compile_variable(&field.var, text);
    text.push(';');
}

pub fn compile_method(method: &Method, text: &mut String, return_value: bool) {
    compile_access(&method.access, text);
    text.push_str(&method.return_type);
    text.push(' ');
    text.push_str(&method.name);
    text.push('(');

    for param in &method.params {
        compile_variable(param, text);
        text.push(',');
    }

    if text.ends_with(',') {
        text.pop();
    }

    text.push(')');

    if !return_value {
        text.push(';');
        return;
    }

    text.push_str(" {");

    if method.return_type != "void" {
        text.push_str("\nreturn ");
        text.push_str(resolve_default_value(&method.return_type));
        text.push_str(";\n");
    }

    text.push('}');
}

pub fn compile_access(access: &Access, text: &mut String) {
    match access {
        Access::Private => text.push_str("private "),
        Access::Public => text.push_str("public "),
        _ => {}
    }
}

pub fn compile_variable(variable: &Variable, text: &mut String) {
    text.push_str(&variable.data_type);
    text.push(' ');
    text.push_str(&variable.name);
}

fn resolve_default_value(data_type: &str) -> &str {
    match data_type {
        "void" => "",
        _ => "null"
    }
}

