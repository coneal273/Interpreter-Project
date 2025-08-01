use crate::parser::Node;
use std::collections::HashMap;
use crate::error::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

type Frame = HashMap<String, Value>;
type Arguments = Node;
type Statements = Node;

#[derive(Debug)]
pub struct Interpreter {
  // Function Table:
  // Key - Function name
  // Value - Vec<Node> arguments, statements
  functions: HashMap<String, (Arguments,Statements)>,
  // Stack:
  // Each element in the stack is a function stack frame.
  // Crate a new stack frame on function entry.
  // Pop stack frame on function return.
  // Key - Variable name
  // Value - Variable value
  stack: Vec<Frame>,
}


impl Interpreter {

  pub fn new() -> Interpreter {
    Interpreter {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  pub fn exec(&mut self, node: &Node) -> Result<Value,AsaErrorKind> {
    match node {
      Node::Program{children} => {
        let mut return_val = Err(AsaErrorKind::NoRun) ;
        for n in children {
          match n {
            Node::FunctionDefine{..} |
            Node::Expression{..} |
            Node::VariableDefine{..} |
            Node::String{..} |
            Node::Number{..} |
            Node::Bool{..} => {
              return_val = Ok (self.exec(n)?) ;
            }
            _ => unreachable!(),
          }
        }
       return_val 
      },
      // Evaluates a mathematical expression based on the elements in the children argument. If the expression is valid, the code evaluates it and returns a new Value object with the resulting value. If the expression is not valid, the code returns an error message.
      Node::MathExpression{name, children} => {
        let leftside: Value = self.exec(&children[0])?;
        let rightside: Value = self.exec(&children[1])?;
          match String:: from_utf8(name.to_vec()).unwrap().as_str() {
            "add" => {
              match (leftside, rightside){
                (Value::Number(lv), Value::Number(rv)) =>  Ok(Value::Number(lv + rv)),
                _ => Err(AsaErrorKind::UndefinedFunction) 
              }
            },
            "sub" => {
              match (leftside, rightside){
                (Value::Number(lv), Value::Number(rv)) =>  Ok(Value::Number(lv - rv)),
                _ => Err(AsaErrorKind::UndefinedFunction) 
              }
            },
            _ => Err(AsaErrorKind::UndefinedFunction),
        }        
      },
      // Defines a function that takes some arguments and executes a program based on those arguments. The code first checks if the function exists, and if it does, it creates a new scope in which to execute the function's statements (push a new Frame onto the interpreter stack). The code then executes each statement in the function's statements list and returns the result of the function's execution. You will have to correlate each passed value with the apprpriate variable in the called function. If the wrong number or an wrong type of variable is passed, return an error. On success, insert the return value of the function (if any) into the appropriate entry of the caller's stack.
      Node::FunctionCall{name, children} => {
        let func_name = String::from_utf8_lossy(&name).into_owned();// extract function name and arguments
        let (args,stmts) = if let Some((Node::FunctionArguments { children: args }, Node::FunctionStatements { children: stmts })) = self.functions.get(&func_name) {
            (args.clone(),stmts.clone())
        } else {    
            
            return Err(AsaErrorKind::UndefinedFunction);
        };
        let call_args = if let Some(Node::FunctionArguments { children: args }) = &children.last() {
            args.clone()
        } else {
            Vec::new()
        };
        let mut new_frame = Frame::new();
        for (arg_node, arg_value) in call_args.iter().zip(args.iter()) {
            let result =  self.exec(&arg_node)?;
            let arg_name = match arg_value {
                Node::Expression { children } => {
                    match &children[0] {
                        Node::Identifier { value } => {
                            String::from_utf8_lossy(&value).into_owned()
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            };
            new_frame.insert(arg_name, result);
        }
        self.stack.push(new_frame);
        let mut result = Err(AsaErrorKind::NoRun) ; 
        for stmt in stmts {
           result = Ok(self.exec(&stmt)?);
        }
        self.stack.pop();
        result
      },
      // Defines a new function based on the elements in the children argument. The name of the function is retrieved from the node struct, the arguments are the first child, and the statements that define the function are the second child. A new key-value pair is then inserted into the functions table of the interprer. If the function was successfully defined, the code returns a Value object with a boolean value of true, otherwise an error is returned.
      Node::FunctionDefine{name, children} => {
        let function_name = String::from_utf8_lossy(&name).into_owned();
        let args = match &children[0] {
            Node::FunctionArguments {..} => children[0].clone(),
            _ => unreachable!(),
        };
        let stmts = match &children[1] {
            Node::FunctionStatements {..} => children[1].clone(),
            _ => unreachable!(),
        };
        self.functions.insert(function_name.clone(), (args, stmts) );
        Ok(Value::Bool(true))
      },
      // Calls the exec() method on the first element in the children argument, which recursively evaluates the AST of the program being executed and returns the resulting value or error message.
      Node::FunctionReturn{children} => {
        Ok(self.exec(&children[0])?)
      },
      // Retrieves the value of the identifier from the current frame on the stack. If the variable is defined in the current frame, the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
        let value_str = String::from_utf8_lossy(value).into_owned();
        match self.stack.last() {
            Some(frame) => {
                if let Some(val) = frame.get(&value_str) {
                    return Ok(val.clone());
                } else {
                    return Err(AsaErrorKind::UndefinedVariable)
                }
            }
            _ => {
                return Err(AsaErrorKind::UndefinedVariable)
            }
        }
      },
      // Checks the type of the first element in the children argument and deciding what to do based on that type. If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => {
        match children[0] {
            Node::VariableDefine { .. } |
            Node::FunctionReturn { .. } => {
                self.exec(&children[0])
            },
            _ => unreachable!(),
        }
      },
      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, and the value is retrieved by running the run method on the second element of the children argument. The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        let variable_identifier = &children[0];
        let value = self.exec(&children[1])?;
        let var_name = match variable_identifier {
            Node::Identifier {value} => {
                String::from_utf8_lossy(value).into_owned()
            },
            _ => unreachable!(),
        };
        if let Some(frame) = self.stack.last_mut() {
            frame.insert(var_name.clone(), value.clone());
            Ok(value)
        } else {
            let mut frame = Frame::new();
            frame.insert(var_name.clone(), value.clone());
            self.stack.push(frame);
            Ok(value)
        }
      },
      // Evaluate the child node using the exec() method.
      Node::Expression{children} => {
        self.exec(&children[0])
      }
      Node::Number{value} => {
        Ok(Value::Number(*value))
      }
      Node::String{value} => {
        Ok(Value::String(value.clone()))
      }
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      },

      Node::IfExpression{children} => {
        let condition = self.exec(&children[0])?;
        match condition{
            Value::Bool(true) => { self.exec(&children[1])?;
            Ok(Value::Bool(true))
            }
            Value::Bool(false) => { self.exec(&children[2])?;
            Ok(Value::Bool(false))
            }
            _ => Err(AsaErrorKind::TypeError),
        }
      },
      
      Node::IfElse{children} => {
        let condition = self.exec(&children[0])?;
        match condition {
            Value::Bool(true) => { self.exec(&children[1])?;
                Ok(Value::Bool(true))
            }
            Value::Bool(false) => {
                let condition2 = self.exec(&children[2])?;
                match condition2 {
                    Value::Bool(true) => { self.exec(&children[3])?;
                    Ok(Value::Bool(true))
                    }
                    Value::Bool(false) => { self.exec(&children[4])?;
                        Ok(Value::Bool(false))
                    }
                    _ => Err(AsaErrorKind::TypeError),
                }
            },
            _ => Err(AsaErrorKind::TypeError),
        }
      },

      Node::IfAssign{children} => {
        let var_name = match &children[0] {
            Node::Identifier{value} => String::from_utf8_lossy(&value).into_owned(),
            _ => return Err(AsaErrorKind::TypeError),
        };
        let condition = self.exec(&children[1])?;
        let value = match condition {
            Value::Bool(true) => self.exec(&children[2])?,
            Value::Bool(false) => self.exec(&children[3])?,
            _ => return Err(AsaErrorKind::TypeError),
        };

        if let Some(frame) = self.stack.last_mut() {
            frame.insert(var_name, value.clone());
        }else{
            let mut frame = Frame::new();
            frame.insert(var_name, value.clone());
            self.stack.push(frame);
        }


        let result = if let Value::Bool(true) = condition {
            self.exec(&children[2])?
        }else{
            self.exec(&children[3])?
        };
      Ok(value)
    },

      Node::MultiLineIf{children} => {
        let condition = self.exec(&children[0])?;
        match condition {
            Value::Bool(true) => {
                if let Node::FunctionStatements{children: then_branch} = &children[1] {
                    for stmt in then_branch {
                        self.exec(stmt)?;
                        }
                    }
                    Ok(Value::Bool(true))
                }
            Value::Bool(false) => {
                if let Node::FunctionStatements{children: else_branch} = &children[2] {
                    for stmt in else_branch{
                    self.exec(stmt)?;
                    }
                }
                Ok(Value::Bool(false))
            }
            _ => Err(AsaErrorKind::TypeError),
        }
      }
      // Return an error message.
      x => {
        unimplemented!();
      },
    }
  }

  pub fn start_main(&mut self, arguments: Vec<Node>) -> Result<Value,AsaErrorKind> {
    // This node is equivalent to the following Asa program source code:
    // "main()"
    // It calls the main function with a FunctionArguments node as input.
    let start_main = Node::FunctionCall{name: "main".into(), children: arguments};
    // Call the main function by running this code through the interpreter. 
    self.exec(&start_main)
  }
}
