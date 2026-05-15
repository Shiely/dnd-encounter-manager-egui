// src-tauri/src/domain/encounter.rs
use serde::{Deserialize, Serialize};
use crate::domain::{Combatant, UndoStack, Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub id: String,
    pub name: String,
    pub combatants: Vec<Combatant>,
    pub undo_stack: UndoStack,
    pub round: u32,
    pub current_turn_index: usize,
    pub is_active: bool,
}

impl Encounter {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            combatants: Vec::new(),
            undo_stack: UndoStack::new(5),
            round: 1,
            current_turn_index: 0,
            is_active: false,
        }
    }

    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);
    }

    pub fn remove_combatant(&mut self, id: &str) -> Option<Combatant> {
        if let Some(pos) = self.combatants.iter().position(|c| c.id == id) {
            Some(self.combatants.remove(pos))
        } else {
            None
        }
    }

    pub fn get_combatant_mut(&mut self, id: &str) -> Option<&mut Combatant> {
        self.combatants.iter_mut().find(|c| c.id == id)
    }

    pub fn sort_by_initiative(&mut self) {
        self.combatants.sort_by(|a, b| {
            b.initiative.value.cmp(&a.initiative.value)
                .then_with(|| b.initiative.tiebreaker.cmp(&a.initiative.tiebreaker))
        });
    }

    pub fn next_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }
        self.current_turn_index = (self.current_turn_index + 1) % self.combatants.len();
        if self.current_turn_index == 0 {
            self.round += 1;
        }
    }

    pub fn previous_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }
        if self.current_turn_index == 0 {
            self.round = self.round.saturating_sub(1);
            self.current_turn_index = self.combatants.len() - 1;
        } else {
            self.current_turn_index -= 1;
        }
    }

    pub fn execute_command(&mut self, mut command: Box<dyn Command>, target_id: &str) -> Option<String> {
        if let Some(combatant) = self.get_combatant_mut(target_id) {
            command.execute(combatant);
            let desc = command.description();
            self.undo_stack.push(command);
            Some(desc)
        } else {
            None
        }
    }

    pub fn undo_last(&mut self) -> Option<String> {
        // Get the last command's target before borrowing undo_stack
        let target_id = self.get_last_command_target();
        if let Some(id) = target_id {
            if let Some(combatant) = self.get_combatant_mut(&id) {
                return self.undo_stack.undo_last(combatant);
            }
        }
        None
    }

    fn get_last_command_target(&self) -> Option<String> {
        self.combatants.last().map(|c| c.id.clone())
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.sort_by_initiative();
        self.current_turn_index = 0;
        self.round = 1;
    }

    pub fn end(&mut self) {
        self.is_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Combatant, HitPoints, Condition, UndoStack, DamageCommand, HealCommand};

    #[test]
    fn test_hit_points_damage_and_heal() {
        let mut hp = HitPoints::new(20);
        assert_eq!(hp.current, 20);
        assert!(!hp.is_bloodied());

        hp.apply_damage(8);
        assert_eq!(hp.current, 12);
        assert!(hp.is_bloodied());

        hp.heal(5);
        assert_eq!(hp.current, 17);
        assert!(!hp.is_bloodied());
    }

    #[test]
    fn test_hit_points_death() {
        let mut hp = HitPoints::new(10);
        hp.apply_damage(15);
        assert!(hp.is_dead());
        assert_eq!(hp.current, -5);
    }

    #[test]
    fn test_undo_stack() {
        let mut stack = UndoStack::new(5);
        let mut combatant = Combatant::new("1".to_string(), "Test".to_string(), 20, true);

        let cmd = Box::new(DamageCommand::new(5));
        stack.push(cmd);

        assert_eq!(combatant.hit_points.current, 15);

        stack.undo_last(&mut combatant);
        assert_eq!(combatant.hit_points.current, 20);
    }

    #[test]
    fn test_encounter_add_and_sort() {
        let mut encounter = Encounter::new("e1".to_string(), "Test Encounter".to_string());

        let mut c1 = Combatant::new("1".to_string(), "Goblin".to_string(), 10, false);
        c1.initiative = crate::domain::Initiative::new(12);

        let mut c2 = Combatant::new("2".to_string(), "Hero".to_string(), 25, true);
        c2.initiative = crate::domain::Initiative::new(18);

        encounter.add_combatant(c1);
        encounter.add_combatant(c2);

        encounter.sort_by_initiative();

        assert_eq!(encounter.combatants[0].name, "Hero");
        assert_eq!(encounter.combatants[1].name, "Goblin");
    }

    #[test]
    fn test_encounter_turn_order() {
        let mut encounter = Encounter::new("e1".to_string(), "Test".to_string());
        encounter.add_combatant(Combatant::new("1".to_string(), "A".to_string(), 10, false));
        encounter.add_combatant(Combatant::new("2".to_string(), "B".to_string(), 10, false));

        encounter.start();
        assert_eq!(encounter.current_turn_index, 0);

        encounter.next_turn();
        assert_eq!(encounter.current_turn_index, 1);

        encounter.next_turn();
        assert_eq!(encounter.current_turn_index, 0);
        assert_eq!(encounter.round, 2);
    }
}