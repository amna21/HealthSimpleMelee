pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

//START: EnemyTag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
//END: EnemyTag

