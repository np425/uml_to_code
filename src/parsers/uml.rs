use std::iter::Peekable;
use std::mem;
use std::str::Chars;

use crate::helpers::iter::ParserFuncs;
use crate::models::uml::*;

type Iter<'a> = Peekable<Chars<'a>>;

pub fn parse_uml(text: &str) -> Option<SourceTree> {
    let mut iter = text.chars().peekable();

    if !iter.expect_word("@startuml") {
        return None;
    }

    let st = SourceTree {
        root_namespace: parse_namespace(&mut iter)?
    };

    iter.expect_word("@enduml");
    Some(st)
}

fn parse_namespace(iter: &mut Iter<'_>) -> Option<Namespace> {
    let mut namespaces = vec![];
    let mut cur_ns = Namespace::default();

    loop {
        if iter.expect_char('}') {
            let mut prev_ns = namespaces.pop()?;
            mem::swap(&mut cur_ns, &mut prev_ns);
            cur_ns.namespaces.push(prev_ns);
        } else if iter.expect_word("namespace") {
            let mut next_ns = Namespace {
                name: iter.next_word()?,
                ..Default::default()
            };

            if !iter.expect_char('{') {
                cur_ns.namespaces.push(next_ns);
            } else {
                mem::swap(&mut cur_ns, &mut next_ns);
                namespaces.push(next_ns);
            }
        } else if let Some(cls) = parse_class(iter) {
            cur_ns.classes.push(cls);
        } else if let Some((child, rel)) = parse_relation(iter) {
            let child_cls = cur_ns.classes.iter_mut()
                .find(|cls| cls.name == child)?;
            child_cls.relations.push(rel);
        } else {
            break;
        }
    }

    Some(cur_ns)
}

fn parse_relation(iter: &mut Iter<'_>) -> Option<(String, Relation)> {
    let mut rel = Relation::default();
    rel.relates = iter.next_word()?;

    rel.rel_type = if iter.expect_word("<|--") {
        RelationType::Extends
    } else {
        return None;
    };

    let child = iter.next_word()?;
    Some((child, rel))
}

fn parse_class(iter: &mut Iter<'_>) -> Option<Class> {
    let mut cls = Class::default();

    cls.class_type = if iter.expect_word("class") {
        ClassType::Class
    } else if iter.expect_word("interface") {
        ClassType::Interface
    } else {
        return None;
    };

    cls.name = iter.next_word()?;

    if !iter.expect_char('{') {
        return Some(cls);
    }

    loop {
        if iter.expect_char('}') {
            return Some(cls);
        } else if let Some(field) = parse_field(iter) {
            cls.fields.push(field);
        } else {
            break;
        }
    }

    Some(cls)
}

fn parse_field(iter: &mut Iter<'_>) -> Option<Field> {
    let mut field = Field::default();

    field.access = if iter.expect_char('-') {
        Access::Private
    } else if iter.expect_char('+') {
        Access::Public
    } else {
        Access::Undefined
    };

    field.data = iter.next_line()?.trim().to_owned();
    (field.data != "").then_some(field)
}

