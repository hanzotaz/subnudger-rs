use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <input.srt> <output.srt> <offset_ms>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let offset_ms: i64 = args[3].parse()?;

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut output = File::create(output_file)?;
    let time_regex = Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3})")?;

    while let Some(line) = lines.next() {
        let line = line?;
        if time_regex.is_match(&line) {
            let adjusted_line = time_regex.replace_all(&line, |caps: &regex::Captures| {
                let hours: i64 = caps[1].parse().unwrap();
                let minutes: i64 = caps[2].parse().unwrap();
                let seconds: i64 = caps[3].parse().unwrap();
                let millis: i64 = caps[4].parse().unwrap();

                let total_duration =
                    Duration::from_secs((hours * 3600 + minutes * 60 + seconds) as u64)
                        + Duration::from_millis(millis as u64);
                let total_millis = (total_duration.as_millis() as i64) + offset_ms;

                let new_duration = if total_millis >= 0 {
                    Duration::from_millis(total_millis as u64)
                } else {
                    Duration::from_millis(0)
                };

                format!(
                    "{:02}:{:02}:{:02},{:03}",
                    (new_duration.as_secs() / 3600) % 24,
                    (new_duration.as_secs() / 60) % 60,
                    new_duration.as_secs() % 60,
                    new_duration.subsec_millis()
                )
            });
            writeln!(output, "{}", adjusted_line)?;
        } else {
            writeln!(output, "{}", line)?;
        }
    }

    Ok(())
}
