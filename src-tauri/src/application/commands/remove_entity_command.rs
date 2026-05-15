use crate::application::commands::base_command::Command;
use crate::domain::entities::encounter_entity::EncounterEntity;

#[derive(Debug)]
pub struct RemoveEntityCommand {
    pub entity: *mut EncounterEntity,
    was_active: bool,
}

impl RemoveEntityCommand {
    pub fn new(entity: *mut EncounterEntity) -> Self {
        unsafe {
            Self { entity, was_active: (*entity).is_active }
        }
    }
}

impl Command for RemoveEntityCommand {
    fn execute(&mut self) {
        unsafe {
            (*self.entity).is_active = false;
        }
    }

    fn undo(&mut self) {
        unsafe {
            (*self.entity).is_active = self.was_active;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_deactivates_entity() {
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
        let mut cmd = RemoveEntityCommand::new(&mut entity as *mut _);
        cmd.execute();
        unsafe { assert!(!(*cmd.entity).is_active); }
    }

    #[test]
    fn undo_restores_active_state() {
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
        let mut cmd = RemoveEntityCommand::new(&mut entity as *mut _);
        cmd.execute();
        cmd.undo();
        unsafe { assert!((*cmd.entity).is_active); }
    }
}