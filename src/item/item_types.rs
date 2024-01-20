use crate::item::effect::Effect;
use crate::item::statistics::{Rarity, Statistics};
use crate::item::targeting::*;
use crate::item::{Item, ItemKind};

pub fn create_testitem(level: i32, rarity: Rarity) -> (Item, Effect) {
    let name = String::from("testitem");
    let description = String::from("A fancy description");

    let target_params = BurstParams {
        range: 10,
        radius: 4,
    };

    let target = TargetKind::Area(AreaKind::Burst(target_params));
    let kind = ItemKind::Active(target);
    let stats = Statistics::new(level, rarity);

    let effect = Effect {};

    (Item::new(name, description, kind, stats), effect)
}
