pub trait Cataloging { }

pub struct DualStack;

impl DualStack {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Cataloging for DualStack { }
