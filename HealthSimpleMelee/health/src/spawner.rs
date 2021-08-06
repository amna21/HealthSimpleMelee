use crate::prelude::*;

pub fn spawn_player(ecs : &mut World, pos : Point) {
    //START: player_health
    ecs.push(
        (Player, 
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            },
            //START_HIGHLIGHT
            Health{ current: 20, max: 20 }
            //END_HIGHLIGHT
        )
    );
    //END: player_health
}

//START: mob_health
pub fn spawn_monster(
    ecs: &mut World, 
    rng: &mut RandomNumberGenerator, 
    pos : Point
) {
    let (hp, name, glyph) = match rng.roll_dice(1,10) { // <callout id="co.hsmh.destructure" />
        1..=8 => goblin(),// <callout id="co.hsmh.weighting" />
        _ => orc()
    };

    ecs.push(
        (Enemy,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            MovingRandomly{},
            Health{current: hp, max: hp},// <callout id="co.hsmh.healthcomp" />
            Name(name)// <callout id="co.hsmh.namecomp" />
        )
    );
}
//END: mob_health

//START: goblin
fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}
//END: goblin

//START: orc
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
//END: orc
