//! `kd`: analyze messages and evaluate the emotion engine from the command line.
//!
//! ```text
//! kd analyze "message" --bundle <dir>
//! kd eval <csv> --bundle <dir>
//! ```
//!
//! ONNX Runtime is loaded at run time: set `ORT_DYLIB_PATH` to `libonnxruntime.so`/`.dylib`.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;

use kd_core::Emotion;
use kd_emotion::{Bundle, Embedder, EmotionEngine};

const USAGE: &str = "usage:\n  kd analyze <message> --bundle <dir>\n  kd eval <csv> --bundle <dir>";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Splits arguments into the positional one and the `--bundle` value.
fn parse_args(args: &[String]) -> Result<(String, PathBuf)> {
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
            return Err(format!("unexpected argument `{arg}`\n{USAGE}").into());
        }
    }
    match (positional, bundle) {
        (Some(p), Some(b)) => Ok((p, b)),
        _ => Err(USAGE.into()),
    }
}

fn run(args: Vec<String>) -> Result<()> {
    let (command, rest) = args.split_first().ok_or(USAGE)?;
    let (input, bundle) = parse_args(rest)?;
    match command.as_str() {
        "analyze" => analyze(&input, &bundle),
        "eval" => eval(&input, &bundle),
        _ => Err(USAGE.into()),
    }
}

fn engine(bundle: &PathBuf) -> Result<EmotionEngine<impl Embedder>> {
    Ok(Bundle::open(bundle)?.into_engine()?)
}

fn analyze(text: &str, bundle: &PathBuf) -> Result<()> {
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
fn read_rows(path: &str) -> Result<Vec<(String, Emotion)>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("{path}: missing `{name}` column"))
    };
    let (text_col, label_col) = (column("text")?, column("label")?);
    let mut rows = Vec::new();
    for (i, record) in reader.records().enumerate() {
        let record = record?;
        let label = record.get(label_col).unwrap_or_default();
        let emotion = Emotion::from_id(label)
            .ok_or_else(|| format!("{path}: row {}: unknown label `{label}`", i + 2))?;
        rows.push((record.get(text_col).unwrap_or_default().to_owned(), emotion));
    }
    Ok(rows)
}

fn eval(csv_path: &str, bundle: &PathBuf) -> Result<()> {
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

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn arguments_parse_in_any_order() {
        let (p, b) = parse_args(&args(&["--bundle", "dir", "salut"])).unwrap();
        assert_eq!((p.as_str(), b), ("salut", PathBuf::from("dir")));
        assert!(parse_args(&args(&["salut"])).is_err());
        assert!(parse_args(&args(&["a", "b", "--bundle", "d"])).is_err());
    }

    #[test]
    fn csv_rows_handle_quoted_commas() {
        let path = std::env::temp_dir().join(format!("kd-cli-test-{}.csv", std::process::id()));
        std::fs::write(
            &path,
            "lang,label,text\nfr,joie,\"oui, super !\"\nfr,calme,ok\n",
        )
        .unwrap();
        let rows = read_rows(path.to_str().unwrap()).unwrap();
        assert_eq!(rows[0], ("oui, super !".to_owned(), Emotion::Joy));
        assert_eq!(rows.len(), 2);
        std::fs::remove_file(path).unwrap();
    }
}
