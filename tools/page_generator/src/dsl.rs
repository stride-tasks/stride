use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateValue {
    Html(String),
    #[allow(unused)]
    Bool(bool),
    List(Vec<TemplateValue>),
    Map(HashMap<String, TemplateValue>),
}

impl TemplateValue {
    #[allow(unused)]
    pub fn as_html(&self) -> Option<&str> {
        match self {
            TemplateValue::Html(value) => Some(value),
            TemplateValue::Bool(_) | TemplateValue::List(_) | TemplateValue::Map(_) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            TemplateValue::Html(value) => value.trim().is_empty(),
            TemplateValue::Bool(value) => !value,
            TemplateValue::List(value) => value.is_empty(),
            TemplateValue::Map(value) => value.is_empty(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TemplateContext {
    values: HashMap<String, TemplateValue>,
}

impl TemplateContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: TemplateValue,
    ) -> Option<TemplateValue> {
        self.values.insert(key.into(), value)
    }

    pub fn get(&self, key: &str) -> Option<&TemplateValue> {
        let key = key.trim();
        if key.is_empty() {
            return None;
        }

        let mut parts = Vec::new();
        let mut current = String::new();
        let mut chars = key.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    if !current.is_empty() {
                        parts.push(current);
                        current = String::new();
                    }
                }
                '[' => {
                    if !current.is_empty() {
                        parts.push(current);
                        current = String::new();
                    }
                    let mut index = String::new();
                    for next in chars.by_ref() {
                        if next == ']' {
                            break;
                        }
                        index.push(next);
                    }
                    if !index.is_empty() {
                        parts.push(index);
                    }
                }
                _ => current.push(ch),
            }
        }
        if !current.is_empty() {
            parts.push(current);
        }

        let mut value = self.values.get(parts.first()?)?;
        for part in &parts[1..] {
            match value {
                TemplateValue::Map(map) => {
                    value = map.get(part)?;
                }
                TemplateValue::List(list) => {
                    let index = part.parse::<usize>().ok()?;
                    value = list.get(index)?;
                }
                _ => return None,
            }
        }

