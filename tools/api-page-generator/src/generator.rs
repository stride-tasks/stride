use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use stride_api_codegen::{parse, Node, TypeNode, TypeRef};

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
    description: String,
    group: String,
    properties: Vec<PropertyRow>,
    sub_sections: Vec<SchemaDoc>,
}

pub fn generate_all_html(api_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let nodes = parse(api_dir)?;
    let docs = build_docs_from_nodes(&nodes);

    let html = render_page(&docs);
    fs::create_dir_all(output_dir)?;
    fs::write(output_dir.join("index.html"), html)?;

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

    let nested_type_names = collect_embedded_type_names(nodes, &struct_by_name);
    let mut docs = Vec::new();
    for node in nodes {
        match node {
            Node::Type(TypeNode::Struct(node)) => {
                if nested_type_names.contains(&node.name) {
                    continue;
                }
                let mut doc = struct_doc(node);
                doc.sub_sections = nested_sections_for_struct(node, &struct_by_name);
                docs.push(doc);
            }
            Node::Type(TypeNode::Enum(node)) => {
                let doc = enum_doc(node);
                docs.push(doc);
            }
            Node::Method(node) => {
                let mut doc = method_doc(node, &struct_by_name);
                let params_sub_sections = match &node.params {
                    TypeNode::Struct(inner) => nested_sections_for_struct(inner, &struct_by_name),
                    TypeNode::Enum(_) => Vec::new(),
                    TypeNode::Ref(type_ref) => nested_sections_for_type_ref(
                        type_ref,
                        &struct_by_name,
                        Some(node.name.as_str()),
                    ),
                };
                let result_sub_sections = match &node.result {
                    TypeNode::Struct(inner) => nested_sections_for_struct(inner, &struct_by_name),
                    TypeNode::Enum(_) => Vec::new(),
                    TypeNode::Ref(type_ref) => nested_sections_for_type_ref(
                        type_ref,
                        &struct_by_name,
                        Some(node.name.as_str()),
                    ),
                };
                doc.sub_sections = params_sub_sections
                    .into_iter()
                    .chain(result_sub_sections)
                    .collect();
                docs.push(doc);
            }
            Node::Notification(node) => {
                let mut doc = notification_doc(node, &struct_by_name);
                doc.sub_sections = match &node.params {
                    TypeNode::Struct(inner) => nested_sections_for_struct(inner, &struct_by_name),
                    TypeNode::Enum(_) => Vec::new(),
                    TypeNode::Ref(type_ref) => nested_sections_for_type_ref(
                        type_ref,
                        &struct_by_name,
                        Some(node.name.as_str()),
                    ),
                };
                docs.push(doc);
            }
            Node::Type(TypeNode::Ref(_)) => {}
        }
    }

    docs.sort_by(|a, b| a.title.cmp(&b.title));
    docs
}

fn struct_doc(node: &stride_api_codegen::StructNode) -> SchemaDoc {
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
            type_name: type_ref_to_string(&field.type_ref),
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
        properties,
        sub_sections: Vec::new(),
    }
}

fn enum_doc(node: &stride_api_codegen::EnumNode) -> SchemaDoc {
    let properties = node
        .variants
        .iter()
        .map(|variant| PropertyRow {
            name: variant.name.clone(),
            type_name: "enum".to_string(),
            required: true,
            description: variant.value.clone(),
        })
        .collect::<Vec<_>>();

    SchemaDoc {
        title: normalize_title(
            &node
                .description
                .clone()
                .unwrap_or_else(|| node.name.clone()),
        ),
        id: node.id.clone(),
        description: node.description.clone().unwrap_or_default(),
        group: "type".to_string(),
        properties,
        sub_sections: Vec::new(),
    }
}

