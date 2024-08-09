use punctuated::Punctuated;
use quote::ToTokens;
use syn::*;

pub(crate) trait Rewriter<
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
>
{
    fn rewrite(self, f: F, u: U) -> Self;
}

pub(crate) trait CheckOverWrite<U: Fn(Ident) -> bool + Clone> {
    fn check_overwrite(&self, u: U) -> bool;
}

impl<F, U> Rewriter<F, U> for ItemFn
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.block = Box::new(self.block.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for Local
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.pat = self.pat.rewrite(f.clone(), u.clone());
        self.init = self.init.map(|expr| expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for LocalInit
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for Expr
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        match &mut self {
            Expr::Array(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Assign(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Async(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Await(_) => {
                return self;
            }
            Expr::Binary(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Block(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Break(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Call(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Cast(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Closure(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Const(_) => {
                return self;
            }
            Expr::Continue(_) => {
                return self;
            }
            Expr::Field(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::ForLoop(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Group(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::If(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Index(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Infer(_) => {
                return self;
            }
            Expr::Let(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Lit(_) => {
                return self;
            }
            Expr::Loop(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Macro(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Match(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::MethodCall(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Paren(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Path(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Range(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Reference(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Repeat(_) => {
                return self;
            }
            Expr::Return(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Struct(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Try(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::TryBlock(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Tuple(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Unary(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Unsafe(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Verbatim(_) => {
                return self;
            }
            Expr::While(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Expr::Yield(expr) => {
                *expr = expr.clone().rewrite(f, u);
            }
            _ => {
                unreachable!();
            }
        }

        self
    }
}

impl<F, U> Rewriter<F, U> for ExprArray
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.elems = self
            .elems
            .into_iter()
            .map(|expr| expr.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprAssign
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.right = Box::new(self.right.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprAsync
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.block = self.block.rewrite(f, u);
        self
    }
}

use syn::BinOp::*;

impl<F, U> Rewriter<F, U> for ExprBinary
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        match self.op {
            AddAssign(_) | SubAssign(_) | MulAssign(_) | DivAssign(_) | RemAssign(_)
            | BitXorAssign(_) | BitAndAssign(_) | BitOrAssign(_) | ShlAssign(_) | ShrAssign(_) => {
                self.right = Box::new(self.right.rewrite(f, u));
                return self;
            }
            _ => {
                self.left = Box::new(self.left.rewrite(f.clone(), u.clone()));
                self.right = Box::new(self.right.rewrite(f, u));
                return self;
            }
        }
    }
}

impl<F, U> Rewriter<F, U> for ExprBlock
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.block = self.block.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprBreak
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = self.expr.map(|expr| Box::new(expr.rewrite(f, u)));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprCall
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.func = Box::new(self.func.rewrite(f.clone(), u.clone()));
        self.args = self
            .args
            .into_iter()
            .map(|expr| expr.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprCast
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprClosure
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        if self.check_overwrite(u.clone()) {
            return self;
        }
        self.body = Box::new(self.body.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprField
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.base = Box::new(self.base.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprForLoop
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        if self.check_overwrite(u.clone()) {
            return self;
        }
        self.body = self.body.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprGroup
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprIf
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.cond = Box::new(self.cond.rewrite(f.clone(), u.clone()));
        if self.check_overwrite(u.clone()) {
            return self;
        }
        self.then_branch = self.then_branch.rewrite(f.clone(), u.clone());
        self.else_branch = self
            .else_branch
            .map(|(token, expr)| (token, Box::new(expr.rewrite(f, u))));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprIndex
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f.clone(), u.clone()));
        self.index = Box::new(self.index.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprLet
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f.clone(), u.clone()));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprPath
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.path = self.path.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprLoop
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, _u: U) -> Self {
        self.body = self.body.rewrite(f, _u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprMacro
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.mac = self.mac.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprMatch
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f.clone(), u.clone()));
        self.arms = self
            .arms
            .into_iter()
            .map(|arm| arm.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprMethodCall
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.receiver = Box::new(self.receiver.rewrite(f.clone(), u.clone()));
        self.args = self
            .args
            .into_iter()
            .map(|expr| expr.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprParen
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for Path
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, _u: U) -> Self {
        self.segments = self
            .segments
            .into_iter()
            .enumerate()
            .flat_map(|(i, seg)| {
                if i == 0 {
                    f(seg)
                } else {
                    vec![seg].into_iter().collect()
                }
            })
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprRange
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.start = self
            .start
            .map(|expr| Box::new(expr.rewrite(f.clone(), u.clone())));
        self.end = self.end.map(|expr| Box::new(expr.rewrite(f, u)));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprReference
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprReturn
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = self.expr.map(|expr| Box::new(expr.rewrite(f, u)));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprStruct
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.path = self.path.rewrite(f.clone(), u.clone());
        self.fields = self
            .fields
            .into_iter()
            .map(|field| field.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprTry
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprTryBlock
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.block = self.block.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprTuple
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.elems = self
            .elems
            .into_iter()
            .map(|expr| expr.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprUnary
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = Box::new(self.expr.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprUnsafe
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.block = self.block.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprWhile
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        let overwrite = self.cond.check_overwrite(u.clone());
        self.cond = Box::new(self.cond.rewrite(f.clone(), u.clone()));
        if overwrite {
            return self;
        }
        self.body = self.body.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for ExprYield
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = self.expr.map(|expr| Box::new(expr.rewrite(f, u)));
        self
    }
}

impl<F, U> Rewriter<F, U> for Block
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        for stmt in &mut self.stmts {
            if stmt.check_overwrite(u.clone()) {
                break;
            }
            *stmt = stmt.clone().rewrite(f.clone(), u.clone());
        }
        self
    }
}

impl<F, U> Rewriter<F, U> for Stmt
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        match &mut self {
            Stmt::Local(local) => {
                *local = local.clone().rewrite(f, u);
            }
            Stmt::Item(_) => {
                return self;
            }
            Stmt::Expr(expr, _) => {
                *expr = expr.clone().rewrite(f, u);
            }
            Stmt::Macro(macro_stmt) => {
                *macro_stmt = macro_stmt.clone().rewrite(f, u);
            }
        }
        self
    }
}

impl<F, U> Rewriter<F, U> for StmtMacro
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.mac.tokens = self.mac.tokens.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for Macro
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.tokens = self.tokens.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for FieldValue
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.expr = self.expr.rewrite(f, u);
        self
    }
}

impl<F, U> Rewriter<F, U> for proc_macro2::TokenStream
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(self, f: F, u: U) -> Self {
        self.into_iter()
            .flat_map(|token| match token {
                proc_macro2::TokenTree::Group(group) => {
                    let group = group;
                    let new_group = group.stream().rewrite(f.clone(), u.clone());
                    vec![proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                        group.delimiter(),
                        new_group,
                    ))]
                    .into_iter()
                    .collect::<Vec<_>>()
                }
                proc_macro2::TokenTree::Ident(ident) => {
                    f(ident.into()).to_token_stream().into_iter().collect()
                }
                proc_macro2::TokenTree::Literal(literal) => {
                    vec![proc_macro2::TokenTree::Literal(literal)]
                        .into_iter()
                        .collect()
                }
                proc_macro2::TokenTree::Punct(punct) => vec![proc_macro2::TokenTree::Punct(punct)]
                    .into_iter()
                    .collect(),
            })
            .collect()
    }
}

impl<F, U> Rewriter<F, U> for Arm
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        let overwrite = self.pat.check_overwrite(u.clone());
        self.pat = self.pat.rewrite(f.clone(), u.clone());
        if overwrite {
            return self;
        }
        self.body = Box::new(self.body.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for Pat
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        match &mut self {
            Pat::Const(_) => {
                return self;
            }
            Pat::Ident(_) => {
                return self;
            }
            Pat::Lit(_) => {
                return self;
            }
            Pat::Macro(macros) => {
                *macros = macros.clone().rewrite(f, u);
            }
            Pat::Or(pat_or) => {
                *pat_or = pat_or.clone().rewrite(f, u);
            }
            Pat::Paren(pat_paren) => {
                *pat_paren = pat_paren.clone().rewrite(f, u);
            }
            Pat::Path(pat_path) => {
                *pat_path = pat_path.clone().rewrite(f, u);
            }
            Pat::Range(pat_range) => {
                *pat_range = pat_range.clone().rewrite(f, u);
            }
            Pat::Reference(pat_reference) => {
                *pat_reference = pat_reference.clone().rewrite(f, u);
            }
            Pat::Rest(_) => {
                return self;
            }
            Pat::Slice(pat_slice) => {
                *pat_slice = pat_slice.clone().rewrite(f, u);
            }
            Pat::Struct(pat_struct) => {
                *pat_struct = pat_struct.clone().rewrite(f, u);
            }
            Pat::Tuple(pat_tuple) => {
                *pat_tuple = pat_tuple.clone().rewrite(f, u);
            }
            Pat::TupleStruct(pat_tuple_struct) => {
                *pat_tuple_struct = pat_tuple_struct.clone().rewrite(f, u);
            }
            Pat::Type(pat_type) => {
                *pat_type = pat_type.clone().rewrite(f, u);
            }
            Pat::Verbatim(_) => {
                return self;
            }
            Pat::Wild(_) => {
                return self;
            }
            &mut _ => {
                unreachable!();
            }
        }
        self
    }
}

impl<F, U> Rewriter<F, U> for PatOr
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.cases = self
            .cases
            .into_iter()
            .map(|pat| pat.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for PatParen
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.pat = Box::new(self.pat.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for FieldPat
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.pat = Box::new(self.pat.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for PatReference
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.pat = Box::new(self.pat.rewrite(f, u));
        self
    }
}

impl<F, U> Rewriter<F, U> for PatSlice
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.elems = self
            .elems
            .into_iter()
            .map(|pat| pat.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for PatStruct
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.fields = self
            .fields
            .into_iter()
            .map(|field| field.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for PatTupleStruct
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.elems = self
            .elems
            .into_iter()
            .map(|pat| pat.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for PatTuple
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.elems = self
            .elems
            .into_iter()
            .map(|pat| pat.rewrite(f.clone(), u.clone()))
            .collect();
        self
    }
}

impl<F, U> Rewriter<F, U> for PatType
where
    F: Fn(PathSegment) -> Punctuated<PathSegment, Token![::]> + Clone,
    U: Fn(Ident) -> bool + Clone,
{
    fn rewrite(mut self, f: F, u: U) -> Self {
        self.pat = Box::new(self.pat.rewrite(f, u));
        self
    }
}

impl<U> CheckOverWrite<U> for Expr
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        match &self {
            Expr::Let(expr_let) => expr_let.check_overwrite(u),
            _ => false,
        }
    }
}

impl<U> CheckOverWrite<U> for Stmt
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        match &self {
            Stmt::Local(local) => local.check_overwrite(u),
            Stmt::Expr(expr, _) => expr.check_overwrite(u),
            _ => false,
        }
    }
}

impl<U> CheckOverWrite<U> for Pat
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        match self {
            Pat::Ident(pat_ident) => u(pat_ident.ident.clone()),
            Pat::Reference(pat_reference) => pat_reference.pat.check_overwrite(u),
            Pat::Slice(pat_slice) => pat_slice
                .elems
                .iter()
                .any(|pat| pat.check_overwrite(u.clone())),
            Pat::Struct(pat_struct) => pat_struct
                .fields
                .iter()
                .any(|field| field.pat.check_overwrite(u.clone())),
            Pat::Tuple(pat_tuple) => pat_tuple
                .elems
                .iter()
                .any(|pat| pat.check_overwrite(u.clone())),
            Pat::TupleStruct(pat_tuple_struct) => pat_tuple_struct
                .elems
                .iter()
                .any(|pat| pat.check_overwrite(u.clone())),
            Pat::Type(pat_type) => pat_type.pat.check_overwrite(u),
            _ => false,
        }
    }
}

impl<U> CheckOverWrite<U> for Local
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        self.pat.check_overwrite(u)
    }
}

impl<U> CheckOverWrite<U> for ExprClosure
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        self.inputs.iter().any(|pat| pat.check_overwrite(u.clone()))
    }
}

impl<U> CheckOverWrite<U> for ExprForLoop
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        self.pat.check_overwrite(u)
    }
}

impl<U> CheckOverWrite<U> for ExprIf
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        match self.cond.as_ref() {
            Expr::Let(expr_let) => expr_let.check_overwrite(u),
            _ => false,
        }
    }
}

impl<U> CheckOverWrite<U> for ExprLet
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        self.pat.check_overwrite(u)
    }
}

impl<U> CheckOverWrite<U> for ExprWhile
where
    U: Fn(Ident) -> bool + Clone,
{
    fn check_overwrite(&self, u: U) -> bool {
        self.cond.check_overwrite(u)
    }
}
