// DOMAIN LAYER: stdlib imports only.

pub fn apply_hp_edit(_current_hp: i32, new_value: i32) -> i32 {
    new_value.max(0)
}

pub fn is_auto_remove(hp: i32) -> bool {
    hp <= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn normal_damage() { assert_eq!(apply_hp_edit(20, 12), 12); }
    #[test] fn to_zero() { assert_eq!(apply_hp_edit(7, 0), 0); }
    #[test] fn below_zero_clamped() { assert_eq!(apply_hp_edit(7, -3), 0); }
    #[test] fn temporary_hp_allowed() { assert_eq!(apply_hp_edit(10, 25), 25); }
    #[test] fn auto_remove_logic() {
        assert!(is_auto_remove(0));
        assert!(is_auto_remove(-1));
        assert!(!is_auto_remove(1));
    }
}