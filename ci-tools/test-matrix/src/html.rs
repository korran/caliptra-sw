use std::fmt::Write;

use serde::Serialize;
use tinytemplate::{TinyTemplate, format_unescaped};

use crate::TestMatrix;

#[derive(Serialize)]
struct TemplateContext<'a> {
    pub matrix: &'a TestMatrix,
    pub javascript: &'static str,
    pub style: &'static str,
}

pub fn format(matrix: &TestMatrix) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("matrix", include_str!("html/matrix.html"))
        .unwrap();
    tt.add_template("index", include_str!("html/index.html"))
        .unwrap();
    tt.add_formatter("testcase_cell_style", |val, out| {
        let mut status = val
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or_default();
        if status.is_empty() {
            status = "na";
        }
        out.write_str(&status.to_ascii_lowercase())?;
        Ok(())
    });
    tt.add_formatter("format_unescaped", format_unescaped);
    tt.add_formatter("testcase_cell_text", |val, out| {
        let status = val.get("status").and_then(|s| s.as_str());
        let duration = val
            .get("duration")
            .and_then(|d| d.as_number())
            .and_then(|d| d.as_f64());
        match (status, duration) {
            (Some("Passed"), Some(duration)) => write!(out, "{:.1}s", duration),
            (Some("Passed"), _) => write!(out, "PASS"),
            (Some("Failed"), _) => write!(out, "FAIL"),
            (Some("Ignored"), _) => write!(out, "SKIP"),
            _ => write!(out, "n/a"),
        }?;
        Ok(())
    });
    tt.render("index", &TemplateContext {
        matrix,
        javascript: include_str!("html/matrix.js"),
        style: include_str!("html/style.css"),
    }).unwrap()
}
