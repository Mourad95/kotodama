//! `kd`: analyze messages and evaluate the emotion engine from the command line.
//!
//! ```text
//! kd analyze "message" --bundle <dir>
//! kd eval <csv> --bundle <dir>
//! ```
//!
//! ONNX Runtime is loaded at run time: set `ORT_DYLIB_PATH` to `libonnxruntime.so`/`.dylib`.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use kd_core::Emotion;
use kd_emotion::{Bundle, Embedder, EmotionEngine};

const USAGE: &str = "usage:\n  kd analyze <message> --bundle <dir>\n  kd eval <csv> --bundle <dir>";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> ExitCode {
    match run(std::env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Splits arguments into the positional one and the `--bundle` value.
fn parse_args(args: &[OsString]) -> Result<(OsString, PathBuf)> {
    let mut positional = None;
    let mut bundle = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        if arg == "--bundle" {
            bundle = Some(PathBuf::from(
                it.next().ok_or("--bundle needs a directory")?,
            ));
        } else if positional.is_none() {
            positional = Some(arg.clone());
        } else {
            return Err(format!("unexpected argument `{}`\n{USAGE}", arg.to_string_lossy()).into());
        }
    }
    match (positional, bundle) {
        (Some(p), Some(b)) => Ok((p, b)),
        _ => Err(USAGE.into()),
    }
}

fn run(args: Vec<OsString>) -> Result<()> {
    let (command, rest) = args.split_first().ok_or(USAGE)?;
    let (input, bundle) = parse_args(rest)?;
    match command.to_str() {
        Some("analyze") => {
            let text = input.to_str().ok_or("the message is not valid UTF-8")?;
            analyze(text, &bundle)
        }
        Some("eval") => eval(Path::new(&input), &bundle),
        _ => Err(USAGE.into()),
    }
}

fn engine(bundle: &Path) -> Result<EmotionEngine<impl Embedder>> {
    Ok(Bundle::open(bundle)?.into_engine()?)
}

fn analyze(text: &str, bundle: &Path) -> Result<()> {
    let analysis = engine(bundle)?.analyze(text)?;
    for (emotion, p) in &analysis.scores.probabilities {
        println!("{:<10} {:.4}", emotion.id(), p);
    }
    println!("choice     {}", analysis.scores.choice.id());
    println!("confidence {:.4}", analysis.scores.confidence);
    println!("effective  {}", analysis.effective.id());
    Ok(())
}

/// Reads `(text, label)` rows from a CSV with `text` and `label` columns.
fn read_rows(path: &Path) -> Result<Vec<(String, Emotion)>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("{}: missing `{name}` column", path.display()))
    };
    let (text_col, label_col) = (column("text")?, column("label")?);
    let mut rows = Vec::new();
    for (i, record) in reader.records().enumerate() {
        let record = record?;
        let label = record.get(label_col).unwrap_or_default();
        let emotion = Emotion::from_id(label)
            .ok_or_else(|| format!("{}: row {}: unknown label `{label}`", path.display(), i + 2))?;
        rows.push((record.get(text_col).unwrap_or_default().to_owned(), emotion));
    }
    Ok(rows)
}

fn eval(csv_path: &Path, bundle: &Path) -> Result<()> {
    let rows = read_rows(csv_path)?;
    let engine = engine(bundle)?;
    // Per label: (correct, total). Accuracy is measured on the engine's raw choice.
    let mut per_label: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    let mut correct = 0;
    for (text, expected) in &rows {
        let choice = engine.analyze(text)?.scores.choice;
        let entry = per_label.entry(expected.id()).or_default();
        entry.1 += 1;
        if choice == *expected {
            entry.0 += 1;
            correct += 1;
        }
    }
    let pct = |c: usize, t: usize| {
        if t == 0 {
            0.0
        } else {
            100.0 * c as f64 / t as f64
        }
    };
    println!("messages   {}", rows.len());
    println!(
        "accuracy   {:.1} %  ({correct}/{})",
        pct(correct, rows.len()),
        rows.len()
    );
    for (label, (c, t)) in &per_label {
        println!("{label:<10} recall {:5.1} %  ({c}/{t})", pct(*c, *t));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<OsString> {
        v.iter().map(OsString::from).collect()
    }

    #[test]
    fn arguments_parse_in_any_order() {
        let (p, b) = parse_args(&args(&["--bundle", "dir", "salut"])).unwrap();
        assert_eq!((p, b), (OsString::from("salut"), PathBuf::from("dir")));
        assert!(parse_args(&args(&["salut"])).is_err());
        assert!(parse_args(&args(&["a", "b", "--bundle", "d"])).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn non_utf8_message_is_a_normal_error() {
        use std::os::unix::ffi::OsStringExt;
        let bad = OsString::from_vec(vec![0x66, 0xff, 0x6f]);
        let result = run(vec!["analyze".into(), bad, "--bundle".into(), "dir".into()]);
        assert!(result.unwrap_err().to_string().contains("UTF-8"));
    }

    #[test]
    fn csv_rows_handle_quoted_commas() {
        let path = std::env::temp_dir().join(format!("kd-cli-test-{}.csv", std::process::id()));
        std::fs::write(
            &path,
            "lang,label,text\nfr,joie,\"oui, super !\"\nfr,calme,ok\n",
        )
        .unwrap();
        let rows = read_rows(&path).unwrap();
        assert_eq!(rows[0], ("oui, super !".to_owned(), Emotion::Joy));
        assert_eq!(rows.len(), 2);
        std::fs::remove_file(path).unwrap();
    }
}
