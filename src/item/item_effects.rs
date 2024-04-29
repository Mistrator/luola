use crate::check::{self, Outcome};
use crate::grid::GridSquare;
use crate::info_message::{AttackMessage, AttackResult, MessageType};
use crate::item::effect::{Duration, Effect, EffectResult};
use crate::item::statistics::{self, Rarity, Statistics};
use crate::item::targeting::{self, TargetKind};
use crate::item::ItemKind;
use crate::stat::Proficiency;
use crate::world::Layer;

fn get_target_kind(item_id: u128, layer: &Layer) -> TargetKind {
    let item = layer.items.get(&item_id).unwrap();
    match item.kind {
        ItemKind::Active(target_kind) => target_kind,
        ItemKind::Passive => panic!("passive items have no targets"),
    }
}

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
    effect_id: u128,
    attacker_id: u128,
    target: GridSquare,
    layer: &mut Layer,
) -> EffectResult {
    let effect = layer.effects.get(&effect_id).unwrap();
    let item_damage = effect.get_stat_value("damage");

    let target_kind = get_target_kind(effect_id, layer);
    #[rustfmt::skip]
    let target_squares = targeting::get_targeted_squares(target, target_kind, attacker_id, layer).expect("target should be in range");

    let affected_creatures = targeting::get_affected_creatures(target_squares, layer);

    let mut results: Vec<AttackResult> = Vec::new();

    for c in affected_creatures {
        // Need to get attacker here instead of only once outside the loop
        // to satisfy the borrow checker
        let attacker = layer.creatures.get(&attacker_id).unwrap();
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

        let result = AttackResult {
            target: c,
            check,
            damage: attack_damage,
        };

        results.push(result);
    }

    // item and its effect have the same id
    let message = AttackMessage {
        attacker: attacker_id,
        item: effect_id,
        results,
    };

    EffectResult {
        ongoing_effect: None,
        message: MessageType::Attack(message),
    }
}
