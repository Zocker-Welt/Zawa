use crate::tokenizer::Token;
use crate::expr::LiteralValue;
use crate::stmt::Stmt;
use crate::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    specials: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    should_break: bool,
}

fn time_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f64 / 1000.0)
}

fn print_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    print!("{}", args[0].to_string());

    LiteralValue::Null
}

fn println_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    println!("{}", args[0].to_string());

    LiteralValue::Null
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();

        env.define(
            String::from("time"), LiteralValue::Callable {
            name: "time".to_string(),
            arity: 0,
            fn_: Rc::new(time_impl)
        });

        env.define(
            String::from("print"), LiteralValue::Callable {
            name: "print".to_string(),
            arity: 1,
            fn_: Rc::new(print_impl)
        });

        env.define(
            String::from("println"), LiteralValue::Callable {
            name: "println".to_string(),
            arity: 1,
            fn_: Rc::new(println_impl)
        });

        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(env)),
            should_break: false
        }
    }

    fn for_closure(parent_env: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent_env);

        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: environment,
            should_break: false
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

                    block_result?;
                    if self.should_break {
                        return Ok(());
                    }
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
                    if self.should_break {
                        return Ok(());
                    }
                },
                Stmt::While { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;

                    while flag.is_truthy()  == LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        if self.should_break {
                            self.should_break = false;
                            break;
                        }
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                },
                Stmt::Break => {
                    self.should_break = true;
                    return Ok(());
                },
                Stmt::Function { name, params, body } => {
                    let arity = params.len();

                    let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                    let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                    
                    let name_clone = name.lexeme.clone();
                    let function_impl = move |parent_env_rc: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>| {
                        let mut clos_int = Interpreter::for_closure(parent_env_rc);
                        
                        for (i, arg) in args.iter().enumerate() {
                            clos_int
                                .environment
                                .borrow_mut()
                                .define(params[i].lexeme.clone(), (*arg).clone());
                        }

                        for i in 0..(body.len()) {
                            clos_int
                                .interpret(vec![body[i].as_ref()])
                                .expect(&format!("Evaluating failed inside {}", name_clone));
                            if let Some(value) = clos_int.specials.borrow().get("return") {
                                return  value;
                            }
                        }

                        LiteralValue::Null
                    };
                     
                    let callable = LiteralValue::Callable {
                        name: name.lexeme.clone(),
                        arity: arity,
                        fn_: Rc::new(function_impl)
                    };

                    self.environment.borrow_mut().define(name.lexeme.clone(), callable);
                },
                Stmt::Return { keyword: _, value } => {
                    let eval_val;

                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = LiteralValue::Null;
                    }

                    self.specials
                        .borrow_mut()
                        .define_top_level(String::from("return"), eval_val)
                }
            };
            
            if self.should_break {
                return Ok(());
            }
        }
        Ok(())
    }
}