use crate::creature::action::Action;
use crate::creature::perception::{Awareness, Perception};
use crate::world::Layer;

#[derive(Clone, Copy)]
pub enum Behavior {
    PlayerControlled(u128),
    Inactive,
}

pub struct AI {
    pub owner_id: u128,
    pub perception: Perception,

    default_wander_behavior: Behavior,
    default_combat_behavior: Behavior,
    override_behavior: Option<Behavior>,
}

impl AI {
    pub fn new(id: u128) -> Self {
        Self {
            owner_id: id,
            perception: Perception::new(id),
            default_wander_behavior: Behavior::Inactive,
            default_combat_behavior: Behavior::Inactive,
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

pub fn act(actor: &AI, _layer: &Layer) -> Action {
    let actor_behavior = match actor.perception.get_awareness() {
        Awareness::Wander => actor.get_wander_behavior(),
        Awareness::Combat => actor.get_combat_behavior(),
    };

    match actor_behavior {
        Behavior::PlayerControlled(_) => {
            panic!("AI can't control player-controlled characters");
        }
        Behavior::Inactive => Action::Idle,
    }
}
