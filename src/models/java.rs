pub type SourceTree = Package;
pub type DataType = String;

#[derive(Debug, Default)]
pub struct Package {
    pub packages: Vec<Package>,
    pub name: String,
    pub classes: Vec<Class>,
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Default)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<Method>,
    pub extends: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Method {
    pub name: String,
    pub params: Vec<Variable>,
    pub return_type: DataType,
    pub access: Access,
}

#[derive(Debug, Default)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Default)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Default)]
pub struct Field {
    pub var: Variable,
    pub access: Access,
}

#[derive(Debug, Default)]
pub enum Access {
    #[default]
    Internal,
    Private,
    Public,
}
