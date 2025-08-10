pub trait InputFromActix {
    fn get_from_request() -> Self;
}

impl InputFromActix for () {
    fn get_from_request() -> Self {}
}
