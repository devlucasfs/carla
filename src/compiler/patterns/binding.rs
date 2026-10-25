use crate::compiler::{
    abi,
    iterator::ContextIterator,
    nesting::Context::Common,
    patterns::{
        expressions::{self, Compiled},
        types::CarlaType,
    },
    scanner::{
        Keywords::{Let, Var},
        TokenKind::{Comma, Identifier, Keyword, Semi, Walrus, Wild},
    },
    symbols::{Symbol, Symbols},
};

/// `let a, b := f();` / `var a, b := f();` in the header of an `if` / `while`.
///
/// The value is evaluated once, before the condition, and destructured:
/// `names[i]` receives `types[i]` (`None` is `_`: that element is ignored).
/// With a single name it binds the whole value, tuple or not.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    /// `let`: the names live in the `if`/`while` scope (condition, bodies, `else`).
    /// `var`: the names are declared in the parent scope and outlive the statement.
    pub scoped: bool,
    /// symbol names (`None` is `_`)
    pub names: Vec<Option<String>>,
    pub types: Vec<CarlaType>,
    pub value: Compiled,
}

/// `Some(true)` if the next token is `let`, `Some(false)` for `var`, `None` otherwise.
/// Nothing is consumed: the caller needs it to open the scope before `work` runs.
pub fn peek(it: &mut ContextIterator<'_>) -> Option<bool> {
    match it.peek() {
        Some(Common(tk)) if tk.kind == Keyword(Let) => Some(true),
        Some(Common(tk)) if tk.kind == Keyword(Var) => Some(false),
        _ => None,
    }
}

/// Reads `let|var names := value ;` and declares the names in the CURRENT scope.
/// For `let`, the caller must have pushed the statement scope already.
pub fn work(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Binding, String> {
    let Some(scoped) = peek(it) else {
        return Err("Expected `let` or `var`".into());
    };
    it.next();

    let mut names: Vec<Option<String>> = vec![];
    loop {
        let err = "Expected an identifier or `_` in the binding";
        let Some(Common(tk)) = it.next() else {
            return Err(err.into());
        };

        names.push(match &tk.kind {
            Identifier(id) => Some(id.clone()),
            Wild => None,
            _ => return Err(err.into()),
        });

        match it.next() {
            Some(Common(tk)) if tk.kind == Comma => continue,
            Some(Common(tk)) if tk.kind == Walrus => break,
            _ => return Err("Expected `,` or `:=` in the binding".into()),
        }
    }

    // parsed BEFORE the names exist: `let x := x` reads the outer `x`
    let value = expressions::parse(syms, it)?;

    match it.next() {
        Some(Common(tk)) if tk.kind == Semi => {}
        _ => {
            return Err(
                "Expected `;` after the binding, before the condition (`let a, b := f(); cond`)"
                    .into(),
            );
        }
    }

    let ty = value.expr().ty();
    let types = if names.len() == 1 {
        if ty.base.is_none() && ty.tuple.is_empty() && ty.levels == 0 {
            return Err("the value of this binding has no type (it returns nothing)".into());
        }
        vec![ty]
    } else if ty.tuple.is_empty() {
        return Err(format!(
            "`{}` isn't a tuple: it can't be destructured into {} names",
            ty.to_string(),
            names.len()
        ));
    } else if ty.tuple.len() != names.len() {
        return Err(format!(
            "`{}` has {} elements but the binding has {} names",
            ty.to_string(),
            ty.tuple.len(),
            names.len()
        ));
    } else {
        ty.tuple.clone()
    };

    // keep the symbol names: they differ from the source ones inside an `extend`
    let mut declared = Vec::with_capacity(names.len());
    for (name, ty) in names.iter().zip(&types) {
        declared.push(match name {
            Some(name) => Some(declare(syms, name, ty)?),
            None => None,
        });
    }

    Ok(Binding {
        scoped,
        names: declared,
        types,
        value,
    })
}

/// Declares a local variable in the current scope and returns its symbol name.
/// Same rule as `declaration.rs`: a known name is an error, except an unfold function.
pub fn declare(syms: &mut Symbols, name: &str, ty: &CarlaType) -> Result<String, String> {
    let full = abi::sym_name_by_abi(syms, name);

    if let Some(known) = syms.lookup(&full) {
        if !matches!(known, Symbol::UnfoldFunction(..)) {
            return Err(format!("{name} is already known"));
        }
        syms.erase(&full);
    }

    syms.push(
        &full,
        Symbol::Variable(Some(Box::new(ty.clone())), true, false),
    );
    Ok(full)
}
