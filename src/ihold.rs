pub trait IHold<Marker> {
    fn mark(&mut self) -> Result<Marker, ()>;
    unsafe fn release(&mut self, Marker);
}
