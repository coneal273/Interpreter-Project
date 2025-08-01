#[derive(Debug,PartialEq)]
pub enum AsaErrorKind {
  UndefinedFunction,
  VariableNotDefined(String),
  TypeError,
  DivisionByZero,
  NumberOverflow,
  NumberUnderflow,
  StackError,
  UndefinedVariable,
  NoRun,
  Generic(String),  
}
