mod object;
mod default_env;
mod eval;
mod lex;
mod parse;
mod repl;
mod token;
mod vm;

use default_env::default_env;

use crate::{
    repl::repl,
};

fn main() {
    repl(default_env());
}

// fn main() {
//     let input = "(+ 3 4)";

//     let tokens = TokenStream::new(
//         lex::lex(&input)
//             .into_iter()
//             .filter(|t| t.kind != TokenKind::Comment)
//             .collect(),
//     );

//     let ast = parse::parse(&mut tokens.clone().peekable());

//     for t in tokens {
//         println!("{:?} {}", t, t.lit)
//     }
//     println!("{}", ast);
// }
