use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Type(TypeNode),
    Method(MethodNode),
    Notification(NotificationNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeNode {
    Struct(StructNode),
    Enum(EnumNode),
    Primitive(PrimitiveType),
    Array(Box<TypeNode>),
    Map(Box<TypeNode>),
    Reference(String),
    Any,
    Unit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Uuid,
    Bytes,
    Uri,
    Integer,
    UnsignedInteger,
    Number,
    Boolean,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructNode {
    pub name: String,
    pub id: String,
    pub doc: Vec<String>,
    pub fields: Vec<FieldNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldNode {
    pub name: String,
    pub typ: TypeNode,
    pub required: bool,
    pub description: Option<String>,
    pub default: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumNode {
    pub name: String,
    pub id: String,
    pub doc: Vec<String>,
    pub description: Option<String>,
    /// Discriminant property name; set for tagged enums whose variants carry fields.
    pub tag: Option<String>,
    pub variants: Vec<EnumVariantNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodNode {
    pub id: String,
    pub name: String,
    pub method_name: String,
    pub description: Option<String>,
    pub params: TypeNode,
    pub result: TypeNode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationNode {
    pub name: String,
    pub method_name: String,
    pub params: TypeNode,
    pub description: Option<String>,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumVariantNode {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub fields: Vec<FieldNode>,
}
