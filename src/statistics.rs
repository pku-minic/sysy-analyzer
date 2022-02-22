use rlua::{Context, Result, ToLua, Value};

/// Statistics about a SysY program.
#[derive(Default)]
pub struct Statistics {
  /// Lexical statistics.
  pub lexical: LexicalStatistics,
  /// Grammar statistics.
  pub grammar: GrammarStatistics,
}

impl<'lua> ToLua<'lua> for Statistics {
  /// Converts a `Statistics` to a Lua table.
  fn to_lua(self, lua: Context<'lua>) -> Result<Value<'lua>> {
    let table = lua.create_table()?;
    table.set("lexical", self.lexical)?;
    table.set("grammar", self.grammar)?;
    Ok(Value::Table(table))
  }
}

/// Lexical statistics.
#[derive(Default)]
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

impl<'lua> ToLua<'lua> for LexicalStatistics {
  /// Converts a `LexicalStatistics` to a Lua table.
  fn to_lua(self, lua: Context<'lua>) -> Result<Value<'lua>> {
    let table = lua.create_table()?;
    table.set("ids", self.ids)?;
    table.set("decimals", self.decimals)?;
    table.set("octals", self.octals)?;
    table.set("hexadecimals", self.hexadecimals)?;
    table.set("unary_ops", self.unary_ops)?;
    table.set("binary_ops", self.binary_ops)?;
    Ok(Value::Table(table))
  }
}

/// Grammar statistics.
#[derive(Default)]
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

impl<'lua> ToLua<'lua> for GrammarStatistics {
  /// Converts a `GrammarStatistics` to a Lua table.
  fn to_lua(self, lua: Context<'lua>) -> Result<Value<'lua>> {
    let table = lua.create_table()?;
    table.set("const_decls", self.const_decls)?;
    table.set("const_defs", self.const_defs)?;
    table.set("const_array_defs", self.const_array_defs)?;
    table.set("const_init_exprs", self.const_init_exprs)?;
    table.set("const_init_lists", self.const_init_lists)?;
    table.set("var_decls", self.var_decls)?;
    table.set("var_defs", self.var_defs)?;
    table.set("var_array_defs", self.var_array_defs)?;
    table.set("var_init_exprs", self.var_init_exprs)?;
    table.set("var_init_lists", self.var_init_lists)?;
    table.set("void_func_defs", self.void_func_defs)?;
    table.set("int_func_defs", self.int_func_defs)?;
    table.set("func_params", self.func_params)?;
    table.set("func_array_params", self.func_array_params)?;
    table.set("func_params_max", self.func_params_max)?;
    table.set("local_decls", self.local_decls)?;
    table.set("assign_stmts", self.assign_stmts)?;
    table.set("expr_stmts", self.expr_stmts)?;
    table.set("block_stmts", self.block_stmts)?;
    table.set("if_stmts", self.if_stmts)?;
    table.set("if_else_stmts", self.if_else_stmts)?;
    table.set("while_stmts", self.while_stmts)?;
    table.set("break_stmts", self.break_stmts)?;
    table.set("continue_stmts", self.continue_stmts)?;
    table.set("return_stmts", self.return_stmts)?;
    table.set("array_accesses", self.array_accesses)?;
    table.set("paren_exprs", self.paren_exprs)?;
    table.set("func_calls", self.func_calls)?;
    table.set("unary_exprs", self.unary_exprs)?;
    table.set("mul_exprs", self.mul_exprs)?;
    table.set("add_exprs", self.add_exprs)?;
    table.set("rel_exprs", self.rel_exprs)?;
    table.set("eq_exprs", self.eq_exprs)?;
    table.set("land_exprs", self.land_exprs)?;
    table.set("lor_exprs", self.lor_exprs)?;
    table.set("const_exprs", self.const_exprs)?;
    Ok(Value::Table(table))
  }
}
