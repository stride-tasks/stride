#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleKind {
    Type,
    Method,
    Notification,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Struct(StructNode),
    Enum(EnumNode),
    Method(MethodNode),
    Notification(NotificationNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRef {
    Primitive(PrimitiveType),
    Array(Box<TypeRef>),
    Map(Box<TypeRef>),
    Object(String),
    Enum(String),
    Reference(String),
    Json,
    Unit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Uuid,
    Integer,
    Number,
    Boolean,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructNode {
    pub module: ModuleKind,
    pub name: String,
    pub doc: Vec<String>,
    pub fields: Vec<FieldNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldNode {
    pub name: String,
    pub type_ref: TypeRef,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumNode {
    pub module: ModuleKind,
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<EnumVariantNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodNode {
    pub module: ModuleKind,
    pub name: String,
    pub method_name: String,
    pub result: TypeRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationNode {
    pub module: ModuleKind,
    pub name: String,
    pub method_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumVariantNode {
    pub name: String,
    pub value: String,
}

impl Node {
    pub fn module(&self) -> ModuleKind {
        match self {
            Self::Struct(node) => node.module,
            Self::Enum(node) => node.module,
            Self::Method(node) => node.module,
            Self::Notification(node) => node.module,
        }
    }
}
