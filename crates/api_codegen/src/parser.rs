use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::node::{
    EnumNode, EnumVariantNode, FieldNode, MethodNode, Node, NotificationNode, PrimitiveType,
    StructNode, TypeNode,
};
use crate::schema::{SchemaString, SchemaType};
use crate::{Error, Result, Schema, SchemaConcreteType};

#[derive(Debug, Default)]
struct Context {
    output: Vec<Node>,
}

/// Parse all schema files beneath a directory into generated Rust/Flutter model nodes.
///
/// # Errors
/// Returns an error if a schema file cannot be read or its JSON cannot be parsed.
pub fn parse(dir: &Path) -> Result<Vec<Node>> {
    let mut files = collect_schema_files(dir)?;

    // Sort files to ensure consistent order of processing and output generation.
    files.sort();

    let mut context = Context::default();
    for path in &files {
        let json = fs::read_to_string(path)?;
        let schema = Schema::from_str(&json).map_err(|source| Error::Json {
            path: path.to_string_lossy().into(),
            source,
        })?;

        match module_name_for_path(path) {
            "r#type" => {
                let root_name = schema_name_from_path(path, &schema);
                let type_node = type_for_schema(&schema, &root_name);
                context.output.push(Node::Type(type_node));
            }
            "method" => {
                let method = emit_method_schema(&schema, path);
                context.output.push(Node::Method(method));
            }
            "notification" => {
                let notification = emit_notification_schema(&schema, path);
                context.output.push(Node::Notification(notification));
            }
            _ => unreachable!(),
        }
    }
    Ok(context.output)
}

fn collect_schema_files(api_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for subdir in ["method", "notification", "type"] {
        let dir = api_dir.join(subdir);
        if !dir.exists() {
            continue;
        }
        let mut entries = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                entries.push(path);
            }
        }
        files.append(&mut entries);
    }
    Ok(files)
}

fn module_name_for_path(path: &Path) -> &str {
    let path_str = path.to_string_lossy();
    if path_str.contains("/method/") || path_str.contains("\\method\\") {
        "method"
    } else if path_str.contains("/notification/") || path_str.contains("\\notification\\") {
        "notification"
    } else {
        "r#type"
    }
}

fn emit_method_schema(schema: &Schema, path: &Path) -> MethodNode {
    let SchemaType::Concrete(concrete_type) = &schema.schema_type else {
        panic!("method should not be a reference: {path:?}");
    };
    let SchemaConcreteType::Object { properties, .. } = concrete_type else {
        panic!("method schema must be an object");
    };

    let method_name =
        root_method_name(schema).unwrap_or_else(|| schema_name_from_path(path, schema));

    let params_schema = properties
        .get("params")
        .expect("method must have params property");
    let result_schema = properties
        .get("result")
        .expect("method must have result property");

    let params_name = schema_name_from_path(path, schema);
    let params_type = type_for_schema(params_schema, &params_name);
    let result_type = type_for_schema(
        result_schema,
        &format!("{}Result", schema_name_from_path(path, schema)),
    );

    MethodNode {
        id: schema
            .id
            .clone()
            .unwrap_or_else(|| panic!("method should have an $id: {}", path.display())),
        description: schema.description.clone(),
        name: params_name,
        method_name: method_name.clone(),
        params: params_type,
        result: result_type,
    }
}

fn emit_notification_schema(schema: &Schema, path: &Path) -> NotificationNode {
    let method_name =
        root_method_name(schema).unwrap_or_else(|| schema_name_from_path(path, schema));

    let SchemaType::Concrete(concrete_type) = &schema.schema_type else {
        panic!("notification should not be a reference: {path:?}");
    };
    let SchemaConcreteType::Object { properties, .. } = concrete_type else {
        panic!("notification schema must be an object: {path:?}");
    };

    let params_schema = properties
        .get("params")
        .expect("notification must have params property: {path:?}");
    let params_name = schema_name_from_path(path, schema);
    let params_type = type_for_schema(params_schema, &params_name);

    NotificationNode {
        id: schema
            .id
            .clone()
            .unwrap_or_else(|| panic!("notification should have an $id: {}", path.display())),
        description: schema.description.clone(),
        name: params_name,
        method_name: method_name.clone(),
        params: params_type,
    }
}

