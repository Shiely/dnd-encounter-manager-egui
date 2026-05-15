// src-tauri/src/domain/commands.rs
use serde::{Deserialize, Serialize};
use crate::domain::{Combatant, Condition};

pub trait Command: Send + Sync {
    fn execute(&mut self, combatant: &mut Combatant);
    fn undo(&mut self, combatant: &mut Combatant);
    fn description(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageCommand {
    pub amount: i32,
    pub previous_hp: Option<i32>,
    pub previous_temp: Option<i32>,
}

impl DamageCommand {
    pub fn new(amount: i32) -> Self {
        Self {
            amount,
            previous_hp: None,
            previous_temp: None,
        }
    }
}

impl Command for DamageCommand {
    fn execute(&mut self, combatant: &mut Combatant) {
        self.previous_hp = Some(combatant.hit_points.current);
        self.previous_temp = Some(combatant.hit_points.temporary);
        combatant.hit_points.apply_damage(self.amount);
    }

    fn undo(&mut self, combatant: &mut Combatant) {
        if let Some(hp) = self.previous_hp {
            combatant.hit_points.current = hp;
        }
        if let Some(temp) = self.previous_temp {
            combatant.hit_points.temporary = temp;
        }
    }

    fn description(&self) -> String {
        format!("Damage {} HP", self.amount)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealCommand {
    pub amount: i32,
    pub previous_hp: Option<i32>,
}

impl HealCommand {
    pub fn new(amount: i32) -> Self {
        Self {
            amount,
            previous_hp: None,
        }
    }
}

impl Command for HealCommand {
    fn execute(&mut self, combatant: &mut Combatant) {
        self.previous_hp = Some(combatant.hit_points.current);
        combatant.hit_points.heal(self.amount);
    }

    fn undo(&mut self, combatant: &mut Combatant) {
        if let Some(hp) = self.previous_hp {
            combatant.hit_points.current = hp;
        }
    }

    fn description(&self) -> String {
        format!("Heal {} HP", self.amount)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddConditionCommand {
    pub condition: Condition,
    pub was_present: bool,
}

impl AddConditionCommand {
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            was_present: false,
        }
    }
}

impl Command for AddConditionCommand {
    fn execute(&mut self, combatant: &mut Combatant) {
        self.was_present = combatant.conditions.contains(&self.condition);
        if !self.was_present {
            combatant.conditions.push(self.condition);
        }
    }

    fn undo(&mut self, combatant: &mut Combatant) {
        if !self.was_present {
            combatant.conditions.retain(|c| c != &self.condition);
        }
    }

    fn description(&self) -> String {
        format!("Add condition: {}", self.condition.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveConditionCommand {
    pub condition: Condition,
    pub was_present: bool,
}

impl RemoveConditionCommand {
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            was_present: false,
        }
    }
}

impl Command for RemoveConditionCommand {
    fn execute(&mut self, combatant: &mut Combatant) {
        self.was_present = combatant.conditions.contains(&self.condition);
        combatant.conditions.retain(|c| c != &self.condition);
    }

    fn undo(&mut self, combatant: &mut Combatant) {
        if self.was_present {
            combatant.conditions.push(self.condition);
        }
    }

    fn description(&self) -> String {
        format!("Remove condition: {}", self.condition.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeInitiativeCommand {
    pub new_value: i32,
    pub old_value: Option<i32>,
    pub old_tiebreaker: Option<u64>,
}

impl ChangeInitiativeCommand {
    pub fn new(new_value: i32) -> Self {
        Self {
            new_value,
            old_value: None,
            old_tiebreaker: None,
        }
    }
}

impl Command for ChangeInitiativeCommand {
    fn execute(&mut self, combatant: &mut Combatant) {
        self.old_value = Some(combatant.initiative.value);
        self.old_tiebreaker = Some(combatant.initiative.tiebreaker);
        combatant.initiative.value = self.new_value;
    }

    fn undo(&mut self, combatant: &mut Combatant) {
        if let Some(val) = self.old_value {
            combatant.initiative.value = val;
        }
        if let Some(tb) = self.old_tiebreaker {
            combatant.initiative.tiebreaker = tb;
        }
    }

    fn description(&self) -> String {
        format!("Change initiative to {}", self.new_value)
    }
}

// Undo stack
#[derive(Debug, Default)]
pub struct UndoStack {
    pub commands: Vec<Box<dyn Command>>,
    pub max_size: usize,
}

impl UndoStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            commands: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, command: Box<dyn Command>) {
        if self.commands.len() >= self.max_size {
            self.commands.remove(0);
        }
        self.commands.push(command);
    }

    pub fn undo_last(&mut self, combatant: &mut Combatant) -> Option<String> {
        if let Some(mut cmd) = self.commands.pop() {
            cmd.undo(combatant);
            Some(cmd.description())
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}