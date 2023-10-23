use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    n: bool, // Print the line numbers of each matching line.
    l: bool, // Print only the names of files that contain at least one matching line.
    i: bool, // Match line using a case-insensitive comparison.
    v: bool, // Invert the program -- collect all lines that fail to match the pattern.
    x: bool  // Only match entire lines, instead of lines that contain a match.
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut res = Flags {
            n: false,
            l: false,
            i: false,
            v: false,
            x: false
        };
        flags.iter().for_each(|&f| {
            match f {
                "-n" => res.n = true,
                "-l" => res.l = true,
                "-i" => res.i = true,
                "-v" => res.v = true,
                "-x" => res.x = true,
                _ => ()
            }
        });
        res
    }
}

use std::fs;
pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    if files.is_empty() {
        return Err(Error::msg("no files can be read."));
    }
    let mut match_result: Vec<String> = Vec::new();
    for &f in files.iter() {
        if let Ok(file) = fs::read_to_string(f) {
            for (i, line) in file.lines().enumerate() {
                let mut source: String = line.to_string();
                let mut pat: String = pattern.to_string();
                if flags.i {
                    source = line.to_lowercase();
                    pat = pattern.to_lowercase();
                }
                let mut is_matched: bool;
                if flags.x {
                    is_matched = source == pat;
                }
                else {
                    is_matched = source.contains(&pat);
                }

                if flags.v {
                    is_matched = !is_matched;
                }

                if is_matched {
                    let single_file = files.len() == 1;
                    let added = append_features(flags, single_file, f, i+1, line);
                    match_result.push(added);
                }
            }
        }
        else {
            return Err(Error::msg("file opening failed.".to_string()));
        }
    }
    if flags.l {
        let mut result: Vec<String> = 
            match_result.iter_mut()
                  .map(|s| {
                      if s.ends_with(":") {
                        s.strip_suffix(':').unwrap().to_string()
                      }
                      else {
                          s.to_string()
                      }
                  })
                  .collect();
        result.dedup();
        if files.len() == 1 && !result.is_empty() {
            result = vec![files[0].to_string()];
        }
        return Ok(result);
    }
    else {
        return Ok(match_result);
    }
}

fn append_features(flags: &Flags, single: bool, name: &str, i: usize, line: &str) -> String {
    let mut result: String = String::new();
    if !single {
        result += name;
        result += ":";
    }
    if !flags.l {
        if flags.n {
            result += &(i.to_string() + ":");
        }
        result += line;
    }
    return result;
}
