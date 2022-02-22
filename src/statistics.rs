use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Statistics table.
#[derive(Serialize, Default)]
pub struct Statistics {
  statistics: BTreeSet<StatisticsEntry>,
}

impl Statistics {
  /// Creates a new statistics table.
  pub fn new() -> Self {
    Self::default()
  }

  /// Adds a new entry to the statistics table.
  pub fn add(&mut self, file: String, stat: FileStatistics) {
    self.statistics.insert(StatisticsEntry { file, stat });
  }

  /// Extends the statistics table with the given statistics.
  pub fn extend(&mut self, stats: Statistics) {
    self.statistics.extend(stats.statistics);
  }
}

/// Entry of a statistics table.
#[derive(Serialize)]
pub struct StatisticsEntry {
  /// File name.
  file: String,
  /// Statistics.
  stat: FileStatistics,
}

impl PartialEq for StatisticsEntry {
  fn eq(&self, other: &Self) -> bool {
    self.file == other.file
  }
}

impl Eq for StatisticsEntry {}

impl PartialOrd for StatisticsEntry {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.file.cmp(&other.file))
  }
}

impl Ord for StatisticsEntry {
  fn cmp(&self, other: &Self) -> Ordering {
    self.file.cmp(&other.file)
  }
}

/// Statistics of a SysY source file.
#[derive(Default, Serialize)]
pub struct FileStatistics {
  /// Lexical statistics.
  pub lexical: LexicalStatistics,
  /// Grammar statistics.
  pub grammar: GrammarStatistics,
}

/// Lexical statistics.
#[derive(Default, Serialize)]
pub struct LexicalStatistics {
  /// Number of identifiers.
  pub ids: usize,
  /// Number of decimal integers.
  pub decimals: usize,
  /// Number of octal integers.
  pub octals: usize,
  /// Number of hexadecimal integers.
  pub hexadecimals: usize,
  /// Number of unary operators.
  pub unary_ops: usize,
  /// Number of binary operators.
  pub binary_ops: usize,
}

/// Grammar statistics.
#[derive(Default, Serialize)]
pub struct GrammarStatistics {
  /// Number of constant declarations.
  pub const_decls: usize,
  /// Number of constant definitions.
  pub const_defs: usize,
  /// Number of constant array definitions.
  pub const_array_defs: usize,
  /// Number of expression initial value in constant definitions.
  pub const_init_exprs: usize,
  /// Number of list initial value in constant definitions.
  pub const_init_lists: usize,
  /// Number of variable declarations.
  pub var_decls: usize,
  /// Number of variable definitions.
  pub var_defs: usize,
  /// Number of variable array definitions.
  pub var_array_defs: usize,
  /// Number of expression initial value in variable definitions.
  pub var_init_exprs: usize,
  /// Number of list initial value in variable definitions.
  pub var_init_lists: usize,
  /// Number of void function definitions.
  pub void_func_defs: usize,
  /// Number of integer function definitions.
  pub int_func_defs: usize,
  /// Number of function parameters.
  pub func_params: usize,
  /// Number of array function parameters.
  pub func_array_params: usize,
  /// Maximum number of all function parameters.
  pub func_params_max: usize,
  /// Number of local declarations.
  pub local_decls: usize,
  /// Number of assignment statements.
  pub assign_stmts: usize,
  /// Number of expression statements.
  pub expr_stmts: usize,
  /// Number of block statements.
  pub block_stmts: usize,
  /// Number of if statements.
  pub if_stmts: usize,
  /// Number of if-else statements.
  pub if_else_stmts: usize,
  /// Number of while statements.
  pub while_stmts: usize,
  /// Number of break statements.
  pub break_stmts: usize,
  /// Number of continue statements.
  pub continue_stmts: usize,
  /// Number of return statements.
  pub return_stmts: usize,
  /// Number of array accesses.
  pub array_accesses: usize,
  /// Number of parenthesized expressions.
  pub paren_exprs: usize,
  /// Number of function calls.
  pub func_calls: usize,
  /// Number of unary expressions.
  pub unary_exprs: usize,
  /// Number of multiplication expressions.
  pub mul_exprs: usize,
  /// Number of addition expressions.
  pub add_exprs: usize,
  /// Number of relational expressions.
  pub rel_exprs: usize,
  /// Number of equality expressions.
  pub eq_exprs: usize,
  /// Number of logical AND expressions.
  pub land_exprs: usize,
  /// Number of logical OR expressions.
  pub lor_exprs: usize,
  /// Number of constant expressions.
  pub const_exprs: usize,
}
