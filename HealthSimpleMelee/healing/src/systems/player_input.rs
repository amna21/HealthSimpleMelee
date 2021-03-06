use crate::prelude::*;

//START: health_query
#[system]
#[read_component(Point)]
#[read_component(Player)]
//START_HIGHLIGHT
#[read_component(Enemy)]
#[write_component(Health)]
//END_HIGHLIGHT
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] turn_state : &mut TurnState
) {
    //END: health_query
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
                .unwrap();

        //START: did_something
        let mut did_something = false;
        if delta.x !=0 || delta.y != 0 {
        //END: did_something

            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == destination
                })
                //START: did_combat
                .for_each(|(entity, _) | {
                    hit_something = true;
                    did_something = true;
                //END: did_combat

                    commands
                        .push(((), WantsToAttack{
                            attacker: player_entity,
                            victim: *entity,
                        }));
                });

            //START: did_move
            if !hit_something {
                did_something = true;
                commands
                //END: did_move
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }
        };
        //START: heal
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current+1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
        //END: heal
    }
}
