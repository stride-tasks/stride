use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use stride_api_codegen::{parse, Node, TypeNode};

use crate::dsl::{render_template, TemplateContext, TemplateValue};

const PAGE_TEMPLATE: &str = include_str!("../resources/page.html");
const HOME_TEMPLATE: &str = include_str!("../resources/home.html");
const THEME_SCRIPT: &str = include_str!("../resources/theme.js");
const SECTION_TEMPLATE: &str = include_str!("../resources/section.html");
const SUB_SECTION_TEMPLATE: &str = include_str!("../resources/subsection.html");
const TABLE_TEMPLATE: &str = include_str!("../resources/table.html");

#[derive(Debug, Clone)]
struct PropertyRow {
    name: String,
    type_name: String,
    required: bool,
    description: String,
}

#[derive(Debug, Clone)]
struct SchemaDoc {
    title: String,
    id: String,
    description: Option<String>,
    group: String,
    properties: Option<Vec<PropertyRow>>,
    sub_sections: Vec<SchemaDoc>,
}

pub fn generate_all_html(api_dir: &Path, output_dir: &Path) -> Result<()> {
    let nodes = parse(api_dir)?;
    let docs = build_docs_from_nodes(&nodes);

    let api_html = render_page(&docs);
    let home_html = render_home_page();

    fs::create_dir_all(output_dir)?;
    fs::write(output_dir.join("index.html"), home_html)?;
    fs::write(output_dir.join("api.html"), api_html)?;
    fs::write(output_dir.join("theme.js"), THEME_SCRIPT)?;

    Ok(())
}

fn build_docs_from_nodes(nodes: &[Node]) -> Vec<SchemaDoc> {
    let struct_by_name = nodes
        .iter()
        .filter_map(|node| match node {
            Node::Type(TypeNode::Struct(node)) => Some((node.name.clone(), node)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    let type_titles = nodes
        .iter()
        .filter_map(|node| match node {
            Node::Type(TypeNode::Struct(node)) => {
                Some((node.name.clone(), schema_title_for_struct(node)))
            }
            Node::Type(TypeNode::Enum(node)) => {
                Some((node.name.clone(), schema_title_for_enum(node)))
            }
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let mut docs = Vec::new();
    for node in nodes {
        match node {
            Node::Type(TypeNode::Struct(node)) => {
                let doc = struct_doc(node, &type_titles);
                docs.push(doc);
            }
            Node::Type(TypeNode::Enum(node)) => {
                let doc = enum_doc(node);
                docs.push(doc);
            }
            Node::Method(node) => {
                let doc = method_doc(node, &struct_by_name, &type_titles);
                docs.push(doc);
            }
            Node::Notification(node) => {
                let mut doc = notification_doc(node, &struct_by_name, &type_titles);
                doc.sub_sections = nested_sections_for_type_node(
                    &node.params,
                    &struct_by_name,
                    &type_titles,
                    Some(node.name.as_str()),
                );
                docs.push(doc);
            }
            Node::Type(_) => {}
        }
    }

    docs.sort_by(|a, b| {
        doc_group_sort_key(&a.group)
            .cmp(&doc_group_sort_key(&b.group))
            .then_with(|| a.title.cmp(&b.title))
    });
    docs
}

fn struct_doc(
    node: &stride_api_codegen::StructNode,
    type_titles: &HashMap<String, String>,
) -> SchemaDoc {
    let title = node
        .doc
        .first()
        .cloned()
        .unwrap_or_else(|| node.name.clone());
    let description = collect_doc_description(&node.doc);
    let properties = node
        .fields
        .iter()
        .map(|field| PropertyRow {
            name: field.name.clone(),
            type_name: type_node_to_string(&field.typ, type_titles),
            required: field.required,
            description: field
                .description
                .clone()
                .unwrap_or_else(|| field.name.clone()),
        })
        .collect::<Vec<_>>();

    SchemaDoc {
        title: normalize_title(&title),
        id: node.id.clone(),
        description,
        group: "type".to_string(),
        properties: Some(properties),
        sub_sections: Vec::new(),
    }
}

fn enum_doc(node: &stride_api_codegen::EnumNode) -> SchemaDoc {
    let title = node
        .doc
        .first()
        .cloned()
        .unwrap_or_else(|| node.name.clone());
    let properties = node
        .variants
        .iter()
        .map(|variant| PropertyRow {
            name: variant.value.clone(),
            type_name: "enum".to_string(),
            required: true,
            description: variant
                .description
                .clone()
                .unwrap_or_else(|| variant.value.clone()),
        })
        .collect::<Vec<_>>();

    SchemaDoc {
        title: normalize_title(&title),
        id: node.id.clone(),
        description: node.description.clone(),
        group: "type".to_string(),
        properties: Some(properties),
        sub_sections: Vec::new(),
    }
}

fn method_doc(
    node: &stride_api_codegen::MethodNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
) -> SchemaDoc {
    let mut doc = SchemaDoc {
        title: normalize_title(&node.method_name),
        id: node.id.clone(),
        description: node.description.clone(),
        group: "method".to_string(),
        properties: None,
        sub_sections: Vec::new(),
    };

    let params_properties = method_properties_for_type_node(
        &node.params,
        "Parameters",
        "Parameters for this method.",
        struct_by_name,
        type_titles,
    );
    let params_sub_sections =
        nested_sections_for_type_node(&node.params, struct_by_name, type_titles, None);
    doc.sub_sections.push(SchemaDoc {
        title: "Parameters".to_string(),
        id: String::new(),
        description: None,
        group: "type".to_string(),
        properties: Some(params_properties),
        sub_sections: params_sub_sections,
    });

    let result_properties = method_properties_for_type_node(
        &node.result,
        "Result",
        "Method result.",
        struct_by_name,
        type_titles,
    );
    let result_sub_sections =
        nested_sections_for_type_node(&node.result, struct_by_name, type_titles, None);
    doc.sub_sections.push(SchemaDoc {
        title: "Result".to_string(),
        id: String::new(),
        description: None,
        group: "type".to_string(),
        properties: Some(result_properties),
        sub_sections: result_sub_sections,
    });

    doc
}

fn method_properties_for_type_node(
    type_node: &TypeNode,
    title: &str,
    description: &str,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
) -> Vec<PropertyRow> {
    properties_for_type_node(type_node, struct_by_name, type_titles).unwrap_or_else(|| {
        match type_node {
            TypeNode::Unit | TypeNode::Any => Vec::new(),
            _ => vec![PropertyRow {
                name: if title == "Result" {
                    "result".to_string()
                } else {
                    "value".to_string()
                },
                type_name: type_node_to_string(type_node, type_titles),
                required: true,
                description: description.to_string(),
            }],
        }
    })
}

fn notification_doc(
    node: &stride_api_codegen::NotificationNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
) -> SchemaDoc {
    let properties =
        properties_for_type_node(&node.params, struct_by_name, type_titles).unwrap_or_default();

    SchemaDoc {
        title: normalize_title(&node.method_name),
        id: node.id.clone(),
        description: node.description.clone(),
        group: "notification".to_string(),
        properties: Some(properties),
        sub_sections: Vec::new(),
    }
}

fn collect_doc_description(doc: &[String]) -> Option<String> {
    let description = doc
        .iter()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    if description.is_empty() {
        None
    } else {
        Some(description)
    }
}

fn nested_sections_for_type_node(
    type_node: &TypeNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
    self_name: Option<&str>,
) -> Vec<SchemaDoc> {
    let mut seen = HashSet::new();
    if let Some(self_name) = self_name {
        seen.insert(self_name.to_owned());
    }

    match type_node {
        TypeNode::Struct(node) => {
            seen.insert(node.name.clone());
            nested_sections_for_fields(node, struct_by_name, type_titles, &mut seen)
        }
        _ => {
            let mut sections = Vec::new();
            append_nested_type_node(
                type_node,
                struct_by_name,
                type_titles,
                &mut seen,
                &mut sections,
            );
            sections
        }
    }
}

fn nested_sections_for_fields(
    node: &stride_api_codegen::StructNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
    seen: &mut HashSet<String>,
) -> Vec<SchemaDoc> {
    let mut sections = Vec::new();
    for field in &node.fields {
        append_nested_type_node(&field.typ, struct_by_name, type_titles, seen, &mut sections);
    }
    sections
}

fn append_nested_type_node(
    type_node: &TypeNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
    seen: &mut HashSet<String>,
    sections: &mut Vec<SchemaDoc>,
) {
    match type_node {
        TypeNode::Struct(node) => {
            append_struct_section(node, struct_by_name, type_titles, seen, sections)
        }
        TypeNode::Enum(node) => {
            let doc = enum_doc(node);
            sections.push(doc);
        }
        TypeNode::Array(inner) => {
            append_nested_type_node(inner, struct_by_name, type_titles, seen, sections);
        }
        TypeNode::Reference(_) | TypeNode::Primitive(_) | TypeNode::Any | TypeNode::Unit => {}
    }
}

fn append_struct_section(
    node: &stride_api_codegen::StructNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
    seen: &mut HashSet<String>,
    sections: &mut Vec<SchemaDoc>,
) {
    if !seen.insert(node.name.clone()) {
        return;
    }

    let mut doc = struct_doc(node, type_titles);
    doc.sub_sections = nested_sections_for_fields(node, struct_by_name, type_titles, seen);
    sections.push(doc);
}

fn properties_for_type_node(
    type_node: &TypeNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    type_titles: &HashMap<String, String>,
) -> Option<Vec<PropertyRow>> {
    let node = match type_node {
        TypeNode::Struct(node) => Some(node),
        TypeNode::Reference(name) => struct_by_name.get(name).copied(),
        TypeNode::Enum(_)
        | TypeNode::Primitive(_)
        | TypeNode::Array(_)
        | TypeNode::Any
        | TypeNode::Unit => None,
    }?;

    Some(
        node.fields
            .iter()
            .map(|field| PropertyRow {
                name: field.name.clone(),
                type_name: type_node_to_string(&field.typ, type_titles),
                required: field.required,
                description: field
                    .description
                    .clone()
                    .unwrap_or_else(|| field.name.clone()),
            })
            .collect(),
    )
}

fn type_node_to_string(
    type_node: &stride_api_codegen::TypeNode,
    type_titles: &HashMap<String, String>,
) -> String {
    match type_node {
        TypeNode::Struct(node) => schema_title_for_struct(node),
        TypeNode::Enum(node) => schema_title_for_enum(node),
        TypeNode::Primitive(kind) => match kind {
            stride_api_codegen::PrimitiveType::String => "string".to_string(),
            stride_api_codegen::PrimitiveType::Uuid => "uuid".to_string(),
            stride_api_codegen::PrimitiveType::Integer => "integer".to_string(),
            stride_api_codegen::PrimitiveType::Number => "number".to_string(),
            stride_api_codegen::PrimitiveType::Boolean => "boolean".to_string(),
        },
        TypeNode::Array(inner) => format!("{}[]", type_node_to_string(inner, type_titles)),
        TypeNode::Reference(name) => type_titles
            .get(name)
            .cloned()
            .unwrap_or_else(|| normalize_title(name)),
        TypeNode::Any => "any".to_string(),
        TypeNode::Unit => "unit".to_string(),
    }
}

fn schema_title_for_struct(node: &stride_api_codegen::StructNode) -> String {
    let title = node
        .doc
        .first()
        .cloned()
        .unwrap_or_else(|| node.name.clone());
    normalize_title(&title)
}

fn schema_title_for_enum(node: &stride_api_codegen::EnumNode) -> String {
    let title = node
        .doc
        .first()
        .cloned()
        .unwrap_or_else(|| node.name.clone());
    normalize_title(&title)
}

fn normalize_title(value: &str) -> String {
    let trimmed = value.trim();
    let sans_suffix = trimmed
        .strip_suffix(".schema.json")
        .or_else(|| trimmed.strip_suffix(".json"))
        .or_else(|| trimmed.strip_suffix(".schema"))
        .unwrap_or(trimmed);

    let normalized = sans_suffix
        .split_whitespace()
        .filter(|part| {
            let part = part.trim();
            if part.is_empty() {
                return false;
            }
            !matches!(part.to_ascii_lowercase().as_str(), |"schema"| "json")
        })
        .collect::<Vec<_>>()
        .join(".");

    if normalized.is_empty() {
        sans_suffix.to_string()
    } else {
        normalized
    }
}

fn doc_group_sort_key(group: &str) -> usize {
    match group {
        "method" => 0,
        "notification" => 1,
        "type" => 2,
        _ => 3,
    }
}

fn slugify(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    lower
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn group_badge_class(group: &str) -> String {
    match group {
        "method" => {
            "inline-flex items-center rounded-full border border-emerald-200 bg-emerald-100 px-2 py-1 text-[10px] font-bold uppercase tracking-[0.18em] text-emerald-700 dark:border-emerald-800 dark:bg-emerald-950/60 dark:text-emerald-300".to_string()
        }
        "notification" => {
            "inline-flex items-center rounded-full border border-violet-200 bg-violet-100 px-2 py-1 text-[10px] font-bold uppercase tracking-[0.18em] text-violet-700 dark:border-violet-800 dark:bg-violet-950/60 dark:text-violet-300".to_string()
        }
        _ => {
            "inline-flex items-center rounded-full border border-emerald-200 bg-emerald-100 px-2 py-1 text-[10px] font-bold uppercase tracking-[0.18em] text-emerald-700 dark:border-emerald-800 dark:bg-emerald-950/60 dark:text-emerald-300".to_string()
        }
    }
}

fn render_home_page() -> String {
    HOME_TEMPLATE
        .replace("@PAGE_TITLE@", "Stride | Task management with git")
        .replace("@SITE_NAME@", "Stride")
        .replace("@HERO_TITLE@", "Task management that follows your repo.")
        .replace(
            "@HERO_TEXT@",
            "Stride keeps tasks, project context, and git-backed history in one place so work stays local-first, searchable, and easy to sync.",
        )
        .replace("@PRIMARY_CTA@", "Open API reference")
        .replace("@SECONDARY_CTA@", "Download")
}

fn render_page(docs: &[SchemaDoc]) -> String {
    let type_anchor_lookup = build_type_anchor_lookup(docs);

    let mut toc = String::new();
    for group in ["method", "notification", "type"] {
        let group_docs = docs
            .iter()
            .filter(|doc| doc.group == group)
            .collect::<Vec<_>>();
        if group_docs.is_empty() {
            continue;
        }

        let group_count = group_docs.len();
        let mut group_items = String::new();
        for doc in group_docs {
            let anchor = slugify(&doc.title);
            group_items.push_str(&format!(
                "<li class=\"mb-1\" data-toc-item=\"{anchor}\"><a href=\"#{anchor}\" data-toc-link=\"{anchor}\" class=\"text-emerald-700 hover:underline dark:text-emerald-400\">{}</a></li>",
                escape_html(&doc.title)
            ));
        }

        toc.push_str(&format!(
            "<li class=\"mb-3\" data-toc-group=\"{group}\"><div class=\"mb-2 flex items-center gap-2 text-[10px] font-semibold uppercase tracking-[0.2em] text-slate-500 dark:text-slate-400\"><span>{}</span><span data-toc-group-count class=\"inline-flex min-w-[1.5rem] items-center justify-center rounded-full bg-slate-100 px-1.5 py-0.5 text-[9px] font-bold text-slate-700 dark:bg-slate-800 dark:text-slate-200\">{}</span></div><ul class=\"space-y-1 pl-3\">{}</ul></li>",
            escape_html(group),
            group_count,
            group_items,
        ));
    }

    let mut body = String::new();
    for doc in docs {
        let section_id = slugify(&doc.title);
        let nested_search = doc
            .sub_sections
            .iter()
            .map(|sub| {
                format!(
                    "{} {} {}",
                    sub.title,
                    sub.group,
                    sub.description.as_deref().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join(" ");

        let id_html = if doc.id.is_empty() {
            String::new()
        } else {
            format!(
                "<a href=\"{}\" class=\"break-all text-emerald-700 hover:underline dark:text-emerald-400\">schema</a>",
                escape_html(&doc.id)
            )
        };

        let mut search_text = vec![
            doc.title.clone(),
            doc.group.clone(),
            doc.id.clone(),
            nested_search,
        ];

        if let Some(description) = &doc.description {
            search_text.push(description.clone());
        }

        let table_html = if let Some(properties) = &doc.properties {
            search_text.push(
                properties
                    .iter()
                    .map(|row| {
                        format!(
                            "{} {} {} {}",
                            row.name, row.type_name, row.description, row.required
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
            );

            render_table_html(Some(properties), &type_anchor_lookup)
        } else {
            String::new()
        };

        let sub_sections_html =
            render_sub_sections(&doc.sub_sections, &section_id, &type_anchor_lookup);

        let search_text = search_text.join(" ");
        let mut section_context = TemplateContext::new();
        section_context.insert("SECTION_ID", TemplateValue::Html(section_id.clone()));
        section_context.insert(
            "SEARCH_TEXT",
            TemplateValue::Html(escape_html(&search_text)),
        );
        section_context.insert("GROUP", TemplateValue::Html(escape_html(&doc.group)));
        section_context.insert(
            "GROUP_CLASS",
            TemplateValue::Html(group_badge_class(&doc.group)),
        );
        section_context.insert("TITLE", TemplateValue::Html(escape_html(&doc.title)));
        section_context.insert("ID", TemplateValue::Html(id_html.clone()));
        if let Some(description) = &doc.description {
            section_context.insert("DESCRIPTION", TemplateValue::Html(escape_html(description)));
        }
        section_context.insert("TABLE", TemplateValue::Html(table_html.clone()));
        section_context.insert(
            "SUB_SECTIONS_HTML",
            TemplateValue::Html(sub_sections_html.clone()),
        );

        let section_html = render_template(SECTION_TEMPLATE, &section_context)
            .unwrap_or_else(|err| format!("<pre>template error: {err:?}</pre>"));
        body.push_str(&section_html);
    }

    PAGE_TEMPLATE
        .replace("@TOC@", &toc)
        .replace("@BODY@", &body)
}

fn build_type_anchor_lookup(docs: &[SchemaDoc]) -> HashMap<String, String> {
    let mut anchors = HashMap::new();
    for doc in docs {
        anchors.insert(doc.title.clone(), slugify(&doc.title));
    }
    anchors
}

fn render_table_html(
    rows: Option<&[PropertyRow]>,
    type_anchor_lookup: &HashMap<String, String>,
) -> String {
    let rows = match rows {
        Some(r) => r,
        None => return String::new(),
    };
    if rows.is_empty() {
        return String::new();
    }

    let mut table_context = TemplateContext::new();
    let (headers, row_values) = if rows
        .iter()
        .all(|row| row.type_name == "enum" || row.type_name == "enum[]")
    {
        (
            vec!["Variant", "Description"],
            rows.iter()
                .map(|row| {
                    TemplateValue::Map(HashMap::from([(
                        "columns".to_string(),
                        TemplateValue::List(vec![
                            TemplateValue::Html(format!(
                                r#"<td class="px-3 py-3 font-mono text-slate-900 dark:text-slate-100">{name}</td>"#,
                                name = escape_html(&row.name),
                            )),
                            TemplateValue::Html(format!(
                                r#"<td class="px-3 py-3 text-slate-600 dark:text-slate-300">{description}</td>"#,
                                description = escape_html(&row.description),
                            )),
                        ]),
                    )]))
                })
                .collect::<Vec<_>>(),
        )
    } else {
        (
            vec!["Field", "Type", "Required", "Description"],
            render_rows(rows, type_anchor_lookup),
        )
    };

    table_context.insert(
        "HEADERS",
        TemplateValue::List(
            headers
                .into_iter()
                .map(|header| TemplateValue::Html(header.to_string()))
                .collect(),
        ),
    );
    table_context.insert("ROWS", TemplateValue::List(row_values));

    render_template(TABLE_TEMPLATE, &table_context)
        .unwrap_or_else(|err| format!("<pre>template error: {err:?}</pre>"))
}

fn render_rows(
    rows: &[PropertyRow],
    type_anchor_lookup: &HashMap<String, String>,
) -> Vec<TemplateValue> {
    rows.iter()
        .map(|row| {
            TemplateValue::Map(HashMap::from([(
                "columns".to_string(),
                TemplateValue::List(vec![
                    TemplateValue::Html(format!(
                        r#"<td class="px-3 py-3 font-mono text-slate-900 dark:text-slate-100">{name}</td>"#,
                        name = escape_html(&row.name),
                    )),
                    TemplateValue::Html(format!(
                        r#"<td class="px-3 py-3 text-slate-700 dark:text-slate-300">{type_name}</td>"#,
                        type_name = render_type_name_html(&row.type_name, type_anchor_lookup),
                    )),
                    TemplateValue::Html(format!(
                        r#"<td class="px-3 py-3 text-slate-700 dark:text-slate-300">{required}</td>"#,
                        required = if row.required { "yes" } else { "no" },
                    )),
                    TemplateValue::Html(format!(
                        r#"<td class="px-3 py-3 text-slate-600 dark:text-slate-300">{description}</td>"#,
                        description = escape_html(&row.description),
                    )),
                ]),
            )]))
        })
        .collect()
}

fn render_type_name_html(type_name: &str, type_anchor_lookup: &HashMap<String, String>) -> String {
    let trimmed = type_name.trim();
    let mut suffixes = Vec::new();
    let mut normalized = trimmed;
    while normalized.ends_with("[]") {
        suffixes.push("[]");
        normalized = &normalized[..normalized.len() - 2];
    }

    if let Some(anchor) = type_anchor_lookup.get(normalized) {
        let label = escape_html(normalized);
        let mut out = format!(
            "<a href=\"#{}\" class=\"break-all text-emerald-700 hover:underline dark:text-emerald-400\">{}</a>",
            escape_html(anchor),
            label,
        );
        for suffix in &suffixes {
            out.push_str(suffix);
        }
        return out;
    }

    let mut out = escape_html(normalized);
    for suffix in &suffixes {
        out.push_str(suffix);
    }
    out
}

fn render_sub_sections(
    sub_sections: &[SchemaDoc],
    parent_id: &str,
    type_anchor_lookup: &HashMap<String, String>,
) -> String {
    if sub_sections.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    for sub in sub_sections {
        let sub_id = format!("{parent_id}-{}", slugify(&sub.title));
        let inner = render_sub_sections(&sub.sub_sections, &sub_id, type_anchor_lookup);

        let table_html = render_table_html(sub.properties.as_deref(), type_anchor_lookup);
        let mut sub_context = TemplateContext::new();
        sub_context.insert("SUB_ID", TemplateValue::Html(sub_id.clone()));
        sub_context.insert("TITLE", TemplateValue::Html(escape_html(&sub.title)));
        sub_context.insert("GROUP", TemplateValue::Html(escape_html(&sub.group)));
        if let Some(description) = &sub.description {
            sub_context.insert("DESCRIPTION", TemplateValue::Html(escape_html(description)));
        }
        sub_context.insert("TABLE", TemplateValue::Html(table_html));
        sub_context.insert("INNER", TemplateValue::Html(inner));

        let sub_section_html = render_template(SUB_SECTION_TEMPLATE, &sub_context)
            .unwrap_or_else(|err| format!("<pre>template error: {err:?}</pre>"));
        out.push_str(&sub_section_html);
    }
    out
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
