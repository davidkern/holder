use crate::Holder;

pub struct A<'a> {
    pub(crate) holder: &'a mut Holder,
}

impl<'a> A<'a> {
    pub fn new<T>(&mut self, value: T) -> T {
        value
    }
}

pub fn example<'a>(a: A<'a>) {
    //let x = a.new::<u8>(0);
}