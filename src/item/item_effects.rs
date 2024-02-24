use crate::check::{self, Outcome};
use crate::item::effect::{Duration, Effect, OngoingEffect};
use crate::item::statistics::{self, Rarity, Statistics};
use crate::item::targeting::Target;
use crate::stat::Proficiency;
use crate::world::Layer;

pub fn create_testeffect(level: i32, rarity: Rarity) -> Effect {
    let duration = Duration::Instantaneous;
    let mut stats = Statistics::new(level, rarity);
    stats.values.insert(
        "damage".to_string(),
        statistics::new_single_target_damage(Proficiency::High),
    );

    Effect::new(duration, stats, basic_melee_attack, None, None)
}

pub fn basic_melee_attack(
    effect: u128,
    owner: u128,
    target: Target,
    layer: &mut Layer,
) -> Option<OngoingEffect> {
    let effect = layer.effects.get(&effect).unwrap();
    let item_damage = effect.get_stat_value("damage");

    match target {
        Target::Creatures(creatures) => {
            for c in creatures {
                // Need to get attacker here instead of only once outside the loop
                // to satisfy the borrow checker
                let attacker = layer.creatures.get(&owner).unwrap();
                let defender = layer.creatures.get(&c).unwrap();
                let check = check::melee_attack_roll(&attacker.stats, &defender.stats);
                let damage_multiplier = match check.outcome {
                    Outcome::CriticalSuccess => 2,
                    Outcome::Success => 1,
                    _ => 0,
                };

                let attack_damage = item_damage * damage_multiplier;

                let defender = layer.creatures.get_mut(&c).unwrap();
                defender.change_hp(-attack_damage);
            }
        }
        _ => panic!("effect can only target creatures"),
    }

    None
}
