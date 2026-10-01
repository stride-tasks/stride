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
    Ref(TypeRef),
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
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<EnumVariantNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodNode {
    pub name: String,
    pub method_name: String,
    pub params: TypeNode,
    pub result: TypeNode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationNode {
    pub name: String,
    pub method_name: String,
    pub params: TypeNode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumVariantNode {
    pub name: String,
    pub value: String,
}
