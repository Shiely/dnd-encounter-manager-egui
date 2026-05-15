use crate::application::commands::base_command::Command;
use crate::domain::entities::encounter_entity::EncounterEntity;
use crate::domain::rules::hp_rules::{apply_hp_edit, is_auto_remove};

#[derive(Debug)]
pub struct EditHpCommand {
    pub entity: *mut EncounterEntity,  // Using raw pointer for simplicity in early phase
    pub new_hp: i32,
    old_hp: i32,
}

impl EditHpCommand {
    pub fn new(entity: *mut EncounterEntity, new_hp: i32) -> Self {
        unsafe {
            let old_hp = (*entity).current_hp.unwrap_or(0);
            Self { entity, new_hp, old_hp }
        }
    }
}

impl Command for EditHpCommand {
    fn execute(&mut self) {
        unsafe {
            if let Some(hp) = (*self.entity).current_hp {
                (*self.entity).current_hp = Some(apply_hp_edit(hp, self.new_hp));
                if is_auto_remove((*self.entity).current_hp.unwrap()) {
                    (*self.entity).is_active = false;
                }
            }
        }
    }

    fn undo(&mut self) {
        unsafe {
            (*self.entity).current_hp = Some(self.old_hp);
            (*self.entity).is_active = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::encounter_entity::EncounterEntity;

    #[test]
    fn execute_changes_hp() {
        let mut entity = EncounterEntity {
            instance_id: "test".to_string(),
            display_name: "Test".to_string(),
            entity_type: "monster".to_string(),
            initiative: 10,
            current_hp: Some(20),
            max_hp: Some(20),
            monster_id: None,
            conditions: vec![],
            is_active: true,
        };

        let mut cmd = EditHpCommand::new(&mut entity as *mut _, 12);
        cmd.execute();
        unsafe { assert_eq!((*cmd.entity).current_hp, Some(12)); }
    }

    #[test]
    fn undo_restores_old_hp() {
        let mut entity = EncounterEntity {
            instance_id: "test".to_string(),
            display_name: "Test".to_string(),
            entity_type: "monster".to_string(),
            initiative: 10,
            current_hp: Some(20),
            max_hp: Some(20),
            monster_id: None,
            conditions: vec![],
            is_active: true,
        };

        let mut cmd = EditHpCommand::new(&mut entity as *mut _, 5);
        cmd.execute();
        cmd.undo();
        unsafe { assert_eq!((*cmd.entity).current_hp, Some(20)); }
    }
}