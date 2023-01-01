use crate::ast::{env::Env, object::Object, symbol::Symbol, number::Number};

pub fn default_env() -> Env {
    let mut env = Env::new();
    env.push(
        Symbol::from("+"),
        Object::NativeFn(|env, args| {
            Ok(Object::Num(sum_num_list(args)?))
        }),
    );
    env.push(
        Symbol::from("-"),
        Object::NativeFn(|env, args| {
            Ok(Object::Num(sub_num_list(args)?))
        }),
    );
    env.push(
        "*".to_string(),
        Object::Fn(|args: &[Object]| -> Result<Object, String> {
            Ok(Object::Atom(Atom::Lit(Lit::Num(mul_num_list(args)?))))
        }),
    );
    env.push(
        "/".to_string(),
        Object::Fn(|args: &[Object]| -> Result<Object, String> {
            Ok(Object::Atom(Atom::Lit(Lit::Num(quo_num_list(args)?))))
        }),
    );
    env.push(
        "let".to_string(),
        Object::Fn(|args: &[Object]| -> Result<Object, String> {
            Ok(Object::Atom(Atom::Lit(Lit::Num(sum_num_list(args)?))))
        }),
    );
    env.push(
        "mod".to_string(),
        Object::Fn(|args: &[Object]| -> Result<Object, String> {
            Ok(Object::Atom(Atom::Lit(Lit::Num(mod_num_list(args)?))))
        }),
    );
    env.push("fn".to_string(), Object::Fn(|args: &[Object]| -> Result<Object, String> {
        if !(2..4).contains(&args.len()) {
            return Err("not enough arguments for function declaration".to_string());
        }
        let params = &args[0];
        let body = &args[1];
        let mut fn_args; 
        if args.len() == 3 {
            fn_args = &args[2];
        }
        todo!()
    }));
    env
}

fn sum_num_list(args: Vec<Object>) -> Result<Number, String> {
    args.iter()
        .map(|s| -> Result<Number, String> {
            match s {
                Object::Num(n) => Ok(*n),
                _ => Err(String::from("error converting arguments to numbers")),
            }
        })
        .sum()
}

fn sub_num_list(args: Vec<Object>) -> Result<Number, String> {
    let first = match args[0] {
        Object::Num(n) => n,
        _ => Err(String::from("error converting sub arguments to numbers"))?
    };

    Ok(first - sum_num_list(args[1..].to_vec())?)
}

fn mul_num_list(args: Vec<Object>) -> Result<Number, String> {
    args.iter()
        .map(|s| -> Result<Number, String> {
            match s {
                Object::Num(n) => Ok(*n),
                _ => Err(String::from("error converting mul arguments to numbers"))?
            }
        })
        .product()
}

fn quo_num_list(args: Vec<Object>) -> Result<Number, String> {
    let first = match args[0] {
        Object::Num(n) => n,
        _ => Err(String::from("error converting quo arguments to numbers"))?
    };

    Ok(first / mul_num_list(args[1..].to_vec())?)
}

fn mod_num_list(args: &[Object]) -> Result<Number, String> {
    if args.len() != 2 { return Err("need two args for mod".to_string()); }
    let num = match args[0] {
        Object::Num(n) => n,
        _ => Err(String::from("error converting quo arguments to numbers"))?
    };
    let div = match args[1] {
        Object::Num(n) => n,
        _ => Err(String::from("error converting quo arguments to numbers"))?
    };

    Ok(num % div)
}