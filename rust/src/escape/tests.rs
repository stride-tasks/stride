use crate::escape::unescape;

use super::escape;

#[test]
fn escaping_simple() {
    let input = "Hello, world";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, input);
}

#[test]
fn unescape_simple() {
    let input = "Hello, world";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world");
}

#[test]
fn escaping_newline() {
    let input = "Hello, world\n\nHi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\n\\nHi there!");
}

#[test]
fn unescape_newline() {
    let input = "Hello, world\\n\\nHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\n\nHi there!");
}

#[test]
fn escaping_carriage_return() {
    let input = "Hello, world\rHi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\rHi there!");
}

#[test]
fn unescape_carriage_return() {
    let input = "Hello, world\\rHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\rHi there!");
}

#[test]
fn escaping_horizontal_tab() {
    let input = "Hello, world\tHi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\tHi there!");
}

#[test]
fn unescape_horizontal_tab() {
    let input = "Hello, world\\tHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\tHi there!");
}

#[test]
fn escaping_vertical_tab() {
    let input = "Hello, world\x0bHi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\vHi there!");
}

#[test]
fn unescape_vertical_tab() {
    let input = "Hello, world\\vHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\x0bHi there!");
}

#[test]
fn escaping_form_feed() {
    let input = "Hello, world\x0cHi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\fHi there!");
}

#[test]
fn unescape_form_feed() {
    let input = "Hello, world\\fHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\x0cHi there!");
}

#[test]
fn escaping_backspace() {
    let input = "Hello, world\x08Hi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\bHi there!");
}

#[test]
fn unescape_backspace() {
    let input = "Hello, world\\fHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\x0CHi there!");
}

#[test]
fn escaping_backslash() {
    let input = "Hello, world\\r\nHi there!";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "Hello, world\\\\r\\nHi there!");
}

#[test]
fn unescape_backslash() {
    let input = "Hello, world\\\\r\\nHi there!";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "Hello, world\\r\nHi there!");
}

#[test]
fn escaping_single_backslash() {
    let input = "\\";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "\\\\");
}

#[test]
fn unescape_single_backslash() {
    let input = "\\\\";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\");
}

#[test]
fn escaping_single_quotes() {
    let input = "'Hello, world'";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "'Hello, world'");
}

#[test]
fn unescape_single_quotes() {
    let input = "'Hello,\' world'";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "'Hello,' world'");
}

#[test]
fn escaping_double_quotes() {
    let input = "\"Hello, world\"";

    let mut out = String::new();
    escape(input, &mut out);

    assert_eq!(out, "\"Hello, world\"");
}

#[test]
fn unescape_double_quotes() {
    let input = "\\\"Hello, world\\\"";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\"Hello, world\"");
}

#[test]
fn unescape_hex_upper_and_lowercase() {
    let input = "\\x00HELLO\\x0a \\x0A WORLD\\x60";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\x00HELLO\x0a \x0A WORLD\x60");
}

#[test]
fn unescape_hex_no_bytes_invalid() {
    let input = "\\x";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\x");
}

#[test]
fn unescape_hex_one_byte_invalid() {
    let input = "\\x0 HELLO\\xNa \\x$A WORLD\\x6";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\x0 HELLO\\xNa \\x$A WORLD\\x6");
}

#[test]
fn unescape_hex_two_byte_invalid() {
    let input = "\\x0s HELLO\\xNa \\xg$ WORLD\\x6x";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\x0s HELLO\\xNa \\xg$ WORLD\\x6x");
}

#[test]
fn unescape_unicode_non_braced_simple() {
    let input = "\\u000A";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\u{000A}");
}

#[test]
fn unescape_unicode_non_braced_single_char_invalid() {
    let input = "\\uA";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\uA");
}

#[test]
fn unescape_unicode_non_braced_two_char_invalid() {
    let input = "\\u0A";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\u0A");
}

#[test]
fn unescape_unicode_non_braced_three_char_invalid() {
    let input = "\\u00A";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\u00A");
}

#[test]
fn unescape_unicode_braced_simple() {
    let input = "\\u{A}";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\n");
}

#[test]
fn unescape_unicode_braced_double_char() {
    let input = "\\u{0A}";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\n");
}

#[test]
fn unescape_unicode_braced_three_char() {
    let input = "\\u{00A}";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\n");
}

#[test]
fn unescape_unicode_braced_four_char() {
    let input = "\\u{000A}";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\n");
}

#[test]
fn unescape_unicode_braced_five_char_invalid() {
    let input = "\\u{0000A}";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\\u{0000A}");
}

#[test]
fn unescape_unicode_braced_max_char() {
    let input = "\\u{FFFF}";

    let mut out = String::new();
    unescape(input, &mut out);

    assert_eq!(out, "\u{FFFF}");
}
