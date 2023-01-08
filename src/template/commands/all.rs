use std::io;

use crate::template::{
    readme_benchmarks::{self, Timings},
    ANSI_BOLD, ANSI_ITALIC, ANSI_RESET,
};

pub fn all_handler(is_release: bool, is_timed: bool) {
    let mut timings: Vec<Timings> = vec![];

    (1..=25).for_each(|day| {
        if day > 1 {
            println!();
        }

        println!("{}Day {}{}", ANSI_BOLD, day, ANSI_RESET);
        println!("------");

        let output = child_commands::run_solution(day, is_timed, is_release).unwrap();

        if output.is_empty() {
            println!("Not solved.");
        } else {
            let val = child_commands::parse_exec_time(&output, day);
            timings.push(val);
        }
    });

    if is_timed {
        let total_millis = timings.iter().map(|x| x.total_nanos).sum::<f64>() / 1000000_f64;

        println!(
            "\n{}Total:{} {}{:.2}ms{}",
            ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total_millis, ANSI_RESET
        );

        if is_release {
            match readme_benchmarks::update(timings, total_millis) {
                Ok(_) => println!("Successfully updated README with benchmarks."),
                Err(_) => {
                    eprintln!("Failed to update readme with benchmarks.");
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    BrokenPipe,
    Parser(String),
    IO(io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

pub fn get_path_for_bin(day: usize) -> String {
    let day_padded = format!("{:02}", day);
    format!("./src/bin/{}.rs", day_padded)
}

/// All solutions live in isolated binaries.
/// This module encapsulates interaction with these binaries, both invoking them as well as parsing the timing output.
mod child_commands {
    use super::{get_path_for_bin, Error};
    use std::{
        io::{BufRead, BufReader},
        path::Path,
        process::{Command, Stdio},
        thread,
    };

    /// Run the solution bin for a given day
    pub fn run_solution(
        day: usize,
        is_timed: bool,
        is_release: bool,
    ) -> Result<Vec<String>, Error> {
        let day_padded = format!("{:02}", day);

        // skip command invocation for days that have not been scaffolded yet.
        if !Path::new(&get_path_for_bin(day)).exists() {
            return Ok(vec![]);
        }

        let mut args = vec!["run", "--quiet", "--bin", &day_padded];

        if is_release {
            args.push("--release");
        }

        if is_timed {
            // mirror `--time` flag to child invocations.
            args.push("--");
            args.push("--time");
        }

        // spawn child command with piped stdout/stderr.
        // forward output to stdout/stderr while grabbing stdout lines.

        let mut cmd = Command::new("cargo")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = BufReader::new(cmd.stdout.take().ok_or(super::Error::BrokenPipe)?);
        let stderr = BufReader::new(cmd.stderr.take().ok_or(super::Error::BrokenPipe)?);

        let mut output = vec![];

        let thread = thread::spawn(move || {
            stderr.lines().for_each(|line| {
                eprintln!("{}", line.unwrap());
            });
        });

        for line in stdout.lines() {
            let line = line.unwrap();
            println!("{}", line);
            output.push(line);
        }

        thread.join().unwrap();
        cmd.wait()?;

        Ok(output)
    }

    pub fn parse_exec_time(output: &[String], day: usize) -> super::Timings {
        let mut timings = super::Timings {
            day,
            part_1: None,
            part_2: None,
            parser: None,
            total_nanos: 0_f64,
        };

        output
            .iter()
            .filter_map(|l| {
                if !l.contains(" samples)") {
                    return None;
                }

                let (timing_str, nanos) = match parse_time(l) {
                    Some(v) => v,
                    None => {
                        eprintln!("Could not parse timings from line: {l}");
                        return None;
                    }
                };

                let part = l.split(':').next()?;
                Some((part, timing_str, nanos))
            })
            .for_each(|(part, timing_str, nanos)| {
                if part.contains("Part 1") {
                    timings.part_1 = Some(timing_str.into());
                } else if part.contains("Part 2") {
                    timings.part_2 = Some(timing_str.into());
                } else if part.contains("Parser") {
                    timings.parser = Some(timing_str.into());
                }

                timings.total_nanos += nanos;
            });

        timings
    }

    fn parse_to_float(s: &str, postfix: &str) -> Option<f64> {
        s.split(postfix).next()?.parse().ok()
    }

    fn parse_time(line: &str) -> Option<(&str, f64)> {
        // for possible time formats, see: https://github.com/rust-lang/rust/blob/1.64.0/library/core/src/time.rs#L1176-L1200
        let str_timing = line
            .split(" samples)")
            .next()?
            .split('(')
            .last()?
            .split('@')
            .next()?
            .trim();

        let parsed_timing = match str_timing {
            s if s.contains("ns") => s.split("ns").next()?.parse::<f64>().ok(),
            s if s.contains("µs") => parse_to_float(s, "µs").map(|x| x * 1000_f64),
            s if s.contains("ms") => parse_to_float(s, "ms").map(|x| x * 1000000_f64),
            s => parse_to_float(s, "s").map(|x| x * 1000000000_f64),
        }?;

        Some((str_timing, parsed_timing))
    }

    /// copied from: https://github.com/rust-lang/rust/blob/1.64.0/library/std/src/macros.rs#L328-L333
    #[cfg(test)]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = (&$a, &$b);
            assert!(
                (*a - *b).abs() < 1.0e-6,
                "{} is not approximately equal to {}",
                *a,
                *b
            );
        }};
    }

    #[cfg(test)]
    mod tests {
        use super::parse_exec_time;

        #[test]
        fn test_well_formed() {
            let res = parse_exec_time(
                &[
                    "Parser: ✓ (7.3µs @ 6579 samples)".into(),
                    "Part 1: 0 (74.13ns @ 100000 samples)".into(),
                    "Part 2: 10 (74.13ms @ 99999 samples)".into(),
                    "".into(),
                ],
                1,
            );
            assert_approx_eq!(res.total_nanos, 74137374.13_f64);
            assert_eq!(res.parser.unwrap(), "7.3µs");
            assert_eq!(res.part_1.unwrap(), "74.13ns");
            assert_eq!(res.part_2.unwrap(), "74.13ms");
        }

        #[test]
        fn test_patterns_in_input() {
            let res = parse_exec_time(
                &[
                    "Parser: ✓    (1s @ 5 samples)".into(),
                    "Part 1: @ @ @ ( ) ms (2s @ 5 samples)".into(),
                    "Part 2: 10s (100ms @ 1 samples)".into(),
                    "".into(),
                ],
                1,
            );
            assert_approx_eq!(res.total_nanos, 3100000000_f64);
            assert_eq!(res.parser.unwrap(), "1s");
            assert_eq!(res.part_1.unwrap(), "2s");
            assert_eq!(res.part_2.unwrap(), "100ms");
        }

        #[test]
        fn test_missing_parts() {
            let res = parse_exec_time(
                &[
                    "Parser: ✓ (1ms @ 6579 samples)".into(),
                    "Part 1: ✖        ".into(),
                    "Part 2: ✖        ".into(),
                    "".into(),
                ],
                1,
            );
            assert_approx_eq!(res.total_nanos, 1000000_f64);
            assert_eq!(res.parser.unwrap(), "1ms");
            assert_eq!(res.part_1.is_none(), true);
            assert_eq!(res.part_2.is_none(), true);
        }
    }
}
