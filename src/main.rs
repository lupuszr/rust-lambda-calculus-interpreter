use std::{rc::Rc, collections::HashMap, cell::RefCell, ops::Deref};

// type R<A> = Rc<RefCell<A>>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct R<A> {
    value: Rc<Box<A>>
}

impl<A> R<A> {
    fn new(a: A) -> R<A> {
        // let x = Rc::new(RefCell::new(a));
        let x = Rc::new(Box::new((a)));
        R {
            value: x
        }
    }
}




#[derive(Debug, Clone, PartialEq, Eq)]
enum Expression {
    Variable(String),
    Abstraction {
        param: String,
        body: R<Expression>
    },
    Application {
        func: R<Expression>,
        argument: R<Expression>
    }
}

impl Expression {
    pub fn new_abstraction(param: String, body: Expression) -> Expression {
        Expression::Abstraction { param, body: R::new(body)}
    }
}

type Context<A> = HashMap<String, A>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
        Closure {
            context: Context<R<Value>>,
            param: String,
            body: R<Expression>
        },
        Native(fn(R<Value>) -> R<Value>)
}

impl From<Expression> for R<Expression> {
    fn from(e: Expression) -> Self {
        R {
            value: Rc::new(Box::new(e))
        }
    }
}

impl Deref for R<Value> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        let k = &(*self.value);
        &*k
    }
}

// impl <'a> Deref for R<Expression> {
//     type Target = Expression;

//     fn deref(&self) -> &Self::Target {
//         let z = self.value;

//     }
// }

fn interpret<'a>(context: Context<R<Value>>, expression: R<Expression>) -> R<Value>{
    let e =  expression.value.as_ref().clone();
    
    match *e {
        Expression::Variable(name) => {
            let x = context.get(&name).unwrap();
            x.clone()
        },
        Expression::Abstraction { param, body } => {
            R::new(Value::Closure { context, param: param.clone(), body: body.clone() })
        },
        Expression::Application { func, argument } => {
            // let a = argument.clone();
            // let mut q: vec<&mut Box<Value>> = Vec::new();

             

            let argument = interpret(context.clone(), argument.clone());
            let closure = interpret(context, func.clone());
            let clo = &*closure;
            match clo.clone() {
                Value::Closure { mut context, param, body } => {
                    let _old_val = context.insert(param.clone(), argument.clone());
                    interpret(context, body.clone())
                },
                Value::Native(f) => f(argument),
            }
        },
    }

}
// fn interpret_REC<'a>(context: Context<'a, R<Value>>, expression: R<Expression>) -> R<Value>{
//     match &*expression{
//         Expression::Variable(name) => {
//             let v = *name;
//             let x = context.get(v).unwrap();
//             x.clone()
//         },
//         Expression::Abstraction { param, body } => {
//             R::new(Value::Closure { context, param: param.clone(), body: body.clone() })
//         },
//         Expression::Application { func, argument } => {
//             // let a = argument.clone();
//             let argument = interpret(context.clone(), argument.clone());
//             let closure = interpret(context, func.clone());
//             match *closure {
//                 Value::Closure { mut context, param, body } => {
//                     let _old_val = context.insert(param, argument.clone());
//                     interpret(context, body)
//                 },
//                 Value::Native(f) => f(argument),
//             }
//         },
//     }

// }

struct S<'a> {
    v: &'a str
}

impl<'a> From<S<'a>> for String {
    fn from(f: S<'a>) -> Self {
        f.v.to_string()
    }
}

fn str<'a>(t: &'a str) -> String {
    t.to_string()
}

fn initial_context<'a>() -> Context<R<Value>> {
    use Expression::{Abstraction, Application, Variable};
    use Value::{Native};

    let mut c: Context<R<Value>> = HashMap::new();
    c.insert(
        "print_hello_world".to_string(), 
        R::new(Native(|v| {
            println!("hello world");
            v
        }))
    );
    c
}
 
 fn main() {
    use Expression::{Abstraction, Application, Variable};
    let f = Abstraction { 
        param: str("f"), 
        body: R::new(Application { func: R::new(Variable(str("f"))), argument: R::new(Application { func: R::new(Variable(str("print_hello_world"))), argument: R::new(Variable(str("f")))}) })
    };

    let code = Application { func: R::new(f.clone()), argument: R::new(f.clone()) };
    let _x  = interpret(initial_context(), R::new(code));
}
 