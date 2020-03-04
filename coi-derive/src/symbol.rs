use quote::format_ident;
use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const ARC: Symbol = Symbol("Arc");
pub const COI: Symbol = Symbol("coi");
pub const CRATE: Symbol = Symbol("crate");
pub const INJECT: Symbol = Symbol("inject");
pub const PROVIDES: Symbol = Symbol("provides");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, sym: &Symbol) -> bool {
        self == sym.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, sym: &Symbol) -> bool {
        *self == sym.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, sym: &Symbol) -> bool {
        self.is_ident(sym.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, sym: &Symbol) -> bool {
        self.is_ident(sym.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Symbol {
    pub(crate) fn as_ident(&self) -> Ident {
        format_ident!("{}", format!("{}", self))
    }
}
