use crate::creature::statistics::*;
use crate::creature::Creature;
use crate::grid::GridSquare;

pub fn create_testcreature(level: i32, position: GridSquare) -> Creature {
    let name = String::from("testcreature");
    let stats = Statistics {
        level: level,

        current_hp: new_hp(Proficiency::High),
        max_hp: new_hp(Proficiency::High),

        armor_class: new_defense(Proficiency::Moderate),
        fortitude_dc: new_defense(Proficiency::High),
        reflex_dc: new_defense(Proficiency::Moderate),
        will_dc: new_defense(Proficiency::Low),

        melee_attack: new_attack(Proficiency::High),
        ranged_attack: new_attack(Proficiency::Moderate),
        magic_attack: new_attack(Proficiency::Terrible),

        movement_speed: new_speed(Proficiency::Moderate),
        initiative: new_initiative(Proficiency::Moderate),

        n_actions: new_actions(Proficiency::Moderate),
    };

    Creature::new(name, position, stats)
}