        Some(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError {
    MissingVariable(String),
    UnterminatedIf(String),
    UnterminatedFor(String),
    InvalidCondition(String),
    InvalidExpand(String),
    InvalidFor(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Condition {
    Empty(String),
    Not(Box<Condition>),
}

impl Condition {
    fn parse(raw: &str) -> Result<Self, TemplateError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(TemplateError::InvalidCondition(raw.to_string()));
        }

        if let Some(expr) = trimmed.strip_prefix('!') {
            return Ok(Self::Not(Box::new(Self::parse(expr)?)));
        }

        let (name, args) = parse_call(trimmed)?;
        if name != "empty" || args.len() != 1 {
            return Err(TemplateError::InvalidCondition(trimmed.to_string()));
        }

        Ok(Self::Empty(args[0].clone()))
    }

    fn eval(&self, context: &TemplateContext) -> bool {
        match self {
            Condition::Empty(name) => match context.get(name) {
                Some(value) => value.is_empty(),
                None => true,
            },
            Condition::Not(inner) => !inner.eval(context),
        }
    }
}

fn parse_call(raw: &str) -> Result<(&str, Vec<String>), TemplateError> {
    let eq = raw
        .find('(')
        .ok_or_else(|| TemplateError::InvalidCondition(raw.to_string()))?;
    let close = raw
        .rfind(')')
        .ok_or_else(|| TemplateError::InvalidCondition(raw.to_string()))?;
    if close <= eq || close != raw.len() - 1 {
        return Err(TemplateError::InvalidCondition(raw.to_string()));
    }

    let name = &raw[..eq];
    let args = raw[eq + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    Ok((name.trim(), args))
}

pub fn render_template(template: &str, context: &TemplateContext) -> Result<String, TemplateError> {
    render_block(template, context)
}

fn render_block(template: &str, context: &TemplateContext) -> Result<String, TemplateError> {
    let mut out = String::new();
    let mut cursor = 0usize;

    while cursor < template.len() {
        let for_open = if template[cursor..].starts_with("%for ") {
            Some(("%for ", "%%"))
        } else if template[cursor..].starts_with("$for ") {
            Some(("$for ", "$$"))
        } else {
            None
        };
        if let Some((open, close)) = for_open {
            let header_end_marker = if open.starts_with('%') { '%' } else { '$' };
            let after_for = &template[cursor + open.len()..];
            let tag_end = after_for
                .find(header_end_marker)
                .ok_or_else(|| TemplateError::UnterminatedFor(template[cursor..].to_string()))?;
            let loop_header = after_for[..tag_end].trim();
            let Some((var_name, list_name)) = loop_header.split_once(" in ") else {
                return Err(TemplateError::InvalidFor(loop_header.to_string()));
            };
            let var_name = var_name.trim();
            let list_name = list_name.trim();
            if var_name.is_empty() || list_name.is_empty() {
                return Err(TemplateError::InvalidFor(loop_header.to_string()));
            }

            let body_start = cursor + open.len() + tag_end + 1;
            let mut block_end = body_start;
            let mut depth = 1usize;
            while block_end < template.len() {
                if template[block_end..].starts_with(open) {
                    depth += 1;
                    block_end += open.len();
                    continue;
                }
                if template[block_end..].starts_with(close) {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    block_end += close.len();
                    continue;
                }
                block_end += 1;
            }

            if depth != 0 {
                return Err(TemplateError::UnterminatedFor(loop_header.to_string()));
            }

            let values = match context.get(list_name) {
                Some(TemplateValue::List(items)) => items.clone(),
                Some(_) => return Err(TemplateError::InvalidFor(list_name.to_string())),
                None => return Err(TemplateError::MissingVariable(list_name.to_string())),
            };

            let body = &template[body_start..block_end];
            for item in values {
                let mut loop_context = context.clone();
                loop_context.insert(var_name, item);
                out.push_str(&render_block(body, &loop_context)?);
            }
            cursor = block_end + close.len();
            continue;
        }

        let if_open = if template[cursor..].starts_with("%if ") {
            Some(("%if ", "%"))
        } else if template[cursor..].starts_with("$if ") {
            Some(("$if ", "$"))
        } else {
            None
        };
        if let Some((open, header_end_marker)) = if_open {
            let after_if = &template[cursor + open.len()..];
            let cond_end = after_if
                .find(header_end_marker)
                .ok_or_else(|| TemplateError::UnterminatedIf(template[cursor..].to_string()))?;
            let condition_text = &after_if[..cond_end];
            let condition = Condition::parse(condition_text)?;
            let body_start = cursor + open.len() + cond_end + 1;
            let mut block_end = body_start;
            let mut depth = 1usize;
            let close = if open.starts_with('%') { "%%" } else { "$$" };
            while block_end < template.len() {
                if template[block_end..].starts_with(open) {
                    depth += 1;
                    block_end += open.len();
                    continue;
                }
                if template[block_end..].starts_with(close) {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    block_end += close.len();
                    continue;
                }
                block_end += 1;
            }

            if depth != 0 {
                return Err(TemplateError::UnterminatedIf(condition_text.to_string()));
            }

            let body = &template[body_start..block_end];
            if condition.eval(context) {
                out.push_str(&render_block(body, context)?);
            }
            cursor = block_end + close.len();
            continue;
        }

        if template[cursor..].starts_with("%expand(") || template[cursor..].starts_with("$expand(")
        {
            let (open, close_marker) = if template[cursor..].starts_with("%expand(") {
                ("%expand(", "%")
            } else {
                ("$expand(", "$")
            };
            let start = cursor + open.len();
            let close_paren = template[start..]
                .find(')')
                .ok_or_else(|| TemplateError::InvalidExpand(template[cursor..].to_string()))?;
            let expr = &template[start..start + close_paren];
            let close_tag = template[start + close_paren + 1..]
                .find(close_marker)
                .ok_or_else(|| TemplateError::InvalidExpand(template[cursor..].to_string()))?;
            let variable = expr.trim();
            let value = context
                .get(variable)
                .ok_or_else(|| TemplateError::MissingVariable(variable.to_string()))?;
            let rendered = match value {
                TemplateValue::Html(value) => value.clone(),
                TemplateValue::Bool(value) => value.to_string(),
                TemplateValue::List(_) | TemplateValue::Map(_) => {
                    return Err(TemplateError::InvalidExpand(variable.to_string()));
                }
            };
            out.push_str(&rendered);
            cursor = start + close_paren + close_tag + 2;
            continue;
        }

        if template[cursor..].starts_with('@') {
            let end = template[cursor + 1..].find('@').ok_or_else(|| {
                TemplateError::MissingVariable("unterminated placeholder".to_string())
            })?;
            let name = &template[cursor + 1..cursor + 1 + end];
            let value = context
                .get(name.trim())
                .ok_or_else(|| TemplateError::MissingVariable(name.to_string()))?;
            let rendered = match value {
                TemplateValue::Html(value) => value.clone(),
                TemplateValue::Bool(value) => value.to_string(),
                TemplateValue::List(_) | TemplateValue::Map(_) => {
                    return Err(TemplateError::InvalidExpand(name.trim().to_string()));
                }
            };
            out.push_str(&rendered);
            cursor += end + 2;
            continue;
        }

        let next_special = template[cursor..]
            .find(['%', '@', '$'])
            .unwrap_or(template.len() - cursor);
        out.push_str(&template[cursor..cursor + next_special]);
        cursor += next_special;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{render_template, TemplateContext, TemplateValue};

    #[test]
    fn renders_conditional_blocks() {
        let mut context = TemplateContext::new();
        context.insert(
            "DESCRIPTION",
            TemplateValue::Html("hello world".to_string()),
        );
        context.insert("GROUP", TemplateValue::Html("type".to_string()));

        let template = "prefix %if !empty(DESCRIPTION)%<b>%expand(DESCRIPTION)%</b>%% suffix";
        let rendered = render_template(template, &context).unwrap();
        assert_eq!(rendered, "prefix <b>hello world</b> suffix");
    }

    #[test]
    fn omits_empty_blocks() {
        let context = TemplateContext::new();
        let template = "%if !empty(DESCRIPTION)%<span>visible</span>%%";
        let rendered = render_template(template, &context).unwrap();
        assert_eq!(rendered, "");
    }

    #[test]
    fn renders_placeholders() {
        let mut context = TemplateContext::new();
        context.insert("TITLE", TemplateValue::Html("ssh.host".to_string()));
        let rendered = render_template("@TITLE@", &context).unwrap();
        assert_eq!(rendered, "ssh.host");
    }

    #[test]
    fn renders_for_loops() {
        let mut context = TemplateContext::new();
        context.insert(
            "HEADERS",
            TemplateValue::List(vec![
                TemplateValue::Html("Field".to_string()),
                TemplateValue::Html("Type".to_string()),
            ]),
        );

        let template = "%for header in HEADERS%<th>%expand(header)%</th>%%";
        let rendered = render_template(template, &context).unwrap();
        assert_eq!(rendered, "<th>Field</th><th>Type</th>");
    }

    #[test]
    fn renders_nested_row_columns() {
        let mut context = TemplateContext::new();
        context.insert(
            "ROWS",
            TemplateValue::List(vec![TemplateValue::Map(HashMap::from([(
                "columns".to_string(),
                TemplateValue::List(vec![
                    TemplateValue::Html("<td>name</td>".to_string()),
                    TemplateValue::Html("<td>type</td>".to_string()),
                ]),
            )]))]),
        );

        let template = "%for row in ROWS%<tr>$for column in row.columns$%expand(column)%$$</tr>%%";
        let rendered = render_template(template, &context).unwrap();
        assert_eq!(rendered, "<tr><td>name</td><td>type</td></tr>");
    }
}
