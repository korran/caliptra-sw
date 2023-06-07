// Licensed under the Apache-2.0 license

use std::{io, path::Path, process::Command};

use serde::{Deserialize, Serialize};

use crate::{
    process::{run_cmd, run_cmd_stdout},
    util::{bytes_to_string, expect_line, expect_line_with_prefix},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub title: String,
}
impl CommitInfo {
    fn parse_multiple(s: &str) -> io::Result<Vec<CommitInfo>> {
        let mut lines = s.lines();
        let mut result = vec![];
        'outer: loop {
            let Some(line) = lines.next() else {
                break;
            };
            let commit_id = expect_line_with_prefix("commit ", Some(line))?;
            let author = expect_line_with_prefix("Author: ", lines.next())?;
            expect_line("", lines.next())?;
            let mut title = expect_line_with_prefix("    ", lines.next())?.to_string();
            'inner: loop {
                let Some(line) = lines.next() else {
                    result.push(CommitInfo{
                        id: commit_id.into(),
                        author: author.into(),
                        title,
                    });
                    break 'outer;
                };
                if line.is_empty() {
                    break 'inner;
                }
                title.push('\n');
                title.push_str(expect_line_with_prefix("    ", Some(line))?);
            }
            result.push(CommitInfo {
                id: commit_id.into(),
                author: author.into(),
                title,
            });
        }
        Ok(result)
    }
}

pub fn is_log_linear()()

pub fn commit_log() -> io::Result<Vec<CommitInfo>> {
    CommitInfo::parse_multiple(&bytes_to_string(run_cmd_stdout(
        Command::new("git")
            .arg("log")
            .arg("--pretty=short")
            .arg("--decorate=no"),
        None,
    )?)?)
}

pub struct WorkTree<'a> {
    pub path: &'a Path,
}
impl<'a> WorkTree<'a> {
    pub fn new(path: &'a Path) -> io::Result<Self> {
        run_cmd(Command::new("git").arg("worktree").arg("add").arg(path))?;
        Ok(Self { path })
    }
}
impl Drop for WorkTree<'_> {
    fn drop(&mut self) {
        let _ = run_cmd(
            Command::new("git")
                .arg("worktree")
                .arg("remove")
                .arg(self.path),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_info_parse() {
        let text = r#"commit e1b1e3c566b6bd7cdef0310dc88480034f0aa29f
Author: Vishal Mhatre <38512878+mhatrevi@users.noreply.github.com>

    [fix] Vendor signature should not include owner signed data (#319)

commit bd306c4809f54426a357ff01507ef660291e2b91
Author: Kor Nielsen <kor@google.com>

    Remove RUSTFLAGS from legacy ROM makefile. (#318)
    Multiline title
"#;
        assert_eq!(
            CommitInfo::parse_multiple(text).unwrap(),
            vec![
                CommitInfo {
                    id: "e1b1e3c566b6bd7cdef0310dc88480034f0aa29f".into(),
                    author: "Vishal Mhatre <38512878+mhatrevi@users.noreply.github.com>".into(),
                    title: "[fix] Vendor signature should not include owner signed data (#319)"
                        .into()
                },
                CommitInfo {
                    id: "bd306c4809f54426a357ff01507ef660291e2b91".into(),
                    author: "Kor Nielsen <kor@google.com>".into(),
                    title: "Remove RUSTFLAGS from legacy ROM makefile. (#318)\nMultiline title"
                        .into()
                }
            ]
        );
    }
}