#[allow(clippy::too_many_lines)]
fn type_for_schema(schema: &Schema, fallback_name: &str) -> TypeNode {
    let concrete_type = match &schema.schema_type {
        SchemaType::Any { .. } => {
            return TypeNode::Any;
        }
        SchemaType::Reference { ref_ } => {
            return TypeNode::Reference(resolve_reference(ref_));
        }
        SchemaType::Concrete(concrete_type) => concrete_type,
    };

    let explicit_name = schema.explicit_name();
    let target_name = explicit_name
        .clone()
        .unwrap_or_else(|| fallback_name.to_owned());

    match concrete_type {
        SchemaConcreteType::String {
            kind: SchemaString::Enum { enum_values },
            ..
        } => {
            let enum_name = normalize_schema_name(fallback_name);

            let variants = enum_values
                .iter()
                .map(|s| (enum_variant_name(s), s.clone()))
                .map(|(name, value)| EnumVariantNode { name, value })
                .collect::<Vec<_>>();

            assert!(
                !variants.is_empty(),
                "Enum values are empty for schema: {schema:?}"
            );

            let mut doc = Vec::new();
            if let Some(title) = &schema.title {
                doc.push(title.clone());
            }
            if let Some(description) = &schema.description {
                if !doc.is_empty() {
                    doc.push(String::new());
                }
                for line in description.split('\n') {
                    doc.push(line.to_owned());
                }
            }

            let enum_node = EnumNode {
                name: enum_name,
                id: schema.id.clone().unwrap_or_default(),
                doc,
                description: schema.description.clone(),
                variants,
            };
            TypeNode::Enum(enum_node)
        }
        SchemaConcreteType::Array { items } => TypeNode::Array(Box::new(type_for_schema(
            items,
            &append_type_name(fallback_name, "Item"),
        ))),
        SchemaConcreteType::Boolean { .. } => TypeNode::Primitive(PrimitiveType::Boolean),
        SchemaConcreteType::Integer { .. } => TypeNode::Primitive(PrimitiveType::Integer),
        SchemaConcreteType::Number { .. } => TypeNode::Primitive(PrimitiveType::Number),
        SchemaConcreteType::String {
            kind: SchemaString::String { format },
            ..
        } => {
            if format.as_deref() == Some("uuid") {
                TypeNode::Primitive(PrimitiveType::Uuid)
            } else {
                TypeNode::Primitive(PrimitiveType::String)
            }
        }
        SchemaConcreteType::Null {} => TypeNode::Any,
        SchemaConcreteType::Object {
            properties,
            required,
            ..
        } => {
            let required = required.clone().into_iter().collect::<HashSet<_>>();

            let mut fields = Vec::new();
            for (key, value) in properties {
                let field_type = type_for_schema(value, &append_type_name(&target_name, key));

                fields.push(FieldNode {
                    name: key.clone(),
                    typ: field_type,
                    required: required.contains(key),
                    description: value.description.clone(),
                });
            }

            let mut doc = Vec::new();
            if let Some(title) = &schema.title {
                doc.push(title.clone());
            }
            if let Some(description) = &schema.description {
                if !doc.is_empty() {
                    doc.push(String::new());
                }
                for line in description.split('\n') {
                    doc.push(line.to_owned());
                }
            }

            TypeNode::Struct(StructNode {
                name: target_name,
                id: schema.id.clone().unwrap_or_default(),
                doc,
                fields,
            })
        }
    }
}

fn resolve_reference(reference: &str) -> String {
    let stem = Path::new(reference)
        .file_stem()
        .and_then(|part| part.to_str())
        .unwrap_or(reference)
        .strip_suffix(".schema")
        .unwrap_or(reference);
    normalize_path_name(stem)
}

fn root_method_name(schema: &Schema) -> Option<String> {
    let SchemaType::Concrete(concrete_type) = &schema.schema_type else {
        return None;
    };
    let SchemaConcreteType::Object { properties, .. } = concrete_type else {
        return None;
    };

    let SchemaType::Concrete(concrete_type) = &properties.get("method")?.schema_type else {
        return None;
    };
    let SchemaConcreteType::String { const_value, .. } = concrete_type else {
        return None;
    };

    Some(const_value.as_ref()?.as_str().to_owned())
}

fn schema_name_from_path(path: &Path, schema: &Schema) -> String {
    let from_file = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(normalize_path_name);
    let from_title = schema.title.clone();
    let base = from_file
        .or(from_title)
        .unwrap_or_else(|| "GeneratedSchema".to_owned());
    let normalized = normalize_schema_name(&base);
    apply_module_suffix(normalized, module_name_for_path(path))
}

fn apply_module_suffix(name: String, module: &str) -> String {
    match module {
        "method" => {
            if name.ends_with("Method") {
                name
            } else {
                format!("{name}Method")
            }
        }
        "notification" => {
            if name.ends_with("Notification") {
                name
            } else {
                format!("{name}Notification")
            }
        }
        _ => name,
    }
}

fn normalize_path_name(name: &str) -> String {
    let without_schema = name
        .strip_suffix(".schema")
        .unwrap_or(name)
        .replace("r#", "")
        .replace('#', " ");
    let mut tokens = Vec::new();

    let mut is_method = false;
    let mut is_notification = false;
    for candidate in split_name_parts(&without_schema) {
        let cleaned = candidate.trim().to_owned();
        if cleaned.is_empty() {
            continue;
        }
        let lower = cleaned.to_ascii_lowercase();
        if matches!(lower.as_str(), "method") {
            is_method = true;
            continue;
        }
        if matches!(lower.as_str(), "notification") {
            is_notification = true;
            continue;
        }
        tokens.push(cleaned);
    }

    if is_method {
        tokens.push("Method".to_owned());
    } else if is_notification {
        tokens.push("Notification".to_owned());
    }

    if tokens.is_empty() {
        return "GeneratedSchema".to_owned();
    }

    let mut out = String::new();
    for token in tokens {
        let mut chars = token.chars();
        let first = chars.next().unwrap_or_default();
        let rest = chars.as_str();
        out.push(first.to_ascii_uppercase());
        out.push_str(&rest.to_ascii_lowercase());
    }
    out
}

fn normalize_schema_name(name: &str) -> String {
    let without_schema = name
        .strip_suffix(".schema")
        .unwrap_or(name)
        .replace("r#", "")
        .replace('#', " ");
    let mut tokens = Vec::new();

    let mut is_method = false;
    let mut is_notification = false;
    for candidate in split_name_parts(&without_schema) {
        let cleaned = candidate.trim().to_owned();
        if cleaned.is_empty() {
            continue;
        }
        let lower = cleaned.to_ascii_lowercase();
        if matches!(lower.as_str(), "stride") {
            continue;
        }
        if matches!(lower.as_str(), "method") {
            is_method = true;
            continue;
        }
        if matches!(lower.as_str(), "notification") {
            is_notification = true;
            continue;
        }
        tokens.push(cleaned);
    }

    if is_method {
        tokens.push("Method".to_owned());
    } else if is_notification {
        tokens.push("Notification".to_owned());
    }

    if tokens.is_empty() {
        return "GeneratedSchema".to_owned();
    }

    let mut out = String::new();
    for token in tokens {
        let mut chars = token.chars();
        let first = chars.next().unwrap_or_default();
        let rest = chars.as_str();
        out.push(first.to_ascii_uppercase());
        out.push_str(&rest.to_ascii_lowercase());
    }

    out
}

fn split_name_parts(value: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = value.chars().collect();

    for (index, ch) in chars.iter().copied().enumerate() {
        if matches!(ch, '.' | '-' | '_' | '/' | ' ' | ':' | '#') {
            if !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
            continue;
        }

        let prev = chars.get(index.saturating_sub(1)).copied();
        let next = chars.get(index + 1).copied();
        if let (Some(prev), Some(next)) = (prev, next) {
            let split_before =
                (prev.is_ascii_lowercase() || prev.is_ascii_digit()) && ch.is_ascii_uppercase();
            let split_after_upper =
                prev.is_ascii_uppercase() && ch.is_ascii_uppercase() && next.is_ascii_lowercase();
            if (split_before || split_after_upper) && !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
        }

        current.push(ch);
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

fn append_type_name(base: &str, segment: &str) -> String {
    if base.is_empty() {
        normalize_schema_name(segment)
    } else {
        format!("{base}{}", normalize_schema_name(segment))
    }
}

fn enum_variant_name(value: &str) -> String {
    let normalized = value
        .split(['-', '_', ' ', '.', '/', ':'])
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    if normalized.is_empty() {
        return "Unknown".to_owned();
    }

    let mut out = String::new();
    for part in normalized {
        let mut chars = part.chars();
        let first = chars.next().unwrap_or_default();
        let rest = chars.as_str();
        out.push(first.to_ascii_uppercase());
        out.push_str(&rest.to_ascii_lowercase());
    }
    out
}
