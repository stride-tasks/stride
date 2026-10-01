use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::node::{
    EnumNode, EnumVariantNode, FieldNode, MethodNode, Node, NotificationNode, PrimitiveType,
    StructNode, TypeNode, TypeRef,
};
use crate::{Result, Schema, SchemaType};

/// Parse all schema files beneath a directory into generated Rust/Flutter model nodes.
///
/// # Errors
/// Returns an error if a schema file cannot be read or its JSON cannot be parsed.
pub fn parse(dir: &Path) -> Result<Vec<Node>> {
    let mut files = collect_schema_files(dir)?;

    // Sort files to ensure consistent order of processing and output generation.
    files.sort();

    let mut seen = BTreeSet::new();
    let mut nodes = Vec::new();
    for path in &files {
        let json = fs::read_to_string(path)?;
        let schema = Schema::from_str(&json)?;

        match module_name_for_path(path) {
            "r#type" => emit_type_schema(&schema, path, &mut seen, &mut nodes),
            "method" => emit_method_schema(&schema, path, &mut seen, &mut nodes),
            "notification" => emit_notification_schema(&schema, path, &mut seen, &mut nodes),
            _ => unreachable!(),
        }
    }
    Ok(nodes)
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

fn emit_type_schema(
    schema: &Schema,
    path: &Path,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) {
    let root_name = schema_name_from_path(path, schema);

    if schema
        .enum_values
        .as_ref()
        .is_some_and(|values| !values.is_empty())
    {
        drop(type_ref_for_schema(schema, &root_name, seen, output));
        return;
    }

    drop(emit_object_schema(schema, &root_name, seen, output));
}

fn emit_method_schema(
    schema: &Schema,
    path: &Path,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) {
    let Some(properties) = schema.properties.as_ref() else {
        return;
    };

    let method_name =
        root_method_name(schema).unwrap_or_else(|| schema_name_from_path(path, schema));

    let params_schema = properties.get("params");
    let result_schema = properties.get("result");

    if let Some(params_schema) = params_schema {
        let params_name = schema_name_from_path(path, schema);
        let params_type = emit_object_schema(params_schema, &params_name, seen, output);
        let result_type = match result_schema {
            Some(result_schema) => resolve_result_type(
                result_schema,
                &format!("{}Result", schema_name_from_path(path, schema)),
                seen,
                output,
            ),
            None => TypeRef::Unit,
        };

        output.push(Node::Method(MethodNode {
            name: params_name,
            method_name: method_name.clone(),
            params: TypeNode::Ref(TypeRef::Object(params_type)),
            result: TypeNode::Ref(result_type),
        }));
        return;
    }

    let payload_name = schema_name_from_path(path, schema);
    let payload_type = emit_object_schema(schema, &payload_name, seen, output);
    let result_type = match result_schema {
        Some(result_schema) => resolve_result_type(
            result_schema,
            &format!("{payload_name}Result"),
            seen,
            output,
        ),
        None => TypeRef::Unit,
    };

    output.push(Node::Method(MethodNode {
        name: payload_name,
        method_name,
        params: TypeNode::Ref(TypeRef::Object(payload_type)),
        result: TypeNode::Ref(result_type),
    }));
}

fn emit_notification_schema(
    schema: &Schema,
    path: &Path,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) {
    let method_name =
        root_method_name(schema).unwrap_or_else(|| schema_name_from_path(path, schema));
    let Some(properties) = schema.properties.as_ref() else {
        return;
    };

    if properties.contains_key("params") {
        let params_schema = properties.get("params").unwrap();
        let params_name = schema_name_from_path(path, schema);
        let params_type = emit_object_schema(params_schema, &params_name, seen, output);

        output.push(Node::Notification(NotificationNode {
            name: params_name,
            method_name: method_name.clone(),
            params: TypeNode::Ref(TypeRef::Object(params_type)),
        }));
        return;
    }

    let payload_name = schema_name_from_path(path, schema);
    let payload_type = emit_object_schema(schema, &payload_name, seen, output);
    output.push(Node::Notification(NotificationNode {
        name: payload_name,
        method_name,
        params: TypeNode::Ref(TypeRef::Object(payload_type)),
    }));
}

fn resolve_result_type(
    schema: &Schema,
    preferred_name: &str,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) -> TypeRef {
    if schema.const_value.is_some() {
        if schema.const_value == Some(serde_json::Value::Object(serde_json::Map::new())) {
            return TypeRef::Unit;
        }
        return type_ref_for_schema(schema, preferred_name, seen, output);
    }

    if schema.properties.is_some() {
        let type_name = emit_object_schema(schema, preferred_name, seen, output);
        return TypeRef::Object(type_name);
    }

    if schema.schema_type.as_ref().and_then(SchemaType::as_str) == Some("array") {
        return type_ref_for_schema(schema, preferred_name, seen, output);
    }

    if schema.schema_type.as_ref().and_then(SchemaType::as_str) == Some("object") {
        return type_ref_for_schema(schema, preferred_name, seen, output);
    }

    type_ref_for_schema(schema, preferred_name, seen, output)
}

fn schema_to_node(
    schema: &Schema,
    preferred_name: &str,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) -> Option<Node> {
    let explicit_name = explicit_struct_name(schema);
    let target_name = explicit_name
        .clone()
        .unwrap_or_else(|| preferred_name.to_owned());

    if let Some(enum_values) = schema.enum_values.as_ref() {
        if enum_values.is_empty() {
            return None;
        }

        let enum_name = normalize_schema_name(&target_name);
        let variants = enum_values
            .iter()
            .filter_map(|value| match value {
                serde_json::Value::String(s) => Some((enum_variant_name(s), s.clone())),
                serde_json::Value::Number(n) => {
                    let raw = n
                        .as_i64()
                        .map(|i| i.to_string())
                        .or_else(|| n.as_f64().map(|f| f.to_string()))?;
                    Some((enum_variant_name(&raw), raw))
                }
                serde_json::Value::Bool(b) => {
                    Some((enum_variant_name(&b.to_string()), b.to_string()))
                }
                _ => None,
            })
            .map(|(name, value)| EnumVariantNode { name, value })
            .collect::<Vec<_>>();

        if variants.is_empty() {
            return None;
        }

        return Some(Node::Type(TypeNode::Enum(EnumNode {
            name: enum_name,
            description: schema.description.clone(),
            variants,
        })));
    }

    let properties = schema.properties.as_ref()?;

    let required = schema
        .required
        .clone()
        .unwrap_or_default()
        .into_iter()
        .collect::<HashSet<_>>();

    let skip_reserved_method_fields = explicit_name.is_none();
    let mut fields = Vec::new();
    for (key, value) in properties {
        if skip_reserved_method_fields && matches!(key.as_str(), "method" | "params" | "result") {
            continue;
        }

        let field_type =
            type_ref_for_schema(value, &append_type_name(&target_name, key), seen, output);

        fields.push(FieldNode {
            name: key.clone(),
            type_ref: field_type,
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

    Some(Node::Type(TypeNode::Struct(StructNode {
        name: target_name,
        doc,
        fields,
    })))
}

fn emit_object_schema(
    schema: &Schema,
    preferred_name: &str,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) -> String {
    let explicit_name = explicit_struct_name(schema);
    let target_name = explicit_name
        .clone()
        .unwrap_or_else(|| preferred_name.to_owned());

    if !seen.insert(target_name.clone()) {
        return target_name;
    }

    if let Some(node) = schema_to_node(schema, &target_name, seen, output) {
        output.push(node);
        return target_name;
    }

    target_name
}

#[allow(clippy::too_many_lines)]
fn type_ref_for_schema(
    schema: &Schema,
    fallback_name: &str,
    seen: &mut BTreeSet<String>,
    output: &mut Vec<Node>,
) -> TypeRef {
    if let Some(reference) = schema.ref_.as_deref() {
        return TypeRef::Reference(resolve_reference(reference));
    }

    if let Some(items) = schema.items.as_deref() {
        let inner = type_ref_for_schema(
            items,
            &append_type_name(fallback_name, "Item"),
            seen,
            output,
        );
        return TypeRef::Array(Box::new(inner));
    }

    if let Some(enum_values) = schema.enum_values.as_ref()
        && !enum_values.is_empty()
    {
        let enum_name = normalize_schema_name(fallback_name);
        if seen.contains(&enum_name) {
            return TypeRef::Reference(enum_name);
        }

        let variants: Vec<EnumVariantNode> = enum_values
            .iter()
            .filter_map(|value| match value {
                serde_json::Value::String(s) => Some((enum_variant_name(s), s.clone())),
                serde_json::Value::Number(n) => {
                    let raw = n
                        .as_i64()
                        .map(|i| i.to_string())
                        .or_else(|| n.as_f64().map(|f| f.to_string()))?;
                    Some((enum_variant_name(&raw), raw))
                }
                serde_json::Value::Bool(b) => {
                    Some((enum_variant_name(&b.to_string()), b.to_string()))
                }
                _ => None,
            })
            .map(|(name, value)| EnumVariantNode { name, value })
            .collect::<Vec<_>>();

        if !variants.is_empty() {
            let node = Node::Type(TypeNode::Enum(EnumNode {
                name: enum_name.clone(),
                description: schema.description.clone(),
                variants,
            }));
            output.push(node);
            seen.insert(enum_name.clone());
            return TypeRef::Enum(enum_name);
        }
    }

    if let Some(const_value) = &schema.const_value {
        return match const_value {
            serde_json::Value::String(_) => TypeRef::Primitive(PrimitiveType::String),
            serde_json::Value::Number(_) => TypeRef::Primitive(PrimitiveType::Integer),
            serde_json::Value::Bool(_) => TypeRef::Primitive(PrimitiveType::Boolean),
            serde_json::Value::Object(_)
            | serde_json::Value::Array(_)
            | serde_json::Value::Null => TypeRef::Json,
        };
    }

    if let Some(explicit_name) = explicit_struct_name(schema) {
        if explicit_name == "Type" || explicit_name == "Method" || explicit_name == "Notification" {
            return TypeRef::Object(explicit_name);
        }
        if seen.contains(&explicit_name) {
            return TypeRef::Reference(explicit_name);
        }
        drop(emit_object_schema(schema, &explicit_name, seen, output));
        return TypeRef::Object(explicit_name);
    }

    match schema.schema_type.as_ref().and_then(SchemaType::as_str) {
        Some("string") => {
            if schema.format.as_deref() == Some("uuid") {
                TypeRef::Primitive(PrimitiveType::Uuid)
            } else {
                TypeRef::Primitive(PrimitiveType::String)
            }
        }
        Some("boolean") => TypeRef::Primitive(PrimitiveType::Boolean),
        Some("integer") => TypeRef::Primitive(PrimitiveType::Integer),
        Some("number") => TypeRef::Primitive(PrimitiveType::Number),
        Some("array") => {
            let inner = schema.items.as_deref().map_or(TypeRef::Json, |item| {
                type_ref_for_schema(item, &append_type_name(fallback_name, "Item"), seen, output)
            });
            TypeRef::Array(Box::new(inner))
        }
        Some("object") => {
            if let Some(_object_props) = schema.properties.as_ref() {
                let type_name = normalize_schema_name(fallback_name);
                if seen.contains(&type_name) {
                    return TypeRef::Reference(type_name);
                }
                drop(emit_object_schema(schema, &type_name, seen, output));
                TypeRef::Object(type_name)
            } else if schema.additional_properties.is_some() {
                TypeRef::Map(Box::new(TypeRef::Json))
            } else {
                TypeRef::Json
            }
        }
        _ => {
            if schema.properties.is_some() {
                let type_name = normalize_schema_name(fallback_name);
                if seen.contains(&type_name) {
                    return TypeRef::Reference(type_name);
                }
                drop(emit_object_schema(schema, &type_name, seen, output));
                TypeRef::Object(type_name)
            } else if schema.additional_properties.is_some() {
                TypeRef::Map(Box::new(TypeRef::Json))
            } else {
                TypeRef::Json
            }
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
    schema
        .properties
        .as_ref()?
        .get("method")?
        .const_value
        .as_ref()?
        .as_str()
        .map(str::to_owned)
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
        if matches!(lower.as_str(), "stride" | "type") {
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

fn explicit_struct_name(schema: &Schema) -> Option<String> {
    let comment = schema.comment.as_deref()?;
    let marker = "@name:";
    let name = comment.strip_prefix(marker)?;
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(normalize_schema_name(trimmed))
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
