// Licensed under the Apache-2.0 license

use std::{
    env::{self},
    fs, io,
    path::Path,
    process::Command,
};

use caliptra_builder::{elf_size, FwId};
use serde::{Deserialize, Serialize};

mod cache;
mod cache_gha;
mod git;
mod html;
mod http;
mod process;
mod util;

use crate::cache_gha::GithubActionCache;
use crate::{
    cache::{Cache, FsCache},
    process::run_cmd_stdout,
};

// Increment with non-backwards-compatible changes are made to the cache record
// format
const CACHE_FORMAT_VERSION: &str = "v0";

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
struct Sizes {
    size_with_uart: u64,
    size_prod: u64,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SizeRecord {
    commit: git::CommitInfo,
    sizes: Option<Sizes>,
}

fn main() {
    if let Err(e) = real_main() {
        println!("Fatal Error: {e}");
        std::process::exit(1);
    }
}

fn real_main() -> io::Result<()> {
    let cache = GithubActionCache::new().map(box_cache).or_else(|e| {
        let fs_cache_path = "/tmp/caliptra-size-cache";
        println!(
            "Unable to create github action cache: {e}; using fs-cache instead at {fs_cache_path}"
        );
        FsCache::new(fs_cache_path.into()).map(box_cache)
    })?;

    let git_commits = git::commit_log()?;

    let worktree = git::WorkTree::new(Path::new("/tmp/caliptra-size-history-wt"))?;

    env::set_current_dir(worktree.path)?;

    let mut records = vec![];

    let mut cached_commit = None;
    for commit in git_commits.iter() {
        match cache.get(&format_cache_key(&commit.id)) {
            Ok(Some(cached_records)) => {
                if let Ok(cached_records) =
                    serde_json::from_slice::<Vec<SizeRecord>>(&cached_records)
                {
                    println!("Found cache entry for remaining commits at {}", commit.id);
                    records.extend(cached_records);
                    cached_commit = Some(commit.id.clone());
                    break;
                } else {
                    println!(
                        "Error parsing cache entry {:?}",
                        String::from_utf8_lossy(&cached_records)
                    );
                }
            }
            Ok(None) => {} // not found
            Err(e) => println!("Error reading from cache: {e}"),
        }
        println!(
            "Building firmware at commit {}: {}",
            commit.id, commit.title
        );
        run_cmd_stdout(
            Command::new("git")
                .current_dir(worktree.path)
                .arg("checkout")
                .arg("--no-recurse-submodule")
                .arg("--quiet")
                .arg(&commit.id),
            None,
        )?;

        let sizes = match compute_size(
            &worktree,
            &caliptra_builder::ROM_WITH_UART,
            &caliptra_builder::ROM,
        ) {
            Ok(size_info) => Some(size_info),
            Err(err) => {
                println!("Error building commit {}: {err}", commit.id);
                None
            }
        };

        records.push(SizeRecord {
            commit: commit.clone(),
            sizes,
        });
    }
    for (i, record) in records.iter().enumerate() {
        if Some(&record.commit.id) == cached_commit.as_ref() {
            break;
        }
        if let Err(e) = cache.set(
            &format_cache_key(&record.commit.id),
            &serde_json::to_vec(&records[i..]).unwrap(),
        ) {
            println!(
                "Unable to write to cache for commit {}: {e}",
                record.commit.id
            );
        }
    }

    let html = html::format_records(&records)?;

    if let Ok(file) = env::var("GITHUB_STEP_SUMMARY") {
        fs::write(file, &html)?;
    } else {
        println!("{html}");
    }

    Ok(())
}

fn compute_size(
    worktree: &git::WorkTree,
    with_uart_fwid: &FwId,
    prod_fwid: &FwId,
) -> io::Result<Sizes> {
    // TODO: consider using caliptra_builder from the same repo as the firmware
    let rom_with_uart_elf = caliptra_builder::build_firmware_elf_uncached(&FwId {
        workspace_dir: Some(worktree.path),
        ..*with_uart_fwid
    })?;

    let rom_prod_elf = caliptra_builder::build_firmware_elf_uncached(&FwId {
        workspace_dir: Some(worktree.path),
        ..*prod_fwid
    })?;

    Ok(Sizes {
        size_with_uart: elf_size(&rom_with_uart_elf)?,
        size_prod: elf_size(&rom_prod_elf)?,
    })
}

fn box_cache(val: impl Cache + 'static) -> Box<dyn Cache> {
    Box::new(val)
}

fn format_cache_key(commit: &str) -> String {
    format!("{CACHE_FORMAT_VERSION}-{commit}")
}
