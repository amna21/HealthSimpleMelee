pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair, // <callout id="co.ecs.components.colorpair" />
    pub glyph : FontCharType // <callout id="co.ecs.components.fontchartype" />
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // <callout id="co.ecs.components.player" />


