use crate::creature::statistics::*;
use crate::creature::Creature;
use crate::grid::GridSquare;
use crate::stat::Proficiency;

pub fn create_testcreature(level: i32, position: GridSquare) -> Creature {
    let name = String::from("testcreature");
    let stats = Statistics {
        level: level,

        max_hp: new_hp(Proficiency::High),

        fortitude_dc: new_defense(Proficiency::High),
        will_dc: new_defense(Proficiency::Low),

        melee_attack: new_attack(Proficiency::High),
        magic_attack: new_attack(Proficiency::Terrible),

        ..Statistics::get_default()
    };

    Creature::new(name, position, stats)
}
