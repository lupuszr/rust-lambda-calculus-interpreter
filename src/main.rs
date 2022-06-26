use std::{rc::Rc, collections::HashMap, cell::RefCell};

// type R<A> = Rc<RefCell<A>>;
type R<A> = Box<A>;

fn new_box<A>(a: A) -> R<A> {
    // let x = Rc::new(RefCell::new(a));
    let x = Box::new((a));
    x as R<A>
}

#[derive(Debug, Clone)]
enum Expression<'a> {
    Variable(&'a str),
    Abstraction {
        param: &'a str,
        body: R<Expression<'a>>
    },
    Application {
        func: R<Expression<'a>>,
        argument: R<Expression<'a>>
    }
}

impl<'a> Expression<'a> {
    pub fn new_abstraction(param: &'a str, body: Expression<'a>) -> Expression<'a> {
        Expression::Abstraction { param, body: new_box(body)}
    }
}

type Context<'a, A> = HashMap<&'a str, A>;

#[derive(Debug, Clone)]
enum Value<'a> {
        Closure {
            context: Context<'a, R<Value<'a>>>,
            param: &'a str,
            body: R<Expression<'a>>
        },
        Native(fn(R<Value<'a>>) -> R<Value<'a>>)
}

fn interpret<'a>(context: Context<'a, R<Value<'a>>>, expression: R<Expression<'a>>) -> R<Value<'a>>{
    match &*expression{
        Expression::Variable(name) => {
            let v = *name;
            let x = context.get(v).unwrap();
            x.clone()
        },
        Expression::Abstraction { param, body } => {
            new_box(Value::Closure { context, param: param.clone(), body: body.clone() })
        },
        Expression::Application { func, argument } => {
            // let a = argument.clone();
            let argument = interpret(context.clone(), argument.clone());
            let closure = interpret(context, func.clone());
            match *closure {
                Value::Closure { mut context, param, body } => {
                    let _old_val = context.insert(param, argument.clone());
                    interpret(context, body)
                },
                Value::Native(f) => f(argument),
            }
        },
    }

}

fn initial_context<'a>() -> Context<'a, Box<Value<'a>>> {
    use Expression::{Abstraction, Application, Variable};
    use Value::{Native};

    let mut c: Context<'a, Box<Value>> = HashMap::new();
    c.insert(
        "print_hello_world", 
        Box::new(Native(|v| {
            println!("hello world");
            v
        }))
    );
    c
}
 
 fn main() {
    use Expression::{Abstraction, Application, Variable};
    let f = Abstraction { 
        param: "f", 
        body: Box::new(Application { func: Box::new(Variable("f")), argument: Box::new(Application { func: Box::new(Variable("print_hello_world")), argument: Box::new(Variable("f"))}) })
    };

    let code = Application { func: Box::new(f.clone()), argument: Box::new(f.clone()) };
    let _x  = interpret(initial_context(), Box::new(code));
}
 