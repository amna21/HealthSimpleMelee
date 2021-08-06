#![warn(clippy::pedantic)]

// START: prelude
mod map; // <callout id="co.dungeonmap.mod" />

mod prelude { // <callout id="co.dungeonmap.pubmod" />
    pub use bracket_lib::prelude::*; // <callout id="co.dungeonmap.bracketprelude" />
    pub const SCREEN_WIDTH: i32 = 80; // <callout id="co.dungeonmap.pubconst" />
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*; // <callout id="co.dungeonmap.cratemap" />
}

use prelude::*; // <callout id="co.dungeonmap.useprelude" />
                // END: prelude

// START: mapstate
struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)// <callout id="co.bdc.fps_cap" />
        .build()?;

    main_loop(context, State::new())
}
// END: mapstate
