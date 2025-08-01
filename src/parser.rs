// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/7.1.3/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

//use nom::*;
use crate::lexer::*;
use nom::sequence::tuple;
use nom::combinator::map;


 use nom::{
  IResult,
  branch::alt,
  combinator::opt, 
  multi::{many1, many0},
  bytes::complete::{tag},
  sequence::{delimited, preceded},
  character::complete::{char as char_completer, alphanumeric1, digit1},
};
 
// Here are the different node types. You will use these to make your parser.
// You may add other nodes as you see fit.

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionDefine {name: Vec<u8>, children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Condition {children: Vec<Node> },
  IfExpression {children: Vec<Node> },
  MultiLineIf {children: Vec<Node> },
  IfElse {children: Vec<Node> },
  IfAssign {children: Vec<Node> },
  IfIndent {children: Vec<Node> },
  ReturnIf { children: Vec<Node>},
  ReturnIfNum { children: Vec<Node>},
  ReturnElse {children: Vec<Node>},
  Expression { children: Vec<Node> },
  MathExpression {name: Vec<u8>, children: Vec<Node> },
  FunctionCall { name: Vec<u8>, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: Vec<u8> },
  Operator {value: Vec<u8>},
  String { value: String },
  Comment { value: Vec<u8> },
  NewLine,
  Tab,
  Null,
}

// Some helper functions to use Tokens instead of a &str with Nom. 
// You'll probably have to create more of these as needed.

pub fn t_alpha(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Alpha => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_digit(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Digit => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_true(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::True => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_false(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::False => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_alpha1(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many1(t_alpha)(input)
}

pub fn t_alpha0(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many0(t_alpha)(input)
}

pub fn t_alphanumeric1(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many1(alt((t_alpha,t_digit)))(input)
}

pub fn t_alphanumeric0(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many0(alt((t_alpha,t_digit,)))(input)

}

// keywords 

pub fn t_left_paren(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(&|tk| match tk.kind {
    TokenKind::LeftParen => true,
    _=> false,
  }) ;
  fxn(input.clone())
}

pub fn t_right_paren(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(&|tk| match tk.kind {
    TokenKind::RightParen => true,
    _=> false,
  }) ;
  fxn(input.clone())
}

// Helper function to parse the curly brackets
pub fn t_left_curly(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(&|tk| match tk.kind {
    TokenKind::LeftCurly => true,
    _=> false,
  }) ;
  fxn(input.clone())
}

pub fn t_right_curly(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(&|tk| match tk.kind {
    TokenKind::RightCurly => true,
    _=> false,
  }) ;
  fxn(input.clone())
}

pub fn t_quote(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Quote => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_slash(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Slash => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_comma(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(&|tk| match tk.kind {
    TokenKind::Comma => true,
    _=> false,
  }) ;
  fxn(input.clone())
}

pub fn t_semicolon(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(&|tk| match tk.kind {
    TokenKind::Semicolon => true,
    _=> false,
  }) ;
  fxn(input.clone())
}


pub fn t_let(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Let => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_if(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::If => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_greater(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Greater => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_lessthan(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::LessThan => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_double_equal(input: Tokens) -> IResult<Tokens, Token>{
  let (input, x) = t_equal(input)?;
  let (input, x) = t_equal(input)?;
   let token = Token{kind: TokenKind::DoubleEqual,
  lexeme: vec![b'=', b'='],
start_line: input.tokens[0].start_line,
start_col: input.tokens[0].start_col,
end_line: input.tokens[0].end_line,
end_col: input.tokens[0].end_col,};
Ok((input, token))
}


pub fn t_else(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Else => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_fn(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Fn => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_return(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Return => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_whitespace(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::WhiteSpace => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_tab(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Tab => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_newline(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::NewLine => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_plus(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Plus => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_dash(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Dash => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_equal(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Equal => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn identifier(input: Tokens) -> IResult<Tokens, Node> {
  let (input, first) = t_alpha(input)?;
  let (input, rest) = t_alphanumeric0(input)?;
  let mut identifier = first.lexeme;
  for mut tk in rest {
    identifier.append(&mut tk.lexeme);
  }
  Ok((input,Node::Identifier{value: identifier}))
}

pub fn number(input: Tokens) -> IResult<Tokens, Node> {
  let (input, digits) = many1(t_digit)(input)?;
  let value: Vec<u8> = digits.iter()
                             .flat_map(|token| token.lexeme.iter())
                             .cloned()
                             .collect();
  let parsed_value: i32 = std::str::from_utf8(&value).unwrap().parse::<i32>().unwrap();
  Ok((input, Node::Number { value: parsed_value }))
}

pub fn boolean(input: Tokens) -> IResult<Tokens, Node> {
  let (input, token) = alt((t_true, t_false))(input)?;
   let value = match token.kind {
      TokenKind::True => true,
      TokenKind::False => false,
      _ => unreachable!(),
  };
  Ok((input, Node::Bool { value }))
}

pub fn string(input: Tokens) -> IResult<Tokens, Node> {
 let (input, _) = t_quote(input)?;
  let (input, string) = t_alphanumeric0(input)?;
 let (input, _) = t_quote(input)?;
 let value: Vec<u8> = string.into_iter()
                               .map(|token| token.lexeme)
                               .flatten()
                               .collect();
 Ok((input, Node::String{ value: String::from_utf8(value).unwrap() }))
}

pub fn function_call(input: Tokens) -> IResult<Tokens, Node> {
  let (input, fxn_name) = identifier(input)?;
  let (input, _) = (t_left_paren)(input)?;
  let (input, args) = many0(arguments)(input)?;
  let (input, _) = (t_right_paren)(input)?;
  let args = if args.is_empty() {
    vec![Node::FunctionArguments{ children: vec![]}]
  } else {
    args
  };
  let name: Vec<u8> = match fxn_name {
    Node::Identifier{value} => value,
    _ => unreachable!(),
  }; 
  Ok((input, Node::FunctionCall{name, children: args}))
}

pub fn value(input: Tokens) -> IResult<Tokens, Node> {
  alt((number, identifier, boolean))(input)
}

pub fn math_expression(input: Tokens) -> IResult<Tokens, Node> {
  let (input, leftside) = value(input)?;
  let (input, operator) = alt((t_plus, t_dash))(input)?;
  let (input, rightside) = value(input)?;
  let name = match operator.kind {
    TokenKind::Plus => b"add",
    TokenKind::Dash => b"sub",
    _ => unreachable!(),
  };
  Ok((input, Node::MathExpression{name: name.to_vec(), children: vec![leftside, rightside] }))
}

pub fn return_if(input: Tokens) -> IResult<Tokens, Node> {
  let (input, x) = t_left_curly(input)?;
  let (input, x) = t_return(input)?;
  let (input, value) = boolean(input)?; // Parses a boolean as the return value
  let (input, x) = t_semicolon(input)?;
  let (input, x) = t_right_curly(input)?;
  Ok((input, Node::ReturnIf{children: vec![value]}))
}

pub fn return_ifnum(input: Tokens) -> IResult<Tokens, Node>{
  let (input, x) = t_left_curly(input)?;
  let (input, x) = t_return(input)?;
  let (input, num_node) = number(input)?; // Parses a digit as the return value
  let (input, x) = t_semicolon(input)?;
  let (input, x) = t_right_curly(input)?;
  Ok((input, Node::ReturnIfNum{children: vec![num_node]}))
}

pub fn return_else(input: Tokens)->IResult<Tokens, Node>{
  let (input, x) = t_left_curly(input)?;
  let (input, x) = t_return(input)?;
  let (input, num_node) = number(input)?;
  let (input, x) = t_right_curly(input)?;
  Ok((input, Node::ReturnElse{children: vec![num_node]}))
}

pub fn condition(input: Tokens) -> IResult<Tokens, Node> {
  let (input, leftside) = identifier(input)?;
  let (input, operator) = alt((t_greater, t_lessthan, t_double_equal))(input)?;
  let (input, rightside) = identifier(input)?;

  Ok((input, Node::Condition {children: vec![leftside, Node::Operator{value: operator.lexeme}, rightside] }))
}

pub fn if_indent(input: Tokens) -> IResult<Tokens, Node> {
  let (input, first) = t_newline(input)?;
  let (input, second) = t_tab(input)?;
  Ok((input, Node::IfIndent{children: vec![Node::NewLine{}, Node::Tab{}]}))
}


pub fn if_expression(input: Tokens) -> IResult<Tokens, Node> {
  let (input, x) = t_if(input)?;
  let (input, condition) = condition(input)?;
  let (input, val1) = return_if(input)?;
  let (input, x) = t_else(input)?;
  let (input, val2) = return_if(input)?;
  Ok((input, Node::IfExpression{children: vec! [condition, val1, val2]}))
}


pub fn if_multi_line(input: Tokens) ->IResult<Tokens, Node>{
  let (input, x) = t_if(input)?;
  let (input, boolean1) = boolean(input)?;
  let (input, x) = t_left_curly(input)?;
  let (input, val1) = preceded(if_indent, return_if)(input)?;
  let (input, x) = t_right_curly(input)?;
  let (input, x) = t_else(input)?;
  let (input, x) = t_left_curly(input)?;
  let (input, val2) = preceded(if_indent, return_if)(input)?;
  let (input, x) = t_right_curly(input)?;
  Ok((input, Node::MultiLineIf{children: vec! [boolean1, val1, val2]}))
}

pub fn if_else(input: Tokens) ->IResult<Tokens, Node>{
  let (input, x) = t_if(input)?;
  let (input, condition1) = condition(input)?;
  let (input, val1) = return_ifnum(input)?;
  let (input, x) = t_else(input)?;
  let (input, x) = t_if(input)?;
  let (input, condition2) = condition(input)?;
  let (input, val2) = return_ifnum(input)?;
  let (input, x) = t_else(input)?;
  let (input, val3) = return_else(input)?;
  
  Ok((input, Node::IfElse{children: vec! [condition1, val1, condition2, val2, val3]}))
}

pub fn if_assign(input: Tokens)->IResult<Tokens, Node>{
  let (input, x) = t_let(input)?;
  let (input, variable) = identifier(input)?;
  let (input, x) = t_equal(input)?;
  let (input, x) = t_if(input)?;
  let (input, boolean1) = boolean(input)?;
  let (input, val1) = return_if(input)?;
  let (input, x) = t_else(input)?;
  let (input, val2) = return_if(input)?;

  Ok((input, Node::IfAssign{children: vec! [variable, boolean1, val1, val2]}))
}

pub fn expression(input: Tokens) -> IResult<Tokens, Node> {
   let (input, result) =  alt((boolean, math_expression, function_call, number, string,identifier))(input)?;
   Ok((input, Node::Expression{children: vec! [result]}))
}

pub fn statement(input: Tokens) -> IResult<Tokens, Node> {
  let (input, result) = alt((variable_define, expression, function_return))(input)?;
  let (input, _) = (t_semicolon)(input)?;
  Ok((input, result))
}

pub fn function_return(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = t_return(input)?;
  let (input, result) = alt((function_call, expression, identifier))(input)?;
  Ok((input, Node::FunctionReturn{children: vec! [result]}))
}

pub fn variable_define(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = t_let(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = (t_equal)(input)?;
  let (input, choice) = alt((expression, if_assign))(input)?;
  Ok((input, Node::VariableDefine{children: vec![variable,choice]}))
}

pub fn arguments(input: Tokens) -> IResult<Tokens, Node> {
  let (input, arg) = expression(input)?;
  let (input, mut others) = many0(other_arg) (input)?;
  let mut args = vec! [arg];
  args.append (&mut others) ;
  Ok((input, Node::FunctionArguments{children: args}))
}


pub fn other_arg(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = t_comma(input)?;
  expression(input)
}

pub fn function_define(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = t_fn(input)?;
  let (input, fxn_name) = identifier(input)?;
  let name = match fxn_name {
    Node::Identifier{value} => value,
    _ => unreachable!(),
  };
  let (input, _) = t_left_paren(input)?;
  let (input, args) = many0(arguments)(input)?;
  let (input, _) = t_right_paren(input)?;
  let (input, _) = t_left_curly(input)?;
  let (input, statements) = many1(statement)(input)?;
  let (input, _) = t_right_curly(input)?;
  let fxn_statements = Node::FunctionStatements{children: statements};
  let fxn_arguments = if args.is_empty() {
    Node::FunctionArguments{children: vec![]}
  } else {
    args[0].clone()
  };
  Ok((input, Node::FunctionDefine{name, children: vec![fxn_arguments,fxn_statements] }))
}

pub fn comment(input: Tokens) -> IResult<Tokens, Node> {
  let mut comment_text = Vec::new();
  let (input, _) = t_slash(input)?;
  let (input, _) = t_slash(input)?;
  let (input, alpha_tokens) = many0(t_alpha)(input)?;
  for token in alpha_tokens {
    comment_text.extend_from_slice(&token.lexeme);
  }
  Ok((input, Node::Comment{ value: comment_text }))
}

pub fn program(input: Tokens) -> IResult<Tokens, Node> {
  let (input, result) = many1(alt((function_define,expression,statement,string,boolean,number)))(input)?;
  Ok((input, Node::Program{ children: result }))
}
