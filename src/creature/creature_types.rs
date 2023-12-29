use crate::creature::Creature;

pub fn create_testcreature() -> Creature {
    Creature::new(String::from("testcreature"))
}
