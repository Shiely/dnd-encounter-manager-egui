// DOMAIN LAYER: stdlib imports only.
use crate::domain::entities::encounter_entity::EncounterEntity;

#[derive(Debug, Clone)]
pub struct Encounter {
    pub encounter_id: String,
    pub entities: Vec<EncounterEntity>,
    pub current_turn_index: usize,
    pub round_number: u32,
    pub saved_at: Option<String>, // Using String for now to avoid extra dependencies
}