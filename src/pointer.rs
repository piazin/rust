pub trait Pointer {
    fn ptr(&self) -> String;
}

impl Pointer for String {
    fn ptr(&self) -> String {
        let ptr: *const String = self;
        format!("{:?}", ptr)
    }
}
