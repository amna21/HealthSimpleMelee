use crate::prelude::*;

pub fn spawn_player(ecs : &mut World, pos : Point) { // <callout id="co.ecs.world" />
    ecs.push(// <callout id="co.ecs.components.push" />
        (
            Player,// <callout id="co.ecs.components.playertag" />
            pos, // <callout id="co.ecs.components.playerpos" />
            Render{// <callout id="co.ecs.components.playerrender" />
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            }
        )
    );
}
