use crate::helpers::iter::ParserFuncs;
use crate::models::java;
use crate::models::uml;
use crate::models::uml::RelationType;

enum JavaField {
    Field(java::Field),
    Method(java::Method),
}

pub fn convert(uml: uml::SourceTree) -> Option<java::SourceTree> {
    let java_tree = conv_package(uml.root_namespace)?;
    Some(java_tree)
}

fn conv_package(ns: uml::Namespace) -> Option<java::Package> {
    let mut pack = java::Package::default();

    for cls in ns.classes {
        match cls.class_type {
            uml::ClassType::Class => pack.classes.push(conv_class(cls)?),
            uml::ClassType::Interface => pack.interfaces.push(conv_interface(cls)?)
        }
    }

    for ns in ns.namespaces {
        pack.packages.push(conv_package(ns)?)
    }

    pack.name = ns.name;
    Some(pack)
}

fn conv_class(uml_cls: uml::Class) -> Option<java::Class> {
    let mut java_cls = java::Class::default();

    java_cls.name = uml_cls.name;

    for uml_field in uml_cls.fields {
        let field = conv_field(uml_field)?;
        match field {
            JavaField::Field(f) => java_cls.fields.push(f),
            JavaField::Method(m) => java_cls.methods.push(m)
        }
    }

    Some(java_cls)
}

fn conv_interface(uml_cls: uml::Class) -> Option<java::Interface> {
    let mut java_if = java::Interface::default();

    java_if.name = uml_cls.name;
    for uml_field in uml_cls.fields {
        let field = conv_field(uml_field)?;
        match field {
            JavaField::Method(m) => java_if.methods.push(m),
            _ => return None
        }
    }

    for rel in uml_cls.relations {
        match rel.rel_type {
            RelationType::Extends => java_if.extends.push(rel.relates)
        }
    }

    Some(java_if)
}

fn conv_field(uml_field: uml::Field) -> Option<JavaField> {
    let mut iter = uml_field.data.chars().peekable();

    let name = iter.next_word()?;
    let access = conv_access(&uml_field.access);

    if !iter.expect_char('(') {
        let data_type = resolve_data_type(iter.next_word()?);

        return Some(JavaField::Field(java::Field {
            var: java::Variable {
                name,
                data_type,
            },
            access,
        }));
    }

    let mut method = java::Method {
        name,
        access,
        ..Default::default()
    };

    loop {
        if iter.expect_char(')') {
            break;
        }

        let name = iter.next_word()?;
        let data_type = resolve_data_type(iter.next_word()?);

        method.params.push(java::Variable {
            name,
            data_type,
        });

        iter.expect_char(',');
    }

    method.return_type = match iter.next_word() {
        Some(data_type) => resolve_data_type(data_type),
        _ => "void".to_owned()
    };
    Some(JavaField::Method(method))
}

fn conv_access(uml_access: &uml::Access) -> java::Access {
    match uml_access {
        uml::Access::Undefined => java::Access::Internal,
        uml::Access::Private => java::Access::Private,
        uml::Access::Public => java::Access::Public,
    }
}

fn resolve_data_type(data_type: String) -> String {
    match data_type.as_str() {
        "string" => "String".to_owned(),
        _ => data_type,
    }
}

