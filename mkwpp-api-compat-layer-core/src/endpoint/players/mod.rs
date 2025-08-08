use crate::endpoint::{Root, Scope};

pub struct PlayersScope;
impl Scope for PlayersScope {
    const PATH: &'static str = "/players";

    type OuterScope = Root;
}
