use crate::grid::{GridIntersection, GridSquare};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Target {
    Creatures(Vec<u128>),
    Intersection(GridIntersection),
    Square(GridSquare),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum TargetKind {
    Creature(CreatureParams),
    Area(AreaKind),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum AreaKind {
    Burst(BurstParams),
    BurstVolumetric(BurstVolumetricParams),
    Cone(ConeParams),
    Emanation(EmanationParams),
    Line(LineParams),
    LineReflecting(LineReflectingParams),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreatureParams {
    pub range: i32,
    pub max_number: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BurstParams {
    pub range: i32,
    pub radius: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BurstVolumetricParams {
    pub range: i32,
    pub volume: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConeParams {
    pub length: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EmanationParams {
    pub radius: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LineParams {
    pub length: i32,
    pub width: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LineReflectingParams {
    pub length: i32,
    pub width: i32,
}
