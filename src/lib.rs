// SPDX-FileCopyrightText: 2024 Integral <integral@member.fsf.org>
//
// SPDX-License-Identifier: MPL-2.0

use rayon::prelude::*;
use std::{fs, io, path::Path};

const KIBIBYTE: u64 = 1 << 10;
const MEBIBYTE: u64 = 1 << 20;
const GIBIBYTE: u64 = 1 << 30;
const TEBIBYTE: u64 = 1 << 40;
const PEBIBYTE: u64 = 1 << 50;
const EXBIBYTE: u64 = 1 << 60;

/// Get the size of the file (in bytes).
///
/// If `path` points to a directory, calculate the size of directory recursively,
/// including all of its files and subdirectories.
///
/// This function will return an error if `path` does not exist,
/// or user lacks permissions to perform `metadata` call on `path`.
pub fn get_size_in_bytes(path: &Path) -> io::Result<u64> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_file() => Ok(meta.len()),
        Ok(meta) if meta.is_dir() => get_dir_size(path),
        Ok(_) => Ok(0),
        Err(e) => Err(e),
    }
}

/// Get the size of the file (in human-readable bytes).
///
/// If `path` points to a directory, calculate the size of directory recursively,
/// including all of its files and subdirectories.
///
/// This function will return an error if `path` does not exist,
/// or user lacks permissions to perform `metadata` call on `path`.
pub fn get_size_in_human_bytes(path: &Path) -> io::Result<String> {
    Ok(convert_to_human_bytes(get_size_in_bytes(path)?, false))
}

/// Get the size of the file (in human-readable bytes, using abbreviated units (K, M, G, etc.))
///
/// If `path` points to a directory, calculate the size of directory recursively,
/// including all of its files and subdirectories.
///
/// This function will return an error if `path` does not exist,
/// or user lacks permissions to perform `metadata` call on `path`.
pub fn get_size_in_abbr_human_bytes(path: &Path) -> io::Result<String> {
    Ok(convert_to_human_bytes(get_size_in_bytes(path)?, true))
}

fn get_dir_size(path: &Path) -> io::Result<u64> {
    let entries: Vec<_> = fs::read_dir(path)?.collect();

    let total = entries
        .par_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => match entry.metadata() {
                Ok(meta) if meta.is_file() => Some(meta.len()),
                Ok(meta) if meta.is_dir() => get_dir_size(&entry.path()).ok(),
                _ => None,
            },
            _ => None,
        })
        .sum();

    Ok(total)
}

fn convert_to_human_bytes(size_in_bytes: u64, abbr: bool) -> String {
    const UNITS: [((u64, u64), &str, &str); 6] = [
        ((1, KIBIBYTE), "B", "Bytes"),      // Bytes
        ((KIBIBYTE, MEBIBYTE), "K", "KiB"), // KiB
        ((MEBIBYTE, GIBIBYTE), "M", "MiB"), // MiB
        ((GIBIBYTE, TEBIBYTE), "G", "GiB"), // GiB
        ((TEBIBYTE, PEBIBYTE), "T", "TiB"), // TiB
        ((PEBIBYTE, EXBIBYTE), "P", "PiB"), // PiB
    ];

    for ((min_bytes, max_bytes), abbr_unit, full_unit) in UNITS {
        if size_in_bytes < max_bytes {
            return format!(
                "{} {}",
                size_in_bytes / min_bytes,
                if abbr { abbr_unit } else { full_unit }
            );
        }
    }

    format!(
        "{} {}",
        size_in_bytes / EXBIBYTE,
        if abbr { "E" } else { "EiB" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_human_bytes() {
        for (size_in_bytes, human_bytes) in [
            (0, "0 Bytes"),
            (KIBIBYTE - 1, "1023 Bytes"),
            (KIBIBYTE, "1 KiB"),
            (MEBIBYTE - 1, "1023 KiB"),
            (MEBIBYTE, "1 MiB"),
            (GIBIBYTE - 1, "1023 MiB"),
            (GIBIBYTE, "1 GiB"),
            (TEBIBYTE - 1, "1023 GiB"),
            (TEBIBYTE, "1 TiB"),
            (PEBIBYTE - 1, "1023 TiB"),
            (PEBIBYTE, "1 PiB"),
            (EXBIBYTE - 1, "1023 PiB"),
            (EXBIBYTE, "1 EiB"),
        ] {
            println!("{size_in_bytes} bytes -> {human_bytes}");
            assert_eq!(convert_to_human_bytes(size_in_bytes, false), human_bytes);
        }
    }

    #[test]
    fn test_convert_to_abbr_human_bytes() {
        for (size_in_bytes, abbr_human_bytes) in [
            (0, "0 B"),
            (KIBIBYTE - 1, "1023 B"),
            (KIBIBYTE, "1 K"),
            (MEBIBYTE - 1, "1023 K"),
            (MEBIBYTE, "1 M"),
            (GIBIBYTE - 1, "1023 M"),
            (GIBIBYTE, "1 G"),
            (TEBIBYTE - 1, "1023 G"),
            (TEBIBYTE, "1 T"),
            (PEBIBYTE - 1, "1023 T"),
            (PEBIBYTE, "1 P"),
            (EXBIBYTE - 1, "1023 P"),
            (EXBIBYTE, "1 E"),
        ] {
            println!("{size_in_bytes} bytes -> {abbr_human_bytes}");
            assert_eq!(
                convert_to_human_bytes(size_in_bytes, true),
                abbr_human_bytes
            );
        }
    }
}
