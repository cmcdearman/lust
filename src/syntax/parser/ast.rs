use crate::util::{intern::InternedString, meta::Meta};
use num_rational::Rational64;

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub items: Vec<Item>,
    pub meta: Meta,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Def(Def),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: InternedString,
    pub value: Expr,
    pub meta: Meta,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Symbol {
        name: InternedString,
        meta: Meta,
    },
    Lit {
        value: Lit,
        meta: Meta,
    },
    List {
        values: Vec<Self>,
        meta: Meta,
    },
    Lambda {
        params: Vec<Self>,
        body: Box<Self>,
        meta: Meta,
    },
    Apply {
        fun: Box<Self>,
        args: Vec<Self>,
        meta: Meta,
    },
    Let {
        name: InternedString,
        value: Box<Self>,
        body: Box<Self>,
        meta: Meta,
    },
    Quote {
        value: Box<Self>,
        meta: Meta,
    },
    Unquote {
        value: Box<Self>,
        meta: Meta,
    },
    UnquoteSplice {
        value: Box<Self>,
        meta: Meta,
    },
    If {
        cond: Box<Self>,
        then: Box<Self>,
        else_: Box<Self>,
        meta: Meta,
    },
    Nil(Meta),
}

impl Expr {
    pub fn meta(&self) -> &Meta {
        match self {
            Expr::Symbol { meta, .. } => meta,
            Expr::Lit { meta, .. } => meta,
            Expr::List { meta, .. } => meta,
            Expr::Lambda { meta, .. } => meta,
            Expr::Apply { meta, .. } => meta,
            Expr::Let { meta, .. } => meta,
            Expr::Quote { meta, .. } => meta,
            Expr::Unquote { meta, .. } => meta,
            Expr::UnquoteSplice { meta, .. } => meta,
            Expr::If { meta, .. } => meta,
            Expr::Nil(m) => m,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(pub InternedString);

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    // Int(i64),
    Number(Rational64),
    // Real(f64),
    // Char(char),
    String(InternedString),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

// use crate::util::{spaces, Format, InternedString, Span};
// use std::fmt::Debug;

// #[derive(Clone, PartialEq)]
// pub struct Root {
//     pub items: Vec<Item>,
//     pub span: Span,
// }

// impl Debug for Root {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         write!(f, "Root @ {}\n", self.span)?;
//         for item in self.items.clone() {
//             write!(f, "{:?}", Format::new(2, item))?;
//         }
//         Ok(())
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Item {
//     Expr(Expr),
//     Def {
//         name: InternedString,
//         value: Expr,
//         span: Span,
//     },
// }

// impl Debug for Format<Item> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         match &self.value {
//             Item::Expr(expr) => write!(f, "{:?}", Format::new(self.indent, expr.clone())),
//             Item::Def { name, value, span } => write!(
//                 f,
//                 "{}Def({}) @ {}\n{:?}",
//                 spaces(self.indent),
//                 name,
//                 span,
//                 Format::new(self.indent + 2, value.clone())
//             ),
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Expr {
//     Symbol {
//         name: InternedString,
//         span: Span,
//     },
//     Number {
//         value: i64,
//         span: Span,
//     },
//     Bool {
//         value: bool,
//         span: Span,
//     },
//     String {
//         value: InternedString,
//         span: Span,
//     },
//     List {
//         exprs: Vec<Self>,
//         span: Span,
//     },
//     Lambda {
//         params: Vec<InternedString>,
//         body: Box<Self>,
//         span: Span,
//     },
//     Apply {
//         fun: Box<Self>,
//         args: Vec<Self>,
//         span: Span,
//     },
//     If {
//         cond: Box<Self>,
//         then: Box<Self>,
//         else_: Box<Self>,
//         span: Span,
//     },
// }

// impl Debug for Format<Expr> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         match &self.value {
//             Expr::Symbol { name, span } => {
//                 write!(f, "{}Sym({}) @ {}", spaces(self.indent), name, span)
//             }
//             Expr::Number { value, span } => {
//                 write!(f, "{}Num({}) @ {}", spaces(self.indent), value, span)
//             }
//             Expr::Bool { value, span } => {
//                 write!(f, "{}Bool({}) @ {}", spaces(self.indent), value, span)
//             }
//             Expr::String { value, span } => {
//                 write!(f, "{}Str({}) @ {}", spaces(self.indent), value, span)
//             }
//             Expr::List { exprs, span } => {
//                 write!(f, "{}List @ {}\n", spaces(self.indent), span)?;
//                 for (i, expr) in exprs.clone().iter().enumerate() {
//                     write!(f, "{:?}", Format::new(self.indent + 2, expr.clone()))?;
//                     if i != exprs.len() - 1 {
//                         write!(f, ",\n")?;
//                     }
//                 }
//                 Ok(())
//             }
//             Expr::Lambda { params, body, span } => {
//                 write!(f, "{}Lambda @ {}\n", spaces(self.indent), span)?;
//                 for param in params.clone() {
//                     write!(f, "{}Param({})\n", spaces(self.indent + 2), param)?;
//                 }
//                 write!(
//                     f,
//                     "{:?}",
//                     Format::new(self.indent + 2, body.clone().as_ref().clone())
//                 )
//             }
//             Expr::Apply { fun, args, span } => {
//                 write!(f, "{}Apply @ {}\n", spaces(self.indent), span)?;
//                 write!(
//                     f,
//                     "{:?}",
//                     Format::new(self.indent + 2, fun.clone().as_ref().clone())
//                 )?;
//                 for arg in args.clone() {
//                     write!(f, ",\n")?;
//                     write!(f, "{:?}", Format::new(self.indent + 2, arg.clone()))?;
//                 }
//                 Ok(())
//             }
//             Expr::If {
//                 cond,
//                 then,
//                 else_,
//                 span,
//             } => {
//                 write!(f, "{}If @ {}\n", spaces(self.indent), span)?;
//                 write!(
//                     f,
//                     "{:?}",
//                     Format::new(self.indent + 2, cond.clone().as_ref().clone())
//                 )?;
//                 write!(f, ",\n")?;
//                 write!(
//                     f,
//                     "{:?}",
//                     Format::new(self.indent + 2, then.clone().as_ref().clone())
//                 )?;
//                 write!(f, ",\n")?;
//                 write!(
//                     f,
//                     "{:?}",
//                     Format::new(self.indent + 2, else_.clone().as_ref().clone())
//                 )?;
//                 Ok(())
//             }
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct CondPair {
//     pub cond: Expr,
//     pub expr: Expr,
//     pub span: Span,
// }

// impl Debug for Format<CondPair> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Pretty print with indents and spans
//         write!(f, "{}CondPair @ {}\n", spaces(self.indent), self.value.span)?;
//         write!(
//             f,
//             "{:?}",
//             Format::new(self.indent + 2, self.value.cond.clone())
//         )?;
//         write!(f, ",\n")?;
//         write!(
//             f,
//             "{:?}",
//             Format::new(self.indent + 2, self.value.expr.clone())
//         )?;
//         Ok(())
//     }
// }
