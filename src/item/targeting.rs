use crate::grid::gridalgos;
use crate::grid::GridSquare;
use crate::world::Layer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum TargetKind {
    Square(SquareParams),
    Burst(BurstParams),
    BurstVolumetric(BurstVolumetricParams),
    Cone(ConeParams),
    Emanation(EmanationParams),
    Line(LineParams),
    LineReflecting(LineReflectingParams),
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct SquareParams {
    pub range: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct BurstParams {
    pub range: i32,
    pub radius: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct BurstVolumetricParams {
    pub range: i32,
    pub volume: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct ConeParams {
    pub length: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct EmanationParams {
    pub radius: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct LineParams {
    pub length: i32,
    pub width: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct LineReflectingParams {
    pub length: i32,
    pub width: i32,
}

#[derive(Debug)]
pub enum TargetingError {
    OutOfRange,
}

pub fn get_targeted_squares(
    target: GridSquare,
    target_kind: TargetKind,
    actor_id: u128,
    layer: &Layer,
) -> Result<Vec<GridSquare>, TargetingError> {
    let actor = layer.creatures.get(&actor_id).expect("actor should exist");
    let actor_pos = actor.get_position();

    match target_kind {
        TargetKind::Square(params) => {
            let dist = gridalgos::distance(target, actor_pos);
            if dist > params.range {
                return Err(TargetingError::OutOfRange);
            }
            return Ok(vec![target]);
        }
        _ => panic!("unimplemented target type"),
    }
}

pub fn get_affected_creatures(targeted_squares: Vec<GridSquare>, layer: &Layer) -> Vec<u128> {
    let mut creatures: Vec<u128> = Vec::new();

    for square in targeted_squares {
        let mut creatures_at_sq = layer.get_living_creatures_at(square);
        creatures.append(&mut creatures_at_sq);
    }

    creatures
}
