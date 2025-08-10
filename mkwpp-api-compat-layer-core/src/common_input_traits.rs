
pub trait HasSessionToken {
    fn get_session_token<'a>(&self) -> &'a str;
}