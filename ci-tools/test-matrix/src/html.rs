use std::fmt::Write;

use octocrab::models::workflows::Run;
use serde::Serialize;
use tinytemplate::{TinyTemplate, format_unescaped};
use std::fmt::Debug;

use crate::{TestMatrix, RunInfo};

#[derive(Serialize)]
struct TemplateContext<'a> {
    pub matrix: &'a TestMatrix,
    pub javascript: &'static str,
    pub style: &'static str,
    pub run: &'a Run,
    pub run_infos: Vec<MaybeSelected<&'a RunInfo>>,
}

#[derive(Serialize)]
pub struct MaybeSelected<T: Serialize> {
    pub selected: bool,
    pub value: T,
}

impl<'a, T: Serialize + PartialEq> MaybeSelected<&'a T> {
    fn select(selected: &'a T, all: &'a [T]) -> Vec<Self> {
        let mut result = vec![];
        for val in all {
            result.push(
                MaybeSelected{
                    selected: *val == *selected,
                    value: val,
                }
            );
        }
        result
    }
}

impl<'a, T: Serialize + Debug> Debug for MaybeSelected<&'a T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaybeSelected").field("selected", &self.selected).field("value", &self.value).finish()
    }
}

pub fn format(run: &Run, toc: &[RunInfo], matrix: &TestMatrix) -> String {
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


    let this_run_info = RunInfo::from_run(run);
    tt.render("index", &TemplateContext {
        matrix,
        javascript: include_str!("html/matrix.js"),
        style: include_str!("html/style.css"),
        run: run,
        run_infos: MaybeSelected::select(&this_run_info, toc),
    }).unwrap()
}
