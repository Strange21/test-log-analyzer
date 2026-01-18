use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod parser;
use parser::*;

fn main() -> io::Result<()> {
    let path = "messages.log";
    let file = match File::open(path) {
        Ok(file_obj) => file_obj,
        Err(e) => {
            eprintln!("Error while opening file: {}", e);
            return Err(e);
        }
    };
    let mut reader = BufReader::with_capacity(128 * 1024, file); // bigger buffer for throughput

    let mut counts = LogCount::default();
    let mut count: usize = 0;
    let mut buf = Vec::with_capacity(8 * 1024);

    // for (lineno, line_res) in reader.lines().enumerate() {  ** this loop is faster but allocates new String for each new line
    //     let line = match line_res {
    loop {
        buf.clear();
        let n = reader.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break; // EOF
        }

        // drop trailing newline and optional '\r'
        let mut slice = &buf[..];
        if slice.ends_with(&[b'\n']) {
            slice = &slice[..slice.len() - 1];
        }
        if slice.ends_with(&[b'\r']) {
            slice = &slice[..slice.len() - 1];
        }

        // convert to &str without allocation; if invalid UTF-8, treat as malformed
        let line_str = match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => {
                counts.malformed += 1;
                count += 1;
                continue;
            }
        };

        let s = line_str.trim();
        if s.is_empty() {
            count += 1;
            continue;
        }

        match parse_level(s) {
            Ok(Level::Error) => counts.error += 1,
            Ok(Level::Info) => counts.info += 1,
            Ok(Level::Warn) => counts.warn += 1,
            Err(_) => counts.malformed += 1,
        }
        count += 1;
    }
    println!("Total number of lines analysed {}", count);

    println!(
        "INFO: {}\nWARN: {}\nERROR: {}\nMALFORMED: {}",
        counts.info, counts.warn, counts.error, counts.malformed
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_line(ts: &str, lvl: &str, svc: &str, msg: &str) -> String {
        format!("{}|{}|{}|{}", ts, lvl, svc, msg)
    }

    #[test]
    fn parse_valid_levels() {
        let l = make_line("2025-01-01T12:00:00Z", "ERROR", "auth", "bad");
        assert_eq!(parse_level(&l).unwrap(), Level::Error);

        let l = make_line("2025-01-01T12:00:00Z. ", "info", "auth", "ok");
        assert_eq!(parse_level(&l).unwrap(), Level::Info);

        let l = make_line("2025-01-01T12:00:00Z", "Warn", "svc", "warny");
        assert_eq!(parse_level(&l).unwrap(), Level::Warn);

        let l = make_line("2025-01-01T12:00:00Z", "WARNING", "svc", "warny");
        assert_eq!(parse_level(&l).unwrap(), Level::Warn);
    }

    #[test]
    fn parse_malformed_lines() {
        // missing pipes
        let l = "no-pipes-here";
        assert_eq!(parse_level(l).unwrap_err(), LineErr::MissingField);

        // empty level
        let l = "2025-01-01T12:00:00Z||svc|msg";
        assert_eq!(parse_level(l).unwrap_err(), LineErr::EmptyLevel);

        // unknown level
        let l = make_line("2025-01-01T12:00:00Z", "VERBOSE", "svc", "msg");
        match parse_level(&l) {
            Err(LineErr::UnknownLevel(s)) => assert_eq!(s, "VERBOSE"),
            other => panic!("expected UnknownLevel, got {:?}", other),
        }

        // malformed character (null) inside level
        let l = make_line("2025-01-01T12:00:00Z", "ER\u{0}ROR", "svc", "msg");
        match parse_level(&l) {
            Err(LineErr::MalformedChars { pos, ch }) => {
                assert_eq!(ch, '\0');
                assert!(pos > 0);
            }
            other => panic!("expected MalformedChars, got {:?}", other),
        }
    }
}