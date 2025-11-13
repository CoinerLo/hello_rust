fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn append_tag(buffer: &mut String, tag: &str) {
    if !buffer.is_empty() {
        buffer.push(' ');
    }

    buffer.push_str(tag);
}

fn extract_domain(url: &str) -> &str {
    let start = url.find("://").map(|p| p + 3).unwrap_or(0);
    let domain_end = url[start..].find('/').map(|p| start + p).unwrap_or(url.len());
    &url[start..domain_end]
}

fn redact_sensitive<'a>(text: &'a mut String, keyword: &'a str) -> &'a str {
    let mut last_end = 0;
    let mut result = String::new();

    while let Some(start) = text[last_end..].find(keyword) {
        let abs_start = last_end + start;
        result.push_str(&text[last_end..abs_start]);
        result.push_str("[REDACTED]");
        last_end = abs_start + keyword.len();
    }

    result.push_str(&text[last_end..]);

    *text = result;
    &text[..10.min(text.len())]
}

fn build_summary<'a>(title: &'a str, body: &'a mut String, max_len: usize) -> (usize, &'a str) {
    let word_count = count_words(body);

    if body.len() > max_len {
        body.truncate(max_len);
        body.push_str("...");
    }

    append_tag(body, "[SUMMARY]");

    let preview = extract_domain(title);
    (word_count, preview)
}

fn main() {
    let url = "https://example.com/article";
    let mut content = String::from("  Secret password is 12345  ");

    {
        let temp = "hello world";
        assert_eq!(count_words(temp), 2);
    }

    let (words, domain) = build_summary(url, &mut content, 15);

    assert_eq!(words, 4);
    assert_eq!(domain, "example.com");

    assert!(content.ends_with("... [SUMMARY]"));

    let redacted_preview = redact_sensitive(&mut content, "  Secret");

    assert_eq!(redacted_preview, "[REDACTED]");
    println!("Все тесты прошли!");
}
