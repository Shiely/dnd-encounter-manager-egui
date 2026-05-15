// DOMAIN LAYER: stdlib imports only.
use crate::domain::value_objects::condition::Condition;

pub fn toggle_condition(conditions: &[Condition], condition: Condition) -> Vec<Condition> {
    if conditions.contains(&condition) {
        conditions.iter().cloned().filter(|c| *c != condition).collect()
    } else {
        let mut list = conditions.to_vec();
        list.push(condition);
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn adds_when_missing() {
        let result = toggle_condition(&[], Condition::Frightened);
        assert_eq!(result.len(), 1);
    }

    #[test] fn removes_when_present() {
        let initial = vec![Condition::Poisoned, Condition::Frightened];
        let result = toggle_condition(&initial, Condition::Frightened);
        assert_eq!(result, vec![Condition::Poisoned]);
    }

    #[test] fn does_not_mutate_input() {
        let initial = vec![Condition::Blinded];
        let _ = toggle_condition(&initial, Condition::Blinded);
        assert_eq!(initial.len(), 1);
    }
}