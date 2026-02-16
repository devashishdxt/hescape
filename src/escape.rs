use core::fmt;

/// Escapes a HTML string and returns the escaped string.
pub fn escape(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    escape_to(&mut output, input).unwrap();
    output
}

/// Escapes a HTML string into a writer.
pub fn escape_to<W>(writer: &mut W, input: &str) -> fmt::Result
where
    W: fmt::Write + ?Sized,
{
    // Fast path for strings without special characters
    if !input
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
    {
        writer.write_str(input)?;
        return Ok(());
    }

    let bytes = input.as_bytes();
    let mut last = 0usize;
    let mut i = 0usize;

    while i < bytes.len() {
        let replacement = match bytes[i] {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => {
                i += 1;
                continue;
            }
        };

        if last < i {
            writer.write_str(&input[last..i])?;
        }
        writer.write_str(replacement)?;

        i += 1;
        last = i;
    }

    if last < input.len() {
        writer.write_str(&input[last..])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(escape(""), "");
    }

    #[test]
    fn test_no_special_characters() {
        assert_eq!(escape("hello world"), "hello world");
    }

    #[test]
    fn test_escape_less_than() {
        assert_eq!(escape("<"), "&lt;");
    }

    #[test]
    fn test_escape_greater_than() {
        assert_eq!(escape(">"), "&gt;");
    }

    #[test]
    fn test_escape_ampersand() {
        assert_eq!(escape("&"), "&amp;");
    }

    #[test]
    fn test_escape_quote() {
        assert_eq!(escape("\""), "&quot;");
    }

    #[test]
    fn test_escape_apos() {
        assert_eq!(escape("\'"), "&#39;");
    }

    #[test]
    fn test_html_tag() {
        assert_eq!(escape("<div>"), "&lt;div&gt;");
    }

    #[test]
    fn test_mixed_content() {
        assert_eq!(
            escape("Hello <world> & \"friends\""),
            "Hello &lt;world&gt; &amp; &quot;friends&quot;"
        );
    }

    #[test]
    fn test_special_at_start() {
        assert_eq!(escape("<start"), "&lt;start");
    }

    #[test]
    fn test_special_at_end() {
        assert_eq!(escape("end>"), "end&gt;");
    }

    #[test]
    fn test_consecutive_special_chars() {
        assert_eq!(escape("<<>>"), "&lt;&lt;&gt;&gt;");
    }

    #[test]
    fn test_unicode_passthrough() {
        assert_eq!(escape("日本語"), "日本語");
        assert_eq!(escape("émoji 🎉"), "émoji 🎉");
    }

    #[test]
    fn test_unicode_with_special_chars() {
        assert_eq!(escape("<日本語>"), "&lt;日本語&gt;");
    }

    #[test]
    fn test_already_escaped_sequence() {
        assert_eq!(escape("&lt;"), "&amp;lt;");
        assert_eq!(escape("&amp;"), "&amp;amp;");
    }

    #[test]
    fn test_script_tag() {
        assert_eq!(
            escape("<script>alert(\"xss\")</script>"),
            "&lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;"
        );
    }
}
