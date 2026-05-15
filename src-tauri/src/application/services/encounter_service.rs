use crate::domain::entities::encounter::Encounter;
use crate::domain::entities::encounter_entity::EncounterEntity;

pub struct EncounterService {
    pub encounter: Encounter,
}

impl EncounterService {
    pub fn new(encounter: Encounter) -> Self {
        Self { encounter }
    }

    pub fn add_entity(&mut self, entity: EncounterEntity) {
        self.encounter.entities.push(entity);
    }

    pub fn get_active_entities(&self) -> Vec<&EncounterEntity> {
        self.encounter.entities.iter().filter(|e| e.is_active).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::encounter::Encounter;

    #[test]
    fn add_entity_increases_count() {
        let mut service = EncounterService::new(Encounter {
            encounter_id: "test".to_string(),
            entities: vec![],
            current_turn_index: 0,
            round_number: 1,
            saved_at: None,
        });

        let entity = EncounterEntity {
            instance_id: "e1".to_string(),
            display_name: "Goblin #1".to_string(),
            entity_type: "monster".to_string(),
            initiative: 10,
            current_hp: Some(7),
            max_hp: Some(7),
            monster_id: Some("goblin".to_string()),
            conditions: vec![],
            is_active: true,
        };

        service.add_entity(entity);
        assert_eq!(service.encounter.entities.len(), 1);
    }
}