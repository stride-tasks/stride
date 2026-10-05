use super::*;
use crate::node::{Node, TypeNode};

use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn generate_with_schema(schema_json: &str) -> Result<String> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("stride_api_codegen_schema_{unique}"));
    fs::create_dir_all(dir.join("type")).unwrap();
    fs::write(dir.join("type").join("generated.schema.json"), schema_json).unwrap();
    generate_rust(&dir)
}

#[test]
fn generates_empty_object_as_struct() {
    let output = generate_with_schema(
        r#"{
            "type": "object",
            "properties": {}
        }"#,
    )
    .unwrap();

    assert!(
        output.contains("pub struct Generated {"),
        "output was:\n{output}"
    );
    assert!(
        output.contains("pub struct Generated {\n}"),
        "output was:\n{output}"
    );
}

#[test]
fn keeps_explicitly_named_nested_objects_with_method_fields() {
    let output = generate_with_schema(
        r#"{
            "$comment": "@name:UserPromptTarget",
            "type": "object",
            "properties": {
                "method": { "type": "string" },
                "params": {
                    "type": "object",
                    "properties": {
                        "repository_id": { "type": "string", "format": "uuid" }
                    },
                    "required": ["repository_id"]
                }
            },
            "required": ["method", "params"]
        }"#,
    )
    .unwrap();

    assert!(
        output.contains("pub struct UserPromptTarget"),
        "output was:\n{output}"
    );
    assert!(
        output.contains("pub method: Box<str>"),
        "output was:\n{output}"
    );
    assert!(output.contains("pub params:"), "output was:\n{output}");
}

#[test]
fn stores_inline_object_fields_as_actual_type_nodes() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("stride_api_codegen_inline_object_{unique}"));
    fs::create_dir_all(dir.join("type")).unwrap();
    fs::write(
        dir.join("type").join("stride.user.schema.json"),
        r#"{
            "title": "User",
            "type": "object",
            "properties": {
                "profile": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" }
                    },
                    "required": ["name"]
                }
            },
            "required": ["profile"]
        }"#,
    )
    .unwrap();

    let nodes = parse(&dir).unwrap();
    let root = nodes
        .iter()
        .find_map(|node| match node {
            Node::Type(TypeNode::Struct(node)) if node.name == "User" => Some(node),
            _ => None,
        })
        .unwrap();

    let profile_field = root
        .fields
        .iter()
        .find(|field| field.name == "profile")
        .unwrap();

    assert!(matches!(&profile_field.typ, TypeNode::Struct(_)));
}

#[test]
fn generates_freezed_dart_with_json_keys() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("stride_api_codegen_dart_{unique}"));
    fs::create_dir_all(dir.join("type")).unwrap();
    fs::write(
        dir.join("type").join("stride.ssh.host.schema.json"),
        r#"{
            "title": "SSH Host",
            "type": "object",
            "properties": {
                "hostname": { "type": "string" },
                "key": { "$ref": "https://example.com/type/stride.ssh.key.public.schema.json" }
            },
            "required": ["hostname", "key"]
        }"#,
    )
    .unwrap();

    let output = generate_dart(&dir).unwrap();

    assert!(output.contains("import 'package:freezed_annotation/freezed_annotation.dart';"));
    assert!(output.contains("@freezed"));
    assert!(output.contains("@JsonKey(name: 'hostname')"));
    assert!(output.contains("@JsonKey(name: 'key')"));
}

#[test]
fn generates_dart_docs_for_types_fields_and_enum_variants() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("stride_api_codegen_dart_docs_{unique}"));
    fs::create_dir_all(dir.join("type")).unwrap();
    fs::write(
        dir.join("type").join("stride.user.schema.json"),
        r#"{
            "title": "User",
            "description": "A user account.",
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The user's display name."
                },
                "status": {
                    "type": "string",
                    "description": "The user's current status.",
                    "enum": ["active", "idle"]
                }
            },
            "required": ["name", "status"]
        }"#,
    )
    .unwrap();

    let output = generate_dart(&dir).unwrap();

    assert!(output.contains("/// A user account."));
    assert!(output.contains("/// The user's display name."));
    assert!(output.contains("/// The user's current status."));
    assert!(output.contains("/// active"));
    assert!(output.contains("/// idle"));
}

#[test]
fn generates_dart_enum_variants_in_camel_case() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("stride_api_codegen_dart_enum_variant_{unique}"));
    fs::create_dir_all(dir.join("type")).unwrap();
    fs::write(
        dir.join("type").join("stride.key.type.schema.json"),
        r#"{
            "title": "Key Type",
            "type": "string",
            "enum": ["ssh-rsa", "ssh-ed25519"]
        }"#,
    )
    .unwrap();

    let output = generate_dart(&dir).unwrap();

    assert!(output.contains("@JsonValue('ssh-rsa') sshRsa"));
    assert!(output.contains("@JsonValue('ssh-ed25519') sshEd25519"));
    assert!(!output.contains("sshrsa"));
    assert!(!output.contains("sshed25519"));
}

#[test]
fn generates_uuid_as_uuid_value_in_dart() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("stride_api_codegen_uuid_dart_{unique}"));
    fs::create_dir_all(dir.join("type")).unwrap();
    fs::write(
        dir.join("type").join("stride.repository.schema.json"),
        r#"{
            "title": "Repository",
            "type": "object",
            "properties": {
                "uuid": { "type": "string", "format": "uuid" },
                "name": { "type": "string" }
            },
            "required": ["uuid", "name"]
        }"#,
    )
    .unwrap();

    let output = generate_dart(&dir).unwrap();

    assert!(output.contains("import 'package:uuid/uuid.dart';"));
    assert!(output.contains("UuidValue uuid"));
    assert!(output.contains("@UuidValueJsonConverter()"));
    assert!(!output.contains("String uuid"));
}
