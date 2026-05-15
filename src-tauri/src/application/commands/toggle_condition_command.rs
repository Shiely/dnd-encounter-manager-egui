use crate::application::commands::base_command::Command;
use crate::domain::entities::encounter_entity::EncounterEntity;
use crate::domain::value_objects::condition::Condition;

#[derive(Debug)]
pub struct ToggleConditionCommand {
    pub entity: *mut EncounterEntity,
    condition: Condition,
}

impl ToggleConditionCommand {
    pub fn new(entity: *mut EncounterEntity, condition: Condition) -> Self {
        Self { entity, condition }
    }
}

impl Command for ToggleConditionCommand {
    fn execute(&mut self) {
        unsafe {
            let conditions = &mut (*self.entity).conditions;
            if conditions.contains(&self.condition) {
                conditions.retain(|c| *c != self.condition);
            } else {
                conditions.push(self.condition);
            }
        }
    }

    fn undo(&mut self) {
        // For simplicity in early phase, we re-toggle
        self.execute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_adds_condition() {
        let mut entity = EncounterEntity {
            instance_id: "test".to_string(),
            display_name: "Test".to_string(),
            entity_type: "monster".to_string(),
            initiative: 10,
            current_hp: Some(10),
            max_hp: Some(10),
            monster_id: None,
            conditions: vec![],
            is_active: true,
        };
        let mut cmd = ToggleConditionCommand::new(&mut entity as *mut _, Condition::Frightened);
        cmd.execute();
        unsafe { assert!((*cmd.entity).conditions.contains(&Condition::Frightened)); }
    }
}