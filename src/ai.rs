use crate::creature::action::Action;
use crate::creature::perception::{Awareness, Perception};
use crate::world::Layer;

mod melee_brute;
mod wander;

#[derive(Clone, Copy)]
pub enum Behavior {
    PlayerControlled(u128),
    Inactive,
    Wandering,
    MeleeBrute,
}

pub struct AI {
    pub owner_id: u128,
    pub perception: Perception,

    default_wander_behavior: Behavior,
    default_combat_behavior: Behavior,
    override_behavior: Option<Behavior>,
}

impl AI {
    pub fn new(
        owner_id: u128,
        default_wander_behavior: Behavior,
        default_combat_behavior: Behavior,
    ) -> Self {
        Self {
            owner_id,
            perception: Perception::new(owner_id),
            default_wander_behavior,
            default_combat_behavior,
            override_behavior: None,
        }
    }

    pub fn get_wander_behavior(&self) -> Behavior {
        self.override_behavior
            .unwrap_or(self.default_wander_behavior)
    }

    pub fn get_combat_behavior(&self) -> Behavior {
        self.override_behavior
            .unwrap_or(self.default_combat_behavior)
    }

    pub fn set_override_behavior(&mut self, behavior: Behavior) {
        self.override_behavior = Some(behavior)
    }

    pub fn restore_default_behavior(&mut self) {
        self.override_behavior = None
    }

    pub fn get_controlling_player_id(&self) -> Option<u128> {
        match self.get_combat_behavior() {
            Behavior::PlayerControlled(player_id) => Some(player_id),
            _ => None,
        }
    }

    pub fn is_player_controlled(&self) -> bool {
        self.get_controlling_player_id().is_some()
    }
}

pub fn act(actor: &AI, layer: &Layer) -> Action {
    let actor_behavior = match actor.perception.get_awareness() {
        Awareness::Wander => actor.get_wander_behavior(),
        Awareness::Combat => actor.get_combat_behavior(),
    };

    match actor_behavior {
        Behavior::PlayerControlled(_) => {
            panic!("AI can't control player-controlled characters");
        }
        Behavior::Inactive => Action::Idle,
        Behavior::MeleeBrute => melee_brute::act(actor, layer),
        Behavior::Wandering => wander::act(actor, layer),
    }
}
