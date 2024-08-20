use proc_macro2::TokenStream;
use punctuated::Punctuated;
use syn::*;

/// とある特定のマクロを置換するトレイト
pub trait Switcher<U: Fn(Macro) -> TokenStream> {
    fn switcher(self, u: &U) -> Self;
}

/// Punctuated
impl<U: Fn(Macro) -> TokenStream, T: Switcher<U>, P: Default> Switcher<U> for Punctuated<T, P> {
    fn switcher(self, u: &U) -> Self {
        self.into_iter()
            .map(|item| item.switcher(u))
            .collect::<Punctuated<T, P>>()
    }
}

/// Vec
impl<U: Fn(Macro) -> TokenStream, T: Switcher<U>> Switcher<U> for Vec<T> {
    fn switcher(self, u: &U) -> Self {
        self.into_iter()
            .map(|item| item.switcher(u))
            .collect::<Vec<T>>()
    }
}

/// Option
impl<U: Fn(Macro) -> TokenStream, T: Switcher<U>> Switcher<U> for Option<T> {
    fn switcher(self, u: &U) -> Self {
        match self {
            Some(item) => Some(item.switcher(u)),
            None => None,
        }
    }
}

/// Box
impl<U: Fn(Macro) -> TokenStream, T: Switcher<U>> Switcher<U> for Box<T> {
    fn switcher(self, u: &U) -> Self {
        Box::new((*self).switcher(u))
    }
}

/// Tuple
impl<U: Fn(Macro) -> TokenStream, A, B: Switcher<U>> Switcher<U> for (A, B) {
    fn switcher(self, u: &U) -> Self {
        (self.0, self.1.switcher(u))
    }
}

