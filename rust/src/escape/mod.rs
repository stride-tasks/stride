#[cfg(test)]
mod tests;

pub(crate) fn escape(input: &str, out: &mut String) {
    for c in input.chars() {
        match c {
            '\x07' => out.push_str("\\a"), // Bell
            '\x08' => out.push_str("\\b"), // Backspace
            '\x0B' => out.push_str("\\v"), // Vertical tab
            '\x0C' => out.push_str("\\f"), // Form feed
            '\'' => out.push('\''),        // Form feed
            c => out.extend(c.escape_debug()),
        }
    }
}

pub(crate) fn unescape(input: &str, out: &mut String) {
    let mut iter = input.chars();
    while let Some(c) = iter.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }

        let Some(c2) = iter.next() else {
            out.push(c);
            break;
        };

        match c2 {
            'a' => out.push('\x07'), // Bell
            'b' => out.push('\x08'), // Backspace
            'n' => out.push('\n'),   // Newline
            'r' => out.push('\r'),   // Carriage return
            't' => out.push('\t'),   // Horizontal tab
            'v' => out.push('\x0B'), // Vertical tab
            'f' => out.push('\x0C'), // Form feed
            '\'' => out.push('\''),  // Single quote
            '"' => out.push('"'),    // Double quote
            '\\' => out.push('\\'),  // Backslash
            'x' => unescape_byte(&mut iter, out),
            'u' => {
                let result = unescape_unicode(iter.clone(), out);
                if let Some(result) = result {
                    iter = result;
                } else {
                    out.push_str("\\u");
                }
            }
            c2 => out.extend([c, c2]),
        }
    }
}

fn unescape_byte(iter: &mut std::str::Chars<'_>, out: &mut String) {
    let Some(c1) = iter.next() else {
        out.extend(['\\', 'x']);
        return;
    };
    let Some(c2) = iter.next() else {
        out.extend(['\\', 'x', c1]);
        return;
    };

    let (Some(d1), Some(d2)) = (c1.to_digit(16), c2.to_digit(16)) else {
        out.extend(['\\', 'x', c1, c2]);
        return;
    };
    let Some(c) = char::from_u32(d1 * 16 + d2) else {
        out.extend(['\\', 'x', c1, c2]);
        return;
    };

    out.push(c);
}

fn unescape_unicode<'a>(
    mut iter: std::str::Chars<'a>,
    out: &mut String,
) -> Option<std::str::Chars<'a>> {
    let c1 = iter.next()?;
    let d2 = iter.next()?.to_digit(16)?;

    if c1 != '{' {
        let d1 = c1.to_digit(16)?;
        let d3 = iter.next()?.to_digit(16)?;
        let d4 = iter.next()?.to_digit(16)?;

        out.push(char::from_u32(
            d1 * 16 * 16 * 16 + d2 * 16 * 16 + d3 * 16 + d4,
        )?);
        return Some(iter);
    }

    let mut value = d2;
    let mut count = 0;
    while let Some(c) = iter.next() {
        if c == '}' {
            out.push(char::from_u32(value)?);
            return Some(iter);
        }
        if count == 3 {
            return None;
        }
        value = value * 16 + c.to_digit(16)?;
        count += 1;
    }

    None
}
