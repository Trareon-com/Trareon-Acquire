pub fn acquisition_report_html(
    case_reference: &str,
    source: &str,
    sha256: &str,
    limitations: &[String],
) -> String {
    let limitations = if limitations.is_empty() {
        "<li>None reported</li>".to_string()
    } else {
        limitations
            .iter()
            .map(|item| format!("<li>{}</li>", escape_html(item)))
            .collect()
    };
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>Trareon acquisition report</title></head><body><h1>Acquisition Report</h1><dl><dt>Case</dt><dd>{}</dd><dt>Source</dt><dd>{}</dd><dt>SHA-256</dt><dd><code>{}</code></dd></dl><h2>Limitations</h2><ul>{}</ul></body></html>",
        escape_html(case_reference),
        escape_html(source),
        escape_html(sha256),
        limitations,
    )
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

    #[test]
    fn report_escapes_input_and_lists_limitations() {
        let html = acquisition_report_html("A<1", "disk", "abc", &["read < error".to_string()]);
        assert!(html.contains("A&lt;1"));
        assert!(html.contains("read &lt; error"));
    }
}
