// src-tauri/src/domain/tests.rs
#[cfg(test)]
mod tests {
    use crate::domain::{
        Combatant, HitPoints, Condition, UndoStack,
        DamageCommand, HealCommand, Encounter,
    };

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