mod statistics;

use crate::statistics::{FileStatistics, Statistics};
use lalrpop_util::lalrpop_mod;
use serde_json::Error as SerdeError;
use std::path::Path;
use std::{env, fmt, fs, io, process};

lalrpop_mod! {
  #[allow(clippy::all)]
  sysy
}

fn main() {
  if let Err(err) = try_main() {
    eprintln!("{}", err);
    process::exit(1);
  }
}

fn try_main() -> Result<(), Error> {
  // parse command line arguments
  let CommandLineArgs { input, output } = CommandLineArgs::parse()?;
  // parse the input file
  let stat = parse(&input)?;
  // write the statistics to the output file
  if let Some(output) = output {
    fs::write(output, stat).map_err(Error::File)
  } else {
    println!("{}", stat);
    Ok(())
  }
}

/// Error returned by the `main` procedure.
enum Error {
  /// Invalid command line arguments.
  InvalidArgs,
  /// File error.
  File(io::Error),
  /// Parser error.
  Parser(String),
  /// Invalid UTF-8 string.
  InvalidUtf8,
  /// Serialization error.
  Serialization(SerdeError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::InvalidArgs => write!(
        f,
        r#"Usage: syan INPUT [OUTPUT]

Options:
  INPUT:  the input SysY source file, or a directory containing SysY source files
  OUTPUT: the output JSON file, containing statistics of the input files
          default to stdout"#
      ),
      Self::File(e) => write!(f, "invalid input SysY file: {}", e),
      Self::Parser(file) => write!(f, "error occurred while parsing {}", file),
      Self::InvalidUtf8 => write!(f, "invalid UTF-8 string"),
      Self::Serialization(e) => write!(f, "error occurred while serializing: {}", e),
    }
  }
}

/// Command line arguments.
struct CommandLineArgs {
  /// Input file.
  input: String,
  /// Output file.
  output: Option<String>,
}

impl CommandLineArgs {
  /// Parses the command line arguments.
  fn parse() -> Result<Self, Error> {
    let mut args = env::args();
    args.next();
    match (args.next(), args.next()) {
      (Some(input), output) => Ok(Self { input, output }),
      _ => Err(Error::InvalidArgs),
    }
  }
}

/// Parses the input file.
/// Returns the statistics in the form of a JSON string.
fn parse(input: &str) -> Result<String, Error> {
  // check if the input file is a directory
  let is_dir = fs::metadata(input).map(|m| m.is_dir()).unwrap_or(false);
  let stats = if is_dir {
    parse_dir(input)
  } else {
    parse_file(input)
  }?;
  serde_json::to_string(&stats).map_err(Error::Serialization)
}

/// Parses the given SysY program.
/// Returns statistics about the program.
fn parse_file<P: AsRef<Path>>(file: P) -> Result<Statistics, Error> {
  // read the input file
  let program = fs::read_to_string(&file).map_err(Error::File)?;
  // get the file name
  let file = file
    .as_ref()
    .to_str()
    .ok_or(Error::InvalidUtf8)?
    .to_string();
  // run the parser
  let mut stat = FileStatistics::default();
  sysy::CompUnitParser::new()
    .parse(&mut stat, &program)
    .map_err(|_| Error::Parser(file.clone()))?;
  // return the statistics
  let mut stats = Statistics::new();
  stats.add(file, stat);
  Ok(stats)
}

/// Parses all SysY programs in the given file or directory.
/// Returns statistics about the program.
fn parse_dir<P: AsRef<Path>>(dir: P) -> Result<Statistics, Error> {
  // read the directory
  let dir = fs::read_dir(dir).map_err(Error::File)?;
  // parse all files in the directory
  let mut stats = Statistics::new();
  for entry in dir {
    let path = entry.map_err(Error::File)?.path();
    if path.is_file() && path.extension().map_or(false, |e| e == "c") {
      stats.extend(parse_file(&path)?);
    } else if path.is_dir() {
      stats.extend(parse_dir(path)?);
    }
  }
  Ok(stats)
}
