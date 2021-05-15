use crate::{Cataloging, DualStack, Shelf, Shelving};
use std::marker::PhantomData;

pub struct Library<'a> {
    shelf: (
        Shelf<u8, 1000>,
    ),
    catalog: (
        DualStack,
    ),
    _librarians: PhantomData<&'a ()>  
}

impl<'a> Library<'a> {
    pub fn alpha(&'a self) -> Librarian<'a, Shelf<u8, 1000>, DualStack> {
        Librarian {
            shelf: &self.shelf.0,
            catalog: &self.catalog.0,
        }
    }
}

pub struct Librarian<'a, S: Shelving, C: Cataloging> {
    shelf: &'a S,
    catalog: &'a C,
}
