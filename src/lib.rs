//!

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    unsafe_code,
    unused_braces,
    unused_qualifications,
    clippy::cast_possible_truncation,
    clippy::warn_pub_self_convention,
    clippy::approx_constant
)]
#![deny(clippy::absurd_extreme_comparisons)]
#![cfg_attr(not(test), deny(clippy::result_unwrap_used))]

use color_eyre::eyre;
use once_cell::sync::OnceCell;
use suffix::SuffixTable;

pub static SEPARATOR: OnceCell<char> = OnceCell::new();
pub static ENTRIES: OnceCell<SuffixTable> = OnceCell::new();

pub fn init_entries(entries: Vec<String>) -> eyre::Result<()> {
    #[cfg(feature = "multi-thread")]
    let entries_iter = entries.par_iter();
    #[cfg(not(feature = "multi-thread"))]
    let entries_iter = entries.iter();

    let entries_concat = entries_iter
        .map(|s| format!("{}{}", s, SEPARATOR.get().unwrap_or(&':')))
        .map(|s| s.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>();

    ENTRIES
        .set(SuffixTable::new(entries_concat))
        .map_err(|err| eyre::eyre!("couldn't initialize entries: {:?}", err))
}

/// Loads the filename of all programs in any folder on the PATH variable.
pub fn obtain_entries() -> eyre::Result<Vec<String>> {
    let paths = std::env::var("PATH")?
        .split(*SEPARATOR.get().unwrap_or(&':'))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    #[cfg(feature = "multi-thread")]
    let paths_iter = paths.par_iter();
    #[cfg(not(feature = "multi-thread"))]
    let paths_iter = paths.iter();

    Ok(paths_iter
        .filter_map(|path| {
            // if the dir exists, get all of its entries
            if let Ok(dir) = std::fs::read_dir(path) {
                Some(
                    dir.filter_map(|entry| {
                        // if the entry exits, get only the file name
                        // that's the only thing needed to run the program, and the only
                        // thing the user needs to see
                        if let Ok(entry) = entry {
                            Some(entry.file_name())
                        } else {
                            None
                        }
                    })
                    // collect the entries to a `Vec` so that Rust doesn't complain about
                    // lifetimes
                    .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        })
        .flatten()
        // convert to `String`s, if possible.
        .filter_map(|s| {
            if let Ok(s) = s.into_string() {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}
