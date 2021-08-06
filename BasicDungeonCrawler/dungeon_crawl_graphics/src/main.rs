#![warn(clippy::pedantic)]

// START: prelude
mod map;
mod map_builder;
mod player;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    //START: constants
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    //END: constants
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}
// END: prelude

use prelude::*;

//START: camstate
struct State {
    map: Map,
    player: Player,
    //START_HIGHLIGHT
    camera: Camera
    //END_HIGHLIGHT
}
//END: camstate

impl State {
    //START: consumemap_builder
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map : map_builder.map,
            player: Player::new(map_builder.player_start),
            //START_HIGHLIGHT
            camera: Camera::new(map_builder.player_start)
            //END_HIGHLIGHT
        }
    }
    //END: consumemap_builder
}

impl GameState for State {
    //START: tick
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
    //END: tick
}

fn main() -> BError {
    //START: layers
    let context = BTermBuilder::new()// <callout id="co.dungeongfx.newterm" />
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // <callout id="co.dungeongfx.dimensions" />
        .with_tile_dimensions(32, 32) // <callout id="co.dungeongfx.tiledimensions" />
        .with_resource_path("resources/") // <callout id="co.dungeongfx.resources" />
        .with_font("dungeonfont.png", 32, 32) // <callout id="co.dungeongfx.font" />
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // <callout id="co.dungeongfx.con1" />
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
            "dungeonfont.png") // <callout id="co.dungeongfx.con2" />
        .build()?;
    //END: layers

    main_loop(context, State::new())
}
