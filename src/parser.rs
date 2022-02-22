use crate::error::Error;
use crate::statistics::Statistics;
use lalrpop_util::lalrpop_mod;

lalrpop_mod! {
  #[allow(clippy::all)]
  sysy
}

/// Parses the given SysY program.
/// Returns statistics about the program.
pub fn parse(program: &str) -> Result<Statistics, Error> {
  let mut stats = Statistics::default();
  sysy::CompUnitParser::new()
    .parse(&mut stats, program)
    .map_err(|_| Error::Parser)?;
  Ok(stats)
}