fn method_doc(
    node: &stride_api_codegen::MethodNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> SchemaDoc {
    let properties = struct_by_name
        .get(&node.name)
        .map(|target| {
            target
                .fields
                .iter()
                .map(|field| PropertyRow {
                    name: field.name.clone(),
                    type_name: type_ref_to_string(&field.type_ref),
                    required: field.required,
                    description: field
                        .description
                        .clone()
                        .unwrap_or_else(|| field.name.clone()),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            vec![PropertyRow {
                name: "result".to_string(),
                type_name: type_node_to_string(&node.result),
                required: true,
                description: "Method result".to_string(),
            }]
        });

    SchemaDoc {
        title: normalize_title(&node.method_name),
        id: node.id.clone(),
        description: node.description.clone().unwrap_or_default(),
        group: "method".to_string(),
        properties,
        sub_sections: Vec::new(),
    }
}

fn notification_doc(
    node: &stride_api_codegen::NotificationNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> SchemaDoc {
    let properties = struct_by_name
        .get(&node.name)
        .map(|target| {
            target
                .fields
                .iter()
                .map(|field| PropertyRow {
                    name: field.name.clone(),
                    type_name: type_ref_to_string(&field.type_ref),
                    required: field.required,
                    description: field
                        .description
                        .clone()
                        .unwrap_or_else(|| field.name.clone()),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    SchemaDoc {
        title: normalize_title(&node.method_name),
        id: node.id.clone(),
        description: node.description.clone().unwrap_or_default(),
        group: "notification".to_string(),
        properties,
        sub_sections: Vec::new(),
    }
}

fn collect_doc_description(doc: &[String]) -> String {
    doc.iter()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

fn collect_embedded_type_names(
    nodes: &[Node],
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for node in nodes {
        let self_name = match node {
            Node::Type(TypeNode::Struct(node)) => Some(node.name.as_str()),
            Node::Method(node) => Some(node.name.as_str()),
            Node::Notification(node) => Some(node.name.as_str()),
            Node::Type(TypeNode::Enum(_)) | Node::Type(TypeNode::Ref(_)) => None,
        };

        let mut set = match node {
            Node::Type(TypeNode::Struct(node)) => {
                collect_inline_object_names_from_fields(&node.fields, struct_by_name)
            }
            Node::Method(node) => {
                let mut set =
                    collect_inline_object_names_from_type_node(&node.params, struct_by_name);
                set.extend(collect_inline_object_names_from_type_node(
                    &node.result,
                    struct_by_name,
                ));
                set
            }
            Node::Notification(node) => {
                collect_inline_object_names_from_type_node(&node.params, struct_by_name)
            }
            Node::Type(TypeNode::Enum(_)) | Node::Type(TypeNode::Ref(_)) => HashSet::new(),
        };

        if let Some(self_name) = self_name {
            set.remove(self_name);
        }
        names.extend(set);
    }
    names
}

fn nested_sections_for_struct(
    node: &stride_api_codegen::StructNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> Vec<SchemaDoc> {
    nested_sections_for_type_ref_collection(
        &node
            .fields
            .iter()
            .map(|field| &field.type_ref)
            .collect::<Vec<_>>(),
        struct_by_name,
        Some(node.name.as_str()),
    )
}

fn nested_sections_for_type_ref(
    type_ref: &TypeRef,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    self_name: Option<&str>,
) -> Vec<SchemaDoc> {
    nested_sections_for_type_ref_collection(&[type_ref], struct_by_name, self_name)
}

fn nested_sections_for_type_ref_collection(
    type_refs: &[&TypeRef],
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
    self_name: Option<&str>,
) -> Vec<SchemaDoc> {
    let mut sections = Vec::new();
    let mut seen = HashSet::new();
    for type_ref in type_refs {
        let names = collect_inline_object_names_from_type_ref(type_ref, struct_by_name);
        for name in names {
            if self_name == Some(name.as_str()) || !seen.insert(name.clone()) {
                continue;
            }
            if let Some(target) = struct_by_name.get(&name) {
                let mut doc = struct_doc(target);
                doc.sub_sections = nested_sections_for_struct(target, struct_by_name);
                sections.push(doc);
            }
        }
    }
    sections
}

fn collect_inline_object_names_from_type_node(
    node: &TypeNode,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> HashSet<String> {
    match node {
        TypeNode::Struct(node) => {
            collect_inline_object_names_from_fields(&node.fields, struct_by_name)
        }
        TypeNode::Enum(_) => HashSet::new(),
        TypeNode::Ref(type_ref) => {
            collect_inline_object_names_from_type_ref(type_ref, struct_by_name)
        }
    }
}

fn collect_inline_object_names_from_fields(
    fields: &[stride_api_codegen::FieldNode],
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for field in fields {
        names.extend(collect_inline_object_names_from_type_ref(
            &field.type_ref,
            struct_by_name,
        ));
    }
    names
}

fn collect_inline_object_names_from_type_ref(
    type_ref: &TypeRef,
    struct_by_name: &HashMap<String, &stride_api_codegen::StructNode>,
) -> HashSet<String> {
    let mut names = HashSet::new();
    match type_ref {
        TypeRef::Array(inner) | TypeRef::Map(inner) => {
            names.extend(collect_inline_object_names_from_type_ref(
                inner,
                struct_by_name,
            ));
        }
        TypeRef::Object(name) | TypeRef::Enum(name) | TypeRef::Reference(name) => {
            if struct_by_name.contains_key(name) {
                names.insert(name.clone());
            }
        }
        TypeRef::Primitive(_) | TypeRef::Json | TypeRef::Unit => {}
    }
    names
}

fn type_node_to_string(type_node: &stride_api_codegen::TypeNode) -> String {
    match type_node {
        stride_api_codegen::TypeNode::Struct(node) => node.name.clone(),
        stride_api_codegen::TypeNode::Enum(node) => node.name.clone(),
        stride_api_codegen::TypeNode::Ref(type_ref) => type_ref_to_string(type_ref),
    }
}

fn type_ref_to_string(type_ref: &TypeRef) -> String {
    match type_ref {
        TypeRef::Primitive(kind) => match kind {
            stride_api_codegen::PrimitiveType::String => "string".to_string(),
            stride_api_codegen::PrimitiveType::Uuid => "uuid".to_string(),
            stride_api_codegen::PrimitiveType::Integer => "integer".to_string(),
            stride_api_codegen::PrimitiveType::Number => "number".to_string(),
            stride_api_codegen::PrimitiveType::Boolean => "boolean".to_string(),
        },
        TypeRef::Array(inner) => format!("array<{}>", type_ref_to_string(inner)),
        TypeRef::Map(inner) => format!("map<{}>", type_ref_to_string(inner)),
        TypeRef::Object(name) | TypeRef::Enum(name) | TypeRef::Reference(name) => name.clone(),
        TypeRef::Json => "json".to_string(),
        TypeRef::Unit => "unit".to_string(),
    }
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
            !matches!(
                part.to_ascii_lowercase().as_str(),
                "stride" | "schema" | "json"
            )
        })
        .collect::<Vec<_>>()
        .join(".");

    if normalized.is_empty() {
        sans_suffix.to_string()
    } else {
        normalized
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

fn render_page(docs: &[SchemaDoc]) -> String {
    let mut toc = String::new();
    for doc in docs {
        let anchor = slugify(&doc.title);
        toc.push_str(&format!(
            "<li class=\"mb-2\" data-toc-item=\"{anchor}\"><a href=\"#{anchor}\" data-toc-link=\"{anchor}\" class=\"text-sky-700 hover:underline\">{}</a></li>",
            escape_html(&doc.title)
        ));
    }

    let mut body = String::new();
    for doc in docs {
        let section_id = slugify(&doc.title);
        let nested_search = doc
            .sub_sections
            .iter()
            .map(|sub| format!("{} {} {}", sub.title, sub.group, sub.description))
            .collect::<Vec<_>>()
            .join(" ");
        let search_text = vec![
            doc.title.clone(),
            doc.group.clone(),
            doc.id.clone(),
            doc.description.clone(),
            nested_search,
            doc.properties
                .iter()
                .map(|row| {
                    format!(
                        "{} {} {} {}",
                        row.name, row.type_name, row.description, row.required
                    )
                })
                .collect::<Vec<_>>()
                .join(" "),
        ]
        .join(" ");

        let id_html = if doc.id.is_empty() {
            "<span class=\"text-slate-500 dark:text-slate-400\">No schema id</span>".to_string()
        } else {
            format!(
                "<a href=\"{}\" class=\"break-all text-sky-700 hover:underline dark:text-sky-400\">{}</a>",
                escape_html(&doc.id),
                escape_html(&doc.id)
            )
        };

        let sub_sections_html = render_sub_sections(&doc.sub_sections, &section_id);

        body.push_str(&format!(
            r#"
            <section id="{section_id}" data-section-id="{section_id}" data-search-text="{search_text}" class="mb-8 rounded-xl border border-slate-200 bg-white p-6 shadow-sm transition dark:border-slate-700 dark:bg-slate-900">
              <div class="mb-4 flex flex-col gap-2 border-b border-slate-200 pb-4 md:flex-row md:items-end md:justify-between dark:border-slate-700">
                <div>
                  <p class="text-xs font-semibold uppercase tracking-[0.2em] text-slate-500 dark:text-slate-400">{group}</p>
                  <h2 class="text-2xl font-bold text-slate-900 dark:text-slate-100">{title}</h2>
                </div>
                <div class="text-sm text-slate-600 dark:text-slate-300">{id}</div>
              </div>
              <div class="mb-6">
                <p class="text-sm text-slate-600 dark:text-slate-300">{description}</p>
              </div>
              <div class="overflow-hidden rounded-lg border border-slate-200 dark:border-slate-700">
                <table class="min-w-full divide-y divide-slate-200 text-left text-sm dark:divide-slate-700">
                  <thead class="bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-200">
                    <tr>
                      <th class="px-3 py-2 font-semibold">Field</th>
                      <th class="px-3 py-2 font-semibold">Type</th>
                      <th class="px-3 py-2 font-semibold">Required</th>
                      <th class="px-3 py-2 font-semibold">Description</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-slate-200 bg-white dark:divide-slate-700 dark:bg-slate-900">
                    {rows}
                  </tbody>
                </table>
              </div>
              {sub_sections_html}
            </section>
            "#,
            group = escape_html(&doc.group),
            title = escape_html(&doc.title),
            id = id_html,
            description = escape_html(&doc.description),
            rows = render_rows(&doc.properties),
            search_text = escape_html(&search_text),
            sub_sections_html = sub_sections_html,
        ));
    }

    format!(
        r#"<!doctype html>
<html lang="en" class="h-full">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Stride API</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {{ darkMode: 'class' }};
  </script>
</head>
<body class="h-full bg-slate-100 text-slate-800 antialiased transition-colors dark:bg-slate-950 dark:text-slate-100">
  <div class="mx-auto max-w-7xl px-4 py-8 lg:px-8">
    <header class="mb-6">
      <div class="flex flex-col gap-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-700 dark:bg-slate-900 md:flex-row md:items-center md:justify-between">
        <div>
          <p class="text-sm font-semibold uppercase tracking-[0.3em] text-sky-700 dark:text-sky-400">Stride</p>
          <h1 class="mt-2 text-3xl font-bold text-slate-900 dark:text-slate-100">API schema reference</h1>
        </div>

        <div class="flex w-full max-w-xl items-center gap-3 md:justify-end">
          <label class="relative block w-full">
            <span class="sr-only">Search schemas</span>
            <input id="schema-search" type="search" placeholder="Search schemas, fields, and IDs..." class="w-full rounded-lg border border-slate-200 bg-slate-50 px-4 py-2.5 text-sm text-slate-700 placeholder:text-slate-400 shadow-sm outline-none transition focus:border-sky-500 focus:ring-2 focus:ring-sky-200 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-100 dark:placeholder:text-slate-400 dark:focus:ring-sky-500/20" />
          </label>
          <button id="theme-toggle" type="button" aria-label="Toggle color mode" class="inline-flex items-center gap-2 rounded-lg border border-slate-200 bg-slate-100 px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-200 dark:border-slate-600 dark:bg-slate-800 dark:text-slate-200 dark:hover:bg-slate-700">
            <span id="theme-icon" class="inline-flex h-4 w-4 items-center justify-center" aria-hidden="true"></span>
          </button>
        </div>
      </div>
    </header>

    <div class="grid gap-8 lg:grid-cols-[280px_minmax(0,1fr)]">
      <aside class="h-fit rounded-xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-700 dark:bg-slate-900">
        <div class="mb-4 flex items-center justify-between">
          <h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">Table of contents</h2>
          <span id="search-count" class="rounded-full bg-slate-100 px-2 py-1 text-xs font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">{count}</span>
        </div>
        <ul class="text-sm text-slate-600 dark:text-slate-300">{toc}</ul>
      </aside>

      <main>
        <div id="empty-state" hidden class="rounded-xl border border-dashed border-slate-300 bg-white p-8 text-center text-slate-500 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-300">
          No schemas match your search.
        </div>
        {body}
      </main>
    </div>
  </div>

  <script>
    const root = document.documentElement;
    const searchInput = document.getElementById('schema-search');
    const themeToggle = document.getElementById('theme-toggle');
    const themeIcon = document.getElementById('theme-icon');
    const sections = Array.from(document.querySelectorAll('[data-section-id]'));
    const tocLinks = Array.from(document.querySelectorAll('[data-toc-link]'));
    const emptyState = document.getElementById('empty-state');
    const searchCount = document.getElementById('search-count');

    const sunIcon = '<svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="h-4 w-4"><path d="M10 2.5a.75.75 0 0 1 .75.75v1.25a.75.75 0 0 1-1.5 0V3.25A.75.75 0 0 1 10 2.5Zm0 12.5a.75.75 0 0 1 .75.75v1.25a.75.75 0 0 1-1.5 0v-1.25A.75.75 0 0 1 10 15Zm6.5-5.5a.75.75 0 0 1 0 1.5h-1.25a.75.75 0 0 1 0-1.5H16.5Zm-12.5 0a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1 0-1.5H4Zm9.95-4.05a.75.75 0 0 1 1.06 0l.88.88a.75.75 0 1 1-1.06 1.06l-.88-.88a.75.75 0 0 1 0-1.06Zm-9.9 9.9a.75.75 0 0 1 1.06 0l.88.88a.75.75 0 0 1-1.06 1.06l-.88-.88a.75.75 0 0 1 0-1.06Zm0-9.9a.75.75 0 0 1 0 1.06l-.88.88A.75.75 0 0 1 3.12 4.13l.88-.88a.75.75 0 0 1 1.06 0Zm9.9 9.9a.75.75 0 0 1 0 1.06l-.88.88a.75.75 0 1 1-1.06-1.06l.88-.88a.75.75 0 0 1 1.06 0ZM10 6.25A3.75 3.75 0 1 1 10 13.75A3.75 3.75 0 0 1 10 6.25Z"/></svg>';
    const moonIcon = '<svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="h-4 w-4"><path d="M14.88 12.8A6.5 6.5 0 0 1 7.2 5.12A6.5 6.5 0 1 0 14.88 12.8Z"/></svg>';

    const setTheme = (theme) => {{
      const isDark = theme === 'dark';
      root.classList.toggle('dark', isDark);
      localStorage.setItem('stride-theme', theme);
      themeIcon.innerHTML = isDark ? sunIcon : moonIcon;
    }};

    const preferredTheme = localStorage.getItem('stride-theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    setTheme(preferredTheme);

    themeToggle.addEventListener('click', () => {{
      const nextTheme = root.classList.contains('dark') ? 'light' : 'dark';
      setTheme(nextTheme);
    }});

    const applySearch = () => {{
      const query = searchInput.value.trim().toLowerCase();
      let visibleCount = 0;

      sections.forEach((section) => {{
        const text = (section.dataset.searchText || '').toLowerCase();
        const matches = !query || text.includes(query);
        section.hidden = !matches;
        if (matches) visibleCount += 1;
      }});

      tocLinks.forEach((link) => {{
        const target = document.getElementById(link.dataset.tocLink);
        if (!target) return;
        link.parentElement.hidden = target.hidden;
      }});

      emptyState.hidden = visibleCount !== 0;
      searchCount.textContent = String(visibleCount);
    }};

    searchInput.addEventListener('input', applySearch);
    applySearch();
  </script>
</body>
</html>"#,
        count = docs.len(),
        toc = toc,
        body = body,
    )
}

fn render_rows(rows: &[PropertyRow]) -> String {
    if rows.is_empty() {
        return r#"<tr><td colspan="4" class="px-3 py-4 text-slate-500 dark:text-slate-400">No properties.</td></tr>"#
            .to_string();
    }

    let mut out = String::new();
    for row in rows {
        out.push_str(&format!(
            r#"<tr class="align-top">
                <td class="px-3 py-3 font-mono text-slate-900 dark:text-slate-100">{name}</td>
                <td class="px-3 py-3 text-slate-700 dark:text-slate-300">{type_name}</td>
                <td class="px-3 py-3 text-slate-700 dark:text-slate-300">{required}</td>
                <td class="px-3 py-3 text-slate-600 dark:text-slate-300">{description}</td>
            </tr>"#,
            name = escape_html(&row.name),
            type_name = escape_html(&row.type_name),
            required = if row.required { "yes" } else { "no" },
            description = escape_html(&row.description),
        ));
    }
    out
}

fn render_sub_sections(sub_sections: &[SchemaDoc], parent_id: &str) -> String {
    if sub_sections.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    for sub in sub_sections {
        let sub_id = format!("{parent_id}-{}", slugify(&sub.title));
        out.push_str(&format!(
            r#"
              <div class="mt-8 rounded-lg border border-slate-200 bg-slate-50 p-4 dark:border-slate-700 dark:bg-slate-800/50">
                <div class="mb-4 flex items-center justify-between gap-3">
                  <div>
                    <p class="text-[10px] font-semibold uppercase tracking-[0.2em] text-slate-500 dark:text-slate-400">Sub type</p>
                    <h3 id="{sub_id}" class="text-xl font-semibold text-slate-900 dark:text-slate-100">{title}</h3>
                  </div>
                  <span class="rounded-full bg-slate-200 px-2 py-1 text-xs font-medium text-slate-700 dark:bg-slate-700 dark:text-slate-200">{group}</span>
                </div>
                <div class="mb-4">
                  <p class="text-sm text-slate-600 dark:text-slate-300">{description}</p>
                </div>
                <div class="overflow-hidden rounded-lg border border-slate-200 dark:border-slate-700">
                  <table class="min-w-full divide-y divide-slate-200 text-left text-sm dark:divide-slate-700">
                    <thead class="bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-200">
                      <tr>
                        <th class="px-3 py-2 font-semibold">Field</th>
                        <th class="px-3 py-2 font-semibold">Type</th>
                        <th class="px-3 py-2 font-semibold">Required</th>
                        <th class="px-3 py-2 font-semibold">Description</th>
                      </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-200 bg-white dark:divide-slate-700 dark:bg-slate-900">
                      {rows}
                    </tbody>
                  </table>
                </div>
                {inner}
              </div>
            "#,
            title = escape_html(&sub.title),
            group = escape_html(&sub.group),
            description = escape_html(&sub.description),
            rows = render_rows(&sub.properties),
            inner = render_sub_sections(&sub.sub_sections, &sub_id),
        ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use stride_api_codegen::{
        FieldNode, Node, NotificationNode, PrimitiveType, StructNode, TypeNode, TypeRef,
    };

    #[test]
    fn hides_embedded_object_types_as_sub_sections() {
        let target = StructNode {
            id: "target-id".to_string(),
            name: "UserPromptTarget".to_string(),
            doc: vec!["UserPromptTarget".to_string(), "Target payload".to_string()],
            fields: vec![FieldNode {
                name: "method".to_string(),
                type_ref: TypeRef::Primitive(PrimitiveType::String),
                required: true,
                description: Some("The method name".to_string()),
            }],
        };

        let prompt = StructNode {
            id: "prompt-id".to_string(),
            name: "Prompt".to_string(),
            doc: vec!["Prompt".to_string(), "Prompt notification".to_string()],
            fields: vec![FieldNode {
                name: "target".to_string(),
                type_ref: TypeRef::Object("UserPromptTarget".to_string()),
                required: true,
                description: Some("The target payload".to_string()),
            }],
        };

        let nodes = vec![
            Node::Type(TypeNode::Struct(target)),
            Node::Type(TypeNode::Struct(prompt.clone())),
            Node::Notification(NotificationNode {
                id: "notification-id".to_string(),
                name: "Prompt".to_string(),
                method_name: "stride.user.prompt".to_string(),
                description: Some("Prompt context".to_string()),
                params: TypeNode::Ref(TypeRef::Object("Prompt".to_string())),
            }),
        ];

        let docs = build_docs_from_nodes(&nodes);
        let root = docs
            .iter()
            .find(|doc| {
                doc.sub_sections
                    .iter()
                    .any(|sub| sub.title == "UserPromptTarget")
            })
            .expect("embedded target should be attached to the owning schema section");

        assert_eq!(root.sub_sections.len(), 1);
        assert_eq!(root.sub_sections[0].title, "UserPromptTarget");
        assert!(docs.iter().all(|doc| doc.title != "UserPromptTarget"));
    }
}