/// Type
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Type {
    fn switcher(self, u: &U) -> Self {
        match self {
            Type::Array(TypeArray {
                elem,
                len,
                bracket_token,
                semi_token,
            }) => {
                let elem = elem.switcher(u);
                let len = len.switcher(u);
                return Type::Array(TypeArray {
                    elem,
                    len,
                    bracket_token,
                    semi_token,
                });
            }
            Type::BareFn(TypeBareFn {
                lifetimes,
                inputs,
                output,
                fn_token,
                unsafety,
                abi,
                paren_token,
                variadic,
            }) => {
                let inputs = inputs.switcher(u);
                let output = output.switcher(u);
                return Type::BareFn(TypeBareFn {
                    lifetimes,
                    inputs,
                    output,
                    fn_token,
                    unsafety,
                    abi,
                    paren_token,
                    variadic,
                });
            }
            Type::Group(TypeGroup { elem, group_token }) => {
                let elem = elem.switcher(u);
                return Type::Group(TypeGroup { elem, group_token });
            }
            Type::ImplTrait(TypeImplTrait { bounds, impl_token }) => {
                let bounds = bounds.switcher(u);
                return Type::ImplTrait(TypeImplTrait { bounds, impl_token });
            }
            Type::Macro(TypeMacro { mac, .. }) => {
                let mac = u(mac);
                return parse_quote! { #mac };
            }
            Type::Paren(TypeParen { elem, paren_token }) => {
                let elem = elem.switcher(u);
                return Type::Paren(TypeParen { elem, paren_token });
            }
            Type::Path(TypePath { qself, path }) => {
                let path = path.switcher(u);
                return Type::Path(TypePath { qself, path });
            }
            Type::Ptr(TypePtr {
                elem,
                mutability,
                star_token,
                const_token,
            }) => {
                let elem = elem.switcher(u);
                return Type::Ptr(TypePtr {
                    elem,
                    mutability,
                    star_token,
                    const_token,
                });
            }
            Type::Reference(TypeReference {
                elem,
                lifetime,
                mutability,
                and_token,
            }) => {
                let elem = elem.switcher(u);
                return Type::Reference(TypeReference {
                    elem,
                    lifetime,
                    mutability,
                    and_token,
                });
            }
            Type::Slice(TypeSlice {
                elem,
                bracket_token,
            }) => {
                let elem = elem.switcher(u);
                return Type::Slice(TypeSlice {
                    elem,
                    bracket_token,
                });
            }
            Type::TraitObject(TypeTraitObject { bounds, dyn_token }) => {
                let bounds = bounds.switcher(u);
                return Type::TraitObject(TypeTraitObject { bounds, dyn_token });
            }
            Type::Tuple(TypeTuple { elems, paren_token }) => {
                let elems = elems.switcher(u);
                return Type::Tuple(TypeTuple { elems, paren_token });
            }
            Type::Infer(_) => return self,
            Type::Never(_) => return self,
            Type::Verbatim(_) => return self,
            _ => unreachable!(),
        };
    }
}

/// Expr
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Expr {
    fn switcher(self, u: &U) -> Self {
        match self {
            Expr::Array(ExprArray {
                attrs,
                bracket_token,
                elems,
            }) => {
                let attrs = attrs.switcher(u);
                let elems = elems.switcher(u);
                return Expr::Array(ExprArray {
                    attrs,
                    bracket_token,
                    elems,
                });
            }
            Expr::Assign(ExprAssign {
                attrs,
                left,
                eq_token,
                right,
            }) => {
                let attrs = attrs.switcher(u);
                let left = left.switcher(u);
                let right = right.switcher(u);
                return Expr::Assign(ExprAssign {
                    attrs,
                    left,
                    eq_token,
                    right,
                });
            }
            Expr::Async(ExprAsync {
                attrs,
                async_token,
                block,
                capture,
            }) => {
                let attrs = attrs.switcher(u);
                let block = block.switcher(u);
                return Expr::Async(ExprAsync {
                    attrs,
                    async_token,
                    block,
                    capture,
                });
            }
            Expr::Await(ExprAwait {
                attrs,
                base,
                dot_token,
                await_token,
            }) => {
                let attrs = attrs.switcher(u);
                let base = base.switcher(u);
                return Expr::Await(ExprAwait {
                    attrs,
                    base,
                    dot_token,
                    await_token,
                });
            }
            Expr::Binary(ExprBinary {
                attrs,
                left,
                op,
                right,
            }) => {
                let attrs = attrs.switcher(u);
                let left = left.switcher(u);
                let right = right.switcher(u);
                return Expr::Binary(ExprBinary {
                    attrs,
                    left,
                    op,
                    right,
                });
            }
            Expr::Block(ExprBlock {
                attrs,
                block,
                label,
            }) => {
                let attrs = attrs.switcher(u);
                let block = block.switcher(u);
                return Expr::Block(ExprBlock {
                    attrs,
                    block,
                    label,
                });
            }
            Expr::Break(ExprBreak {
                attrs,
                break_token,
                label,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Break(ExprBreak {
                    attrs,
                    break_token,
                    label,
                    expr,
                });
            }
            Expr::Call(ExprCall {
                attrs,
                func,
                args,
                paren_token,
            }) => {
                let attrs = attrs.switcher(u);
                let func = func.switcher(u);
                let args = args.switcher(u);
                return Expr::Call(ExprCall {
                    attrs,
                    func,
                    args,
                    paren_token,
                });
            }
            Expr::Cast(ExprCast {
                attrs,
                expr,
                as_token,
                ty,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                let ty = ty.switcher(u);
                return Expr::Cast(ExprCast {
                    attrs,
                    expr,
                    as_token,
                    ty,
                });
            }
            Expr::Closure(ExprClosure {
                attrs,
                asyncness,
                movability,
                capture,
                or1_token,
                inputs,
                output,
                or2_token,
                body,
                lifetimes,
                constness,
            }) => {
                let attrs = attrs.switcher(u);
                let inputs = inputs.switcher(u);
                let output = output.switcher(u);
                let body = body.switcher(u);
                return Expr::Closure(ExprClosure {
                    attrs,
                    asyncness,
                    movability,
                    capture,
                    or1_token,
                    inputs,
                    output,
                    or2_token,
                    body,
                    lifetimes,
                    constness,
                });
            }
            Expr::Continue(ExprContinue {
                attrs,
                continue_token,
                label,
            }) => {
                let attrs = attrs.switcher(u);
                return Expr::Continue(ExprContinue {
                    attrs,
                    continue_token,
                    label,
                });
            }
            Expr::Field(ExprField {
                attrs,
                base,
                dot_token,
                member,
            }) => {
                let attrs = attrs.switcher(u);
                let base = base.switcher(u);
                return Expr::Field(ExprField {
                    attrs,
                    base,
                    dot_token,
                    member,
                });
            }
            Expr::ForLoop(ExprForLoop {
                attrs,
                label,
                for_token,
                pat,
                in_token,
                expr,
                body,
            }) => {
                let attrs = attrs.switcher(u);
                let pat = pat.switcher(u);
                let expr = expr.switcher(u);
                let body = body.switcher(u);
                return Expr::ForLoop(ExprForLoop {
                    attrs,
                    label,
                    for_token,
                    pat,
                    in_token,
                    expr,
                    body,
                });
            }
            Expr::Group(ExprGroup {
                attrs,
                group_token,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Group(ExprGroup {
                    attrs,
                    group_token,
                    expr,
                });
            }
            Expr::If(ExprIf {
                attrs,
                if_token,
                cond,
                then_branch,
                else_branch,
            }) => {
                let attrs = attrs.switcher(u);
                let cond = cond.switcher(u);
                let then_branch = then_branch.switcher(u);
                let else_branch = else_branch.switcher(u);
                return Expr::If(ExprIf {
                    attrs,
                    if_token,
                    cond: cond,
                    then_branch,
                    else_branch,
                });
            }
            Expr::Index(ExprIndex {
                attrs,
                expr,
                bracket_token,
                index,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                let index = index.switcher(u);
                return Expr::Index(ExprIndex {
                    attrs,
                    expr,
                    bracket_token,
                    index,
                });
            }
            Expr::Let(ExprLet {
                attrs,
                let_token,
                pat,
                eq_token,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let pat = pat.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Let(ExprLet {
                    attrs,
                    let_token,
                    pat,
                    eq_token,
                    expr,
                });
            }
            Expr::Lit(ExprLit { attrs, lit }) => {
                let attrs = attrs.switcher(u);
                return Expr::Lit(ExprLit { attrs, lit });
            }
            Expr::Loop(ExprLoop {
                attrs,
                label,
                loop_token,
                body,
            }) => {
                let attrs = attrs.switcher(u);
                let body = body.switcher(u);
                return Expr::Loop(ExprLoop {
                    attrs,
                    label,
                    loop_token,
                    body,
                });
            }
            Expr::Macro(ExprMacro { mac, .. }) => {
                let mac = u(mac);
                return parse_quote! { #mac };
            }
            Expr::Match(ExprMatch {
                attrs,
                match_token,
                expr,
                brace_token,
                arms,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                let arms = arms.switcher(u);
                return Expr::Match(ExprMatch {
                    attrs,
                    match_token,
                    expr,
                    brace_token,
                    arms,
                });
            }
            Expr::MethodCall(ExprMethodCall {
                attrs,
                receiver,
                dot_token,
                method,
                turbofish,
                paren_token,
                args,
            }) => {
                let attrs = attrs.switcher(u);
                let receiver = receiver.switcher(u);
                let args = args.switcher(u);
                return Expr::MethodCall(ExprMethodCall {
                    attrs,
                    receiver,
                    dot_token,
                    method,
                    turbofish,
                    paren_token,
                    args,
                });
            }
            Expr::Paren(ExprParen {
                attrs,
                paren_token,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Paren(ExprParen {
                    attrs,
                    paren_token,
                    expr,
                });
            }
            Expr::Path(ExprPath { attrs, path, qself }) => {
                let attrs = attrs.switcher(u);
                let path = path.switcher(u);
                return Expr::Path(ExprPath { attrs, path, qself });
            }
            Expr::Range(ExprRange {
                attrs,
                limits,
                start,
                end,
            }) => {
                let attrs = attrs.switcher(u);
                let start = start.switcher(u);
                let end = end.switcher(u);
                return Expr::Range(ExprRange {
                    attrs,
                    limits,
                    start,
                    end,
                });
            }
            Expr::Reference(ExprReference {
                attrs,
                and_token,
                mutability,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Reference(ExprReference {
                    attrs,
                    and_token,
                    mutability,
                    expr,
                });
            }
            Expr::Repeat(ExprRepeat {
                attrs,
                bracket_token,
                expr,
                semi_token,
                len,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                let len = len.switcher(u);
                return Expr::Repeat(ExprRepeat {
                    attrs,
                    bracket_token,
                    expr,
                    semi_token,
                    len,
                });
            }
            Expr::Return(ExprReturn {
                attrs,
                return_token,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Return(ExprReturn {
                    attrs,
                    return_token,
                    expr,
                });
            }
            Expr::Struct(ExprStruct {
                attrs,
                path,
                brace_token,
                fields,
                dot2_token,
                rest,
                qself,
            }) => {
                let attrs = attrs.switcher(u);
                let path = path.switcher(u);
                let fields = fields.switcher(u);
                let rest = rest.switcher(u);
                return Expr::Struct(ExprStruct {
                    attrs,
                    path,
                    brace_token,
                    fields,
                    dot2_token,
                    rest,
                    qself,
                });
            }
            Expr::Try(ExprTry {
                attrs,
                expr,
                question_token,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Try(ExprTry {
                    attrs,
                    expr,
                    question_token,
                });
            }
            Expr::TryBlock(ExprTryBlock {
                attrs,
                try_token,
                block,
            }) => {
                let attrs = attrs.switcher(u);
                let block = block.switcher(u);
                return Expr::TryBlock(ExprTryBlock {
                    attrs,
                    try_token,
                    block,
                });
            }
            Expr::Tuple(ExprTuple {
                attrs,
                paren_token,
                elems,
            }) => {
                let attrs = attrs.switcher(u);
                let elems = elems.switcher(u);
                return Expr::Tuple(ExprTuple {
                    attrs,
                    paren_token,
                    elems,
                });
            }
            Expr::Unary(ExprUnary { attrs, op, expr }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Unary(ExprUnary { attrs, op, expr });
            }
            Expr::Unsafe(ExprUnsafe {
                attrs,
                unsafe_token,
                block,
            }) => {
                let attrs = attrs.switcher(u);
                let block = block.switcher(u);
                return Expr::Unsafe(ExprUnsafe {
                    attrs,
                    unsafe_token,
                    block,
                });
            }
            Expr::Verbatim(tts) => {
                return Expr::Verbatim(tts);
            }
            Expr::While(ExprWhile {
                attrs,
                label,
                while_token,
                cond,
                body,
            }) => {
                let attrs = attrs.switcher(u);
                let cond = cond.switcher(u);
                let body = body.switcher(u);
                return Expr::While(ExprWhile {
                    attrs,
                    label,
                    while_token,
                    cond,
                    body,
                });
            }
            Expr::Yield(ExprYield {
                attrs,
                yield_token,
                expr,
            }) => {
                let attrs = attrs.switcher(u);
                let expr = expr.switcher(u);
                return Expr::Yield(ExprYield {
                    attrs,
                    yield_token,
                    expr,
                });
            }
            Expr::Const(_) => return self,
            Expr::Infer(_) => return self,
            _ => unreachable!(),
        };
    }
}

/// Pat
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Pat {
    fn switcher(self, u: &U) -> Self {
        match self {
            Pat::Macro(PatMacro { mac, .. }) => {
                let mac = u(mac);
                return parse_quote! { #mac };
            }
            Pat::Const(PatConst {
                attrs,
                const_token,
                block,
            }) => {
                let attrs = attrs.switcher(u);
                let block = block.switcher(u);
                return Pat::Const(PatConst {
                    attrs,
                    const_token,
                    block,
                });
            }
            Pat::Ident(PatIdent {
                attrs,
                by_ref,
                mutability,
                ident,
                subpat,
            }) => {
                let attrs = attrs.switcher(u);
                let subpat = subpat.switcher(u);
                return Pat::Ident(PatIdent {
                    attrs,
                    by_ref,
                    mutability,
                    ident,
                    subpat,
                });
            }
            Pat::Path(PatPath { attrs, qself, path }) => {
                let attrs = attrs.switcher(u);
                let path = path.switcher(u);
                return Pat::Path(PatPath { attrs, qself, path });
            }
            Pat::Struct(PatStruct {
                attrs,
                path,
                brace_token,
                fields,
                rest,
                qself,
            }) => {
                let attrs = attrs.switcher(u);
                let path = path.switcher(u);
                let fields = fields.switcher(u);
                let rest = rest.switcher(u);
                return Pat::Struct(PatStruct {
                    attrs,
                    path,
                    brace_token,
                    fields,
                    rest,
                    qself,
                });
            }
            Pat::TupleStruct(PatTupleStruct {
                attrs,
                path,
                paren_token,
                qself,
                elems,
            }) => {
                let attrs = attrs.switcher(u);
                let path = path.switcher(u);
                let elems = elems.switcher(u);
                return Pat::TupleStruct(PatTupleStruct {
                    attrs,
                    path,
                    paren_token,
                    qself,
                    elems,
                });
            }
            Pat::Tuple(PatTuple {
                attrs,
                elems,
                paren_token,
            }) => {
                let attrs = attrs.switcher(u);
                let elems = elems.switcher(u);
                return Pat::Tuple(PatTuple {
                    attrs,
                    elems,
                    paren_token,
                });
            }
            Pat::Type(PatType {
                attrs,
                pat,
                colon_token,
                ty,
            }) => {
                let attrs = attrs.switcher(u);
                let pat = pat.switcher(u);
                let ty = ty.switcher(u);
                return Pat::Type(PatType {
                    attrs,
                    pat,
                    colon_token,
                    ty,
                });
            }
            Pat::Wild(PatWild {
                attrs,
                underscore_token,
            }) => {
                let attrs = attrs.switcher(u);
                return Pat::Wild(PatWild {
                    attrs,
                    underscore_token,
                });
            }
            Pat::Lit(PatLit { attrs, lit }) => {
                let attrs = attrs.switcher(u);
                return Pat::Lit(PatLit { attrs, lit });
            }
            Pat::Range(PatRange {
                attrs,
                limits,
                start,
                end,
            }) => {
                let attrs = attrs.switcher(u);
                let start = start.switcher(u);
                let end = end.switcher(u);
                return Pat::Range(PatRange {
                    attrs,
                    limits,
                    start,
                    end,
                });
            }
            Pat::Slice(PatSlice {
                attrs,
                bracket_token,
                elems,
            }) => {
                let attrs = attrs.switcher(u);
                let elems = elems.switcher(u);
                return Pat::Slice(PatSlice {
                    attrs,
                    bracket_token,
                    elems,
                });
            }
            Pat::Or(PatOr {
                attrs,
                cases,
                leading_vert,
            }) => {
                let attrs = attrs.switcher(u);
                let cases = cases.switcher(u);
                return Pat::Or(PatOr {
                    attrs,
                    cases,
                    leading_vert,
                });
            }
            Pat::Reference(PatReference {
                attrs,
                and_token,
                mutability,
                pat,
            }) => {
                let attrs = attrs.switcher(u);
                let pat = pat.switcher(u);
                return Pat::Reference(PatReference {
                    attrs,
                    and_token,
                    mutability,
                    pat,
                });
            }
            Pat::Paren(PatParen {
                attrs,
                paren_token,
                pat,
            }) => {
                let attrs = attrs.switcher(u);
                let pat = pat.switcher(u);
                return Pat::Paren(PatParen {
                    attrs,
                    paren_token,
                    pat,
                });
            }
            Pat::Rest(PatRest { attrs, dot2_token }) => {
                let attrs = attrs.switcher(u);
                return Pat::Rest(PatRest { attrs, dot2_token });
            }
            Pat::Verbatim(tts) => {
                return Pat::Verbatim(tts);
            }
            _ => unreachable!(),
        }
    }
}

/// Item
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Item {
    fn switcher(self, u: &U) -> Self {
        match self {
            Item::Const(ItemConst {
                attrs,
                vis,
                const_token,
                ident,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
                generics,
            }) => {
                let attrs = attrs.switcher(u);
                let ty = ty.switcher(u);
                let expr = expr.switcher(u);
                let generics = generics.switcher(u);
                return Item::Const(ItemConst {
                    attrs,
                    vis,
                    const_token,
                    ident,
                    colon_token,
                    ty,
                    eq_token,
                    expr,
                    semi_token,
                    generics,
                });
            }
            Item::Enum(ItemEnum {
                attrs,
                vis,
                enum_token,
                ident,
                generics,
                brace_token,
                variants,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let variants = variants.switcher(u);
                return Item::Enum(ItemEnum {
                    attrs,
                    vis,
                    enum_token,
                    ident,
                    generics,
                    brace_token,
                    variants,
                });
            }
            Item::ExternCrate(ItemExternCrate {
                attrs,
                vis,
                extern_token,
                crate_token,
                ident,
                rename,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                return Item::ExternCrate(ItemExternCrate {
                    attrs,
                    vis,
                    extern_token,
                    crate_token,
                    ident,
                    rename,
                    semi_token,
                });
            }
            Item::Fn(ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => {
                let attrs = attrs.switcher(u);
                let sig = sig.switcher(u);
                let block = block.switcher(u);
                return Item::Fn(ItemFn {
                    attrs,
                    vis,
                    sig,
                    block,
                });
            }
            Item::Impl(ItemImpl {
                attrs,
                defaultness,
                unsafety,
                impl_token,
                generics,
                trait_,
                self_ty,
                brace_token,
                items,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let self_ty = self_ty.switcher(u);
                let items = items.switcher(u);
                return Item::Impl(ItemImpl {
                    attrs,
                    defaultness,
                    unsafety,
                    impl_token,
                    generics,
                    trait_,
                    self_ty,
                    brace_token,
                    items,
                });
            }
            Item::Macro(ItemMacro {
                attrs,
                ident,
                mac,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let mac = u(mac);
                return parse_quote! {
                    #( #attrs )*
                    #ident
                    #mac
                    #semi_token
                };
            }
            Item::Mod(ItemMod {
                attrs,
                vis,
                mod_token,
                ident,
                content,
                semi,
                unsafety,
            }) => {
                let attrs = attrs.switcher(u);
                let content = content.switcher(u);
                return Item::Mod(ItemMod {
                    attrs,
                    vis,
                    mod_token,
                    ident,
                    content,
                    semi,
                    unsafety,
                });
            }
            Item::Static(ItemStatic {
                attrs,
                vis,
                static_token,
                mutability,
                ident,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let ty = ty.switcher(u);
                let expr = expr.switcher(u);
                return Item::Static(ItemStatic {
                    attrs,
                    vis,
                    static_token,
                    mutability,
                    ident,
                    colon_token,
                    ty,
                    eq_token,
                    expr,
                    semi_token,
                });
            }
            Item::Struct(ItemStruct {
                attrs,
                vis,
                struct_token,
                ident,
                generics,
                fields,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let fields = fields.switcher(u);
                return Item::Struct(ItemStruct {
                    attrs,
                    vis,
                    struct_token,
                    ident,
                    generics,
                    fields,
                    semi_token,
                });
            }
            Item::Trait(ItemTrait {
                attrs,
                vis,
                unsafety,
                auto_token,
                trait_token,
                ident,
                generics,
                colon_token,
                supertraits,
                brace_token,
                items,
                restriction,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let supertraits = supertraits.switcher(u);
                let items = items.switcher(u);
                return Item::Trait(ItemTrait {
                    attrs,
                    vis,
                    unsafety,
                    auto_token,
                    trait_token,
                    ident,
                    generics,
                    colon_token,
                    supertraits,
                    brace_token,
                    items,
                    restriction,
                });
            }
            Item::TraitAlias(ItemTraitAlias {
                attrs,
                vis,
                trait_token,
                ident,
                generics,
                eq_token,
                bounds,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let bounds = bounds.switcher(u);
                return Item::TraitAlias(ItemTraitAlias {
                    attrs,
                    vis,
                    trait_token,
                    ident,
                    generics,
                    eq_token,
                    bounds,
                    semi_token,
                });
            }
            Item::Type(ItemType {
                attrs,
                vis,
                type_token,
                ident,
                generics,
                eq_token,
                ty,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let ty = ty.switcher(u);
                return Item::Type(ItemType {
                    attrs,
                    vis,
                    type_token,
                    ident,
                    generics,
                    eq_token,
                    ty,
                    semi_token,
                });
            }
            Item::Union(ItemUnion {
                attrs,
                vis,
                union_token,
                ident,
                generics,
                fields,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let fields = fields.switcher(u);
                return Item::Union(ItemUnion {
                    attrs,
                    vis,
                    union_token,
                    ident,
                    generics,
                    fields,
                });
            }
            Item::Use(ItemUse {
                attrs,
                vis,
                use_token,
                leading_colon,
                tree,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let tree = tree.switcher(u);
                return Item::Use(ItemUse {
                    attrs,
                    vis,
                    use_token,
                    leading_colon,
                    tree,
                    semi_token,
                });
            }
            Item::ForeignMod(ItemForeignMod {
                attrs,
                abi,
                brace_token,
                items,
                unsafety,
            }) => {
                let attrs = attrs.switcher(u);
                let items = items.switcher(u);
                return Item::ForeignMod(ItemForeignMod {
                    attrs,
                    abi,
                    brace_token,
                    items,
                    unsafety,
                });
            }
            Item::Verbatim(_) => return self,
            _ => unreachable!(),
        }
    }
}

/// Variant
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Variant {
    fn switcher(self, u: &U) -> Self {
        let Variant {
            attrs,
            ident,
            fields,
            discriminant,
        } = self;
        let attrs = attrs.switcher(u);
        let fields = fields.switcher(u);
        let discriminant = discriminant.switcher(u);
        return Variant {
            attrs,
            ident,
            fields,
            discriminant,
        };
    }
}

/// ForeignItem
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for ForeignItem {
    fn switcher(self, u: &U) -> Self {
        match self {
            ForeignItem::Fn(ForeignItemFn {
                attrs,
                vis,
                sig,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let sig = sig.switcher(u);
                return ForeignItem::Fn(ForeignItemFn {
                    attrs,
                    vis,
                    sig,
                    semi_token,
                });
            }
            ForeignItem::Static(ForeignItemStatic {
                attrs,
                vis,
                static_token,
                mutability,
                ident,
                colon_token,
                ty,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let ty = ty.switcher(u);
                return ForeignItem::Static(ForeignItemStatic {
                    attrs,
                    vis,
                    static_token,
                    mutability,
                    ident,
                    colon_token,
                    ty,
                    semi_token,
                });
            }
            ForeignItem::Type(ForeignItemType {
                attrs,
                vis,
                type_token,
                ident,
                generics,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                return ForeignItem::Type(ForeignItemType {
                    attrs,
                    vis,
                    type_token,
                    ident,
                    generics,
                    semi_token,
                });
            }
            ForeignItem::Macro(ForeignItemMacro {
                attrs,
                mac,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let mac = u(mac);
                return parse_quote! {
                    #( #attrs )*
                    #mac
                    #semi_token
                };
            }
            ForeignItem::Verbatim(_) => return self,
            _ => unreachable!(),
        }
    }
}

/// UseTree
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for UseTree {
    fn switcher(self, u: &U) -> Self {
        match self {
            UseTree::Glob(UseGlob { star_token }) => {
                return UseTree::Glob(UseGlob { star_token });
            }
            UseTree::Group(UseGroup { brace_token, items }) => {
                let items = items.switcher(u);
                return UseTree::Group(UseGroup { brace_token, items });
            }
            UseTree::Name(UseName { ident }) => {
                return UseTree::Name(UseName { ident });
            }
            UseTree::Path(UsePath {
                ident,
                tree,
                colon2_token,
            }) => {
                let tree = tree.switcher(u);
                return UseTree::Path(UsePath {
                    ident,
                    tree,
                    colon2_token,
                });
            }
            UseTree::Rename(UseRename {
                ident,
                as_token,
                rename,
            }) => {
                return UseTree::Rename(UseRename {
                    ident,
                    as_token,
                    rename,
                });
            }
        }
    }
}

/// FieldsNamed
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for FieldsNamed {
    fn switcher(self, u: &U) -> Self {
        let FieldsNamed { brace_token, named } = self;
        let named = named.switcher(u);
        return FieldsNamed { brace_token, named };
    }
}

/// Fields
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Fields {
    fn switcher(self, u: &U) -> Self {
        match self {
            Fields::Named(fields_named) => {
                let fields_named = fields_named.switcher(u);
                return Fields::Named(fields_named);
            }
            Fields::Unnamed(FieldsUnnamed {
                paren_token,
                unnamed,
            }) => {
                let unnamed = unnamed.switcher(u);
                return Fields::Unnamed(FieldsUnnamed {
                    paren_token,
                    unnamed,
                });
            }
            Fields::Unit => self,
        }
    }
}

/// Field
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Field {
    fn switcher(self, u: &U) -> Self {
        let Field {
            attrs,
            vis,
            ident,
            colon_token,
            ty,
            mutability,
        } = self;
        let attrs = attrs.switcher(u);
        let ty = ty.switcher(u);
        return Field {
            attrs,
            vis,
            ident,
            colon_token,
            ty,
            mutability,
        };
    }
}

/// ImplItem
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for ImplItem {
    fn switcher(self, u: &U) -> Self {
        match self {
            ImplItem::Const(ImplItemConst {
                attrs,
                vis,
                defaultness,
                const_token,
                ident,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
                generics,
            }) => {
                let attrs = attrs.switcher(u);
                let ty = ty.switcher(u);
                let expr = expr.switcher(u);
                let generics = generics.switcher(u);
                return ImplItem::Const(ImplItemConst {
                    attrs,
                    vis,
                    defaultness,
                    const_token,
                    ident,
                    colon_token,
                    ty,
                    eq_token,
                    expr,
                    semi_token,
                    generics,
                });
            }
            ImplItem::Macro(ImplItemMacro {
                attrs,
                mac,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let mac = u(mac);
                return parse_quote! {
                    #( #attrs )*
                    #mac
                    #semi_token
                };
            }
            ImplItem::Type(ImplItemType {
                attrs,
                vis,
                defaultness,
                type_token,
                ident,
                generics,
                eq_token,
                ty,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let ty = ty.switcher(u);
                return ImplItem::Type(ImplItemType {
                    attrs,
                    vis,
                    defaultness,
                    type_token,
                    ident,
                    generics,
                    eq_token,
                    ty,
                    semi_token,
                });
            }
            ImplItem::Verbatim(tts) => {
                return ImplItem::Verbatim(tts);
            }
            ImplItem::Fn(ImplItemFn {
                attrs,
                vis,
                defaultness,
                sig,
                block,
            }) => {
                let attrs = attrs.switcher(u);
                let sig = sig.switcher(u);
                let block = block.switcher(u);
                return ImplItem::Fn(ImplItemFn {
                    attrs,
                    vis,
                    defaultness,
                    sig,
                    block,
                });
            }
            _ => unreachable!(),
        }
    }
}

/// Signature
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Signature {
    fn switcher(self, u: &U) -> Self {
        let Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        } = self;
        let generics = generics.switcher(u);
        let inputs = inputs.switcher(u);
        let output = output.switcher(u);
        return Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        };
    }
}

/// FnArg
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for FnArg {
    fn switcher(self, u: &U) -> Self {
        match self {
            FnArg::Receiver(Receiver {
                attrs,
                reference,
                mutability,
                self_token,
                colon_token,
                ty,
            }) => {
                let attrs = attrs.switcher(u);
                let ty = ty.switcher(u);
                return FnArg::Receiver(Receiver {
                    attrs,
                    reference,
                    mutability,
                    self_token,
                    colon_token,
                    ty,
                });
            }
            FnArg::Typed(PatType {
                attrs,
                pat,
                colon_token,
                ty,
            }) => {
                let attrs = attrs.switcher(u);
                let pat = pat.switcher(u);
                let ty = ty.switcher(u);
                return FnArg::Typed(PatType {
                    attrs,
                    pat,
                    colon_token,
                    ty,
                });
            }
        }
    }
}

/// Stmt
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Stmt {
    fn switcher(self, u: &U) -> Self {
        match self {
            Stmt::Local(local) => {
                let local = local.switcher(u);
                return Stmt::Local(local);
            }
            Stmt::Item(item) => {
                let item = item.switcher(u);
                return Stmt::Item(item);
            }
            Stmt::Expr(expr, semi_token) => {
                let expr = expr.switcher(u);
                return Stmt::Expr(expr, semi_token);
            }
            Stmt::Macro(mut stmt) => {
                stmt.attrs = stmt.attrs.switcher(u);
                let mac = u(stmt.mac);
                return parse_quote! { #mac };
            }
        }
    }
}

/// Generics
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Generics {
    fn switcher(self, u: &U) -> Self {
        let Generics {
            lt_token,
            params,
            gt_token,
            where_clause,
        } = self;
        let params = params.switcher(u);
        let where_clause = where_clause.switcher(u);
        return Generics {
            lt_token,
            params,
            gt_token,
            where_clause,
        };
    }
}

/// GenericParam
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for GenericParam {
    fn switcher(self, u: &U) -> Self {
        match self {
            GenericParam::Type(TypeParam {
                attrs,
                ident,
                colon_token,
                bounds,
                eq_token,
                default,
            }) => {
                let attrs = attrs.switcher(u);
                let bounds = bounds.switcher(u);
                let default = default.switcher(u);
                return GenericParam::Type(TypeParam {
                    attrs,
                    ident,
                    colon_token,
                    bounds,
                    eq_token,
                    default,
                });
            }
            _ => self,
        }
    }
}

/// WhereClause
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for WhereClause {
    fn switcher(self, u: &U) -> Self {
        let WhereClause {
            where_token,
            predicates,
        } = self;
        let predicates = predicates.switcher(u);
        return WhereClause {
            where_token,
            predicates,
        };
    }
}

/// WherePredicate
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for WherePredicate {
    fn switcher(self, u: &U) -> Self {
        match self {
            WherePredicate::Type(type_pred) => {
                let type_pred = type_pred.switcher(u);
                return WherePredicate::Type(type_pred);
            }
            _ => self,
        }
    }
}

/// PredicateType
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for PredicateType {
    fn switcher(self, u: &U) -> Self {
        let PredicateType {
            lifetimes,
            bounded_ty,
            colon_token,
            bounds,
            ..
        } = self;
        let bounded_ty = bounded_ty.switcher(u);
        let bounds = bounds.switcher(u);
        return PredicateType {
            lifetimes,
            bounded_ty,
            colon_token,
            bounds,
        };
    }
}

/// Local
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Local {
    fn switcher(self, u: &U) -> Self {
        let Local {
            attrs,
            let_token,
            pat,
            init,
            semi_token,
        } = self;
        let attrs = attrs.switcher(u);
        let pat = pat.switcher(u);
        let init = init.switcher(u);
        return Local {
            attrs,
            let_token,
            pat,
            init,
            semi_token,
        };
    }
}

/// LocalInit
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for LocalInit {
    fn switcher(self, u: &U) -> Self {
        let LocalInit {
            eq_token,
            expr,
            diverge,
        } = self;
        let expr = expr.switcher(u);
        let diverge = diverge.switcher(u);
        return LocalInit {
            eq_token,
            expr,
            diverge,
        };
    }
}

/// FieldPat
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for FieldPat {
    fn switcher(self, u: &U) -> Self {
        let FieldPat {
            attrs,
            member,
            colon_token,
            pat,
        } = self;
        let attrs = attrs.switcher(u);
        let pat = pat.switcher(u);
        return FieldPat {
            attrs,
            member,
            colon_token,
            pat,
        };
    }
}

/// PatRest
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for PatRest {
    fn switcher(self, u: &U) -> Self {
        let PatRest { attrs, dot2_token } = self;
        let attrs = attrs.switcher(u);
        return PatRest { attrs, dot2_token };
    }
}

/// Path
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Path {
    fn switcher(self, u: &U) -> Self {
        let Path {
            leading_colon,
            segments,
        } = self;
        let segments = segments.switcher(u);
        return Path {
            leading_colon,
            segments,
        };
    }
}

/// PathSegment
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for PathSegment {
    fn switcher(self, u: &U) -> Self {
        let PathSegment { ident, arguments } = self;
        let arguments = arguments.switcher(u);
        return PathSegment { ident, arguments };
    }
}

/// PathArguments
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for PathArguments {
    fn switcher(self, u: &U) -> Self {
        match self {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token,
                lt_token,
                args,
                gt_token,
            }) => {
                let args = args.switcher(u);
                return PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token,
                    lt_token,
                    args,
                    gt_token,
                });
            }
            PathArguments::Parenthesized(ParenthesizedGenericArguments {
                paren_token,
                inputs,
                output,
            }) => {
                let inputs = inputs.switcher(u);
                let output = output.switcher(u);
                return PathArguments::Parenthesized(ParenthesizedGenericArguments {
                    paren_token,
                    inputs,
                    output,
                });
            }
            _ => self,
        }
    }
}

/// GenericArgument
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for GenericArgument {
    fn switcher(self, u: &U) -> Self {
        match self {
            GenericArgument::Type(ty) => {
                let ty = ty.switcher(u);
                return GenericArgument::Type(ty);
            }
            _ => self,
        }
    }
}

/// BareFnArg
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for BareFnArg {
    fn switcher(self, u: &U) -> Self {
        let BareFnArg {
            name, ty, attrs, ..
        } = self;
        let ty = ty.switcher(u);
        let attrs = attrs.switcher(u);
        return BareFnArg { name, ty, attrs };
    }
}

/// ReturnType
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for ReturnType {
    fn switcher(self, u: &U) -> Self {
        match self {
            ReturnType::Type(arrow, ty) => {
                let ty = ty.switcher(u);
                return ReturnType::Type(arrow, ty);
            }
            _ => self,
        }
    }
}

/// TypeParamBound
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for TypeParamBound {
    fn switcher(self, u: &U) -> Self {
        match self {
            TypeParamBound::Trait(trait_bound) => {
                let trait_bound = trait_bound.switcher(u);
                return TypeParamBound::Trait(trait_bound);
            }
            _ => self,
        }
    }
}

/// TraitBound
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for TraitBound {
    fn switcher(self, u: &U) -> Self {
        let TraitBound {
            paren_token,
            modifier,
            lifetimes,
            path,
        } = self;
        let path = path.switcher(u);
        return TraitBound {
            paren_token,
            modifier,
            lifetimes,
            path,
        };
    }
}

/// TraitItem
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for TraitItem {
    fn switcher(self, u: &U) -> Self {
        match self {
            TraitItem::Const(TraitItemConst {
                attrs,
                const_token,
                ident,
                colon_token,
                ty,
                default,
                semi_token,
                generics,
            }) => {
                let attrs = attrs.switcher(u);
                let ty = ty.switcher(u);
                let default = default.switcher(u);
                let generics = generics.switcher(u);
                return TraitItem::Const(TraitItemConst {
                    attrs,
                    const_token,
                    ident,
                    colon_token,
                    ty,
                    default,
                    semi_token,
                    generics,
                });
            }
            TraitItem::Macro(TraitItemMacro {
                attrs,
                mac,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let mac = u(mac);
                return parse_quote! {
                    #( #attrs )*
                    #mac
                    #semi_token
                };
            }
            TraitItem::Type(TraitItemType {
                attrs,
                type_token,
                ident,
                generics,
                colon_token,
                bounds,
                default,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let generics = generics.switcher(u);
                let bounds = bounds.switcher(u);
                let default = default.switcher(u);
                return TraitItem::Type(TraitItemType {
                    attrs,
                    type_token,
                    ident,
                    generics,
                    colon_token,
                    bounds,
                    default,
                    semi_token,
                });
            }
            TraitItem::Verbatim(tts) => {
                return TraitItem::Verbatim(tts);
            }
            TraitItem::Fn(TraitItemFn {
                attrs,
                sig,
                default,
                semi_token,
            }) => {
                let attrs = attrs.switcher(u);
                let sig = sig.switcher(u);
                let default = default.switcher(u);
                return TraitItem::Fn(TraitItemFn {
                    attrs,
                    sig,
                    default,
                    semi_token,
                });
            }
            _ => unreachable!(),
        }
    }
}

/// Block
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Block {
    fn switcher(self, u: &U) -> Self {
        let Block { brace_token, stmts } = self;
        let stmts = stmts.switcher(u);
        return Block { brace_token, stmts };
    }
}

/// FieldValue
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for FieldValue {
    fn switcher(self, u: &U) -> Self {
        let FieldValue {
            member,
            colon_token,
            expr,
            attrs,
        } = self;
        let attrs = attrs.switcher(u);
        let expr = expr.switcher(u);
        return FieldValue {
            attrs,
            member,
            colon_token,
            expr,
        };
    }
}

/// Arm
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Arm {
    fn switcher(self, u: &U) -> Self {
        let Arm {
            attrs,
            pat,
            guard,
            fat_arrow_token,
            body,
            comma,
        } = self;
        let attrs = attrs.switcher(u);
        let pat = pat.switcher(u);
        let guard = guard.switcher(u);
        let body = body.switcher(u);
        return Arm {
            attrs,
            pat,
            guard,
            fat_arrow_token,
            body,
            comma,
        };
    }
}

/// Attribute
impl<U: Fn(Macro) -> TokenStream> Switcher<U> for Attribute {
    fn switcher(self, _u: &U) -> Self {
        let Attribute {
            pound_token,
            style,
            bracket_token,
            meta,
        } = self;
        // let meta = match meta {
        //     Meta::Macro(mac) => {
        //         let mac = u(mac);
        //         return parse_quote! { #mac };
        //     }
        //     _ => meta,
        // };
        return Attribute {
            pound_token,
            style,
            bracket_token,
            meta,
        };
    }
}
