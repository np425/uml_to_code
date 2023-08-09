pub use GlobalScope as SourceTree;

#[derive(Debug, Default)]
pub struct GlobalScope {
    pub root_namespace: Namespace,
}

#[derive(Debug, Default)]
pub struct Namespace {
    pub namespaces: Vec<Namespace>,
    pub name: String,
    pub classes: Vec<Class>,
}

#[derive(Debug, Default)]
pub struct Class {
    pub class_type: ClassType,
    pub name: String,
    pub fields: Vec<Field>,
    pub relations: Vec<Relation>,
}

#[derive(Debug, Default)]
pub struct Field {
    pub data: String,
    pub access: Access,
}

#[derive(Debug, Default)]
pub struct Relation {
    pub relates: String,
    pub rel_type: RelationType,
}

#[derive(Debug, Default)]
pub enum RelationType {
    #[default]
    Extends,
}

#[derive(Debug, Default)]
pub enum ClassType {
    #[default]
    Class,
    Interface,
}

#[derive(Debug, Default)]
pub enum Access {
    #[default]
    Undefined,
    Private,
    Public,
}
