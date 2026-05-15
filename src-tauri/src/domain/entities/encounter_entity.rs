// DOMAIN LAYER: stdlib imports only.
use crate::domain::value_objects::condition::Condition;

#[derive(Debug, Clone)]
pub struct EncounterEntity {
    pub instance_id: String,
    pub display_name: String,
    pub entity_type: String, // "player" or "monster"
    pub initiative: i32,
    pub current_hp: Option<i32>,
    pub max_hp: Option<i32>,
    pub monster_id: Option<String>,
    pub conditions: Vec<Condition>,
    pub is_active: bool,
}