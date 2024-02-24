use crate::item::effect::Effect;
use crate::item::item_effects::*;
use crate::item::statistics::Rarity;
use crate::item::targeting::*;
use crate::item::{Item, ItemKind};

pub fn create_testitem(level: i32, rarity: Rarity) -> (Item, Effect) {
    let name = String::from("testitem");
    let description = String::from("A fancy description");

    let target = TargetKind::Area(AreaKind::Burst(BurstParams {
        range: 10,
        radius: 4,
    }));

    let kind = ItemKind::Active(target);
    let effect = create_testeffect(level, rarity);

    (Item::new(name, description, kind), effect)
}
