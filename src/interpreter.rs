use crate::expr::LiteralValue;
use crate::stmt::Stmt;
use crate::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::SystemTime;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

fn time_impl(_args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f64 / 1000.0)
}

fn print_impl(args: &Vec<LiteralValue>) -> LiteralValue {
    print!("{}", args[0].to_string());

    LiteralValue::Null
}

fn println_impl(args: &Vec<LiteralValue>) -> LiteralValue {
    println!("{}", args[0].to_string());

    LiteralValue::Null
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();

        globals.define(
            String::from("time"), LiteralValue::Callable {
            name: "time".to_string(),
            arity: 0,
            fn_: Rc::new(time_impl)
        });

        globals.define(
            String::from("print"), LiteralValue::Callable {
            name: "print".to_string(),
            arity: 1,
            fn_: Rc::new(print_impl)
        });

        globals.define(
            String::from("println"), LiteralValue::Callable {
            name: "println".to_string(),
            arity: 1,
            fn_: Rc::new(println_impl)
        });

        Self {
            environment: Rc::new(RefCell::new(globals)),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                },
                Stmt::Echo { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;

                    println!("{}", value.to_string());
                },
                Stmt::Let { name, initializer } => {
                    let value = initializer.evaluate(self.environment.clone())?;

                    self.environment.borrow_mut().define(name.lexeme.clone(), value)
                },
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result =  self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;

                    block_result?
                },
                Stmt::If { predicate, then, otherwise } => {
                    let truth_value = predicate.evaluate(self.environment.clone())?;

                    if truth_value.is_truthy() == LiteralValue::True {
                        let statements = vec![then.as_ref()];
                        self.interpret(statements)?;
                    } else if let Some(else_stmt) = otherwise {
                        let statements = vec![else_stmt.as_ref()];
                        self.interpret(statements)?;
                    }
                },
                Stmt::While { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;

                    while flag.is_truthy()  == LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                }
            };
        }
        Ok(())
    }
}