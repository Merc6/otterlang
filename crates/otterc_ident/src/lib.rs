use std::ops::{Deref, DerefMut};

use ustr::Ustr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(Ustr);

impl Deref for Identifier {
    type Target = Ustr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Identifier {
    pub fn new(ident: &str) -> Self {
        Self(Ustr::from(ident))
    }
}
