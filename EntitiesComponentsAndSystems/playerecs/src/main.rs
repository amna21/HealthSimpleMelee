#![warn(clippy::pedantic)]

//START: prelude
mod components;
mod spawner;
mod map;
mod map_builder;
mod systems;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

use prelude::*;
//END: prelude

//START: state
struct State {
    ecs : World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    //START: statenew
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
    //END: statenew
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}
//END: state

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // <callout id="co.dungeongfx.dimensions" />
        .with_tile_dimensions(32, 32) // <callout id="co.dungeongfx.tiledimensions" />
        .with_resource_path("resources/") // <callout id="co.dungeongfx.resources" />
        .with_font("dungeonfont.png", 32, 32) // <callout id="co.dungeongfx.font" />
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // <callout id="co.dungeongfx.con1" />
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // <callout id="co.dungeongfx.con2" />
        .build()?;

    main_loop(context, State::new())
}
