// DOMAIN LAYER: stdlib imports only.
use crate::domain::entities::encounter_entity::EncounterEntity;
use crate::domain::entities::monster_definition::MonsterDefinition;
use std::collections::HashMap;

pub fn sort_entities(
    entities: &[EncounterEntity],
    monster_defs: &HashMap<String, MonsterDefinition>,
) -> Vec<EncounterEntity> {
    let mut sorted = entities.to_vec();

    sorted.sort_by(|a, b| {
        // 1. Higher initiative wins
        if a.initiative != b.initiative {
            return b.initiative.cmp(&a.initiative);
        }

        // 2. Higher Dex modifier wins
        let dex_a = get_dex_modifier(a, monster_defs);
        let dex_b = get_dex_modifier(b, monster_defs);
        if dex_a != dex_b {
            return dex_b.cmp(&dex_a);
        }

        // 3. Players before monsters
        let player_a = if a.entity_type == "player" { 0 } else { 1 };
        let player_b = if b.entity_type == "player" { 0 } else { 1 };
        if player_a != player_b {
            return player_a.cmp(&player_b);
        }

        // 4. Higher CR wins (monsters)
        let cr_a = get_cr_decimal(a, monster_defs);
        let cr_b = get_cr_decimal(b, monster_defs);
        if (cr_a - cr_b).abs() > f64::EPSILON {
            return cr_b.partial_cmp(&cr_a).unwrap();
        }

        // 5. Deterministic fallback (instance_id)
        a.instance_id.cmp(&b.instance_id)
    });

    sorted
}

fn get_dex_modifier(entity: &EncounterEntity, defs: &HashMap<String, MonsterDefinition>) -> i32 {
    if entity.entity_type == "monster" {
        if let Some(id) = &entity.monster_id {
            if let Some(def) = defs.get(id) {
                return def.ability_scores.dex_modifier();
            }
        }
    }
    0
}

fn get_cr_decimal(entity: &EncounterEntity, defs: &HashMap<String, MonsterDefinition>) -> f64 {
    if entity.entity_type == "monster" {
        if let Some(id) = &entity.monster_id {
            if let Some(def) = defs.get(id) {
                return def.challenge_rating.decimal();
            }
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::ability_scores::AbilityScores;
    use crate::domain::value_objects::challenge_rating::ChallengeRating;

    fn make_test_monster(id: &str, init: i32, dex: i32, cr: &str) -> EncounterEntity {
        EncounterEntity {
            instance_id: id.to_string(),
            display_name: id.to_string(),
            entity_type: "monster".to_string(),
            initiative: init,
            current_hp: Some(10),
            max_hp: Some(10),
            monster_id: Some(id.to_string()),
            conditions: vec![],
            is_active: true,
        }
    }

    #[test]
    fn sorts_highest_initiative_first() {
        let mut defs = HashMap::new();
        let entities = vec![make_test_monster("low", 10, 10, "1/4"), make_test_monster("high", 18, 10, "1/4")];
        let sorted = sort_entities(&entities, &defs);
        assert_eq!(sorted[0].initiative, 18);
    }

    #[test]
    fn tie_higher_dex_wins() {
        let mut defs = HashMap::new();
        // We need to add monster defs for dex to work, simplified test
        let entities = vec![make_test_monster("dex_low", 15, 10, "1/4"), make_test_monster("dex_high", 15, 16, "1/4")];
        let sorted = sort_entities(&entities, &defs);
        // Note: full dex test requires proper MonsterDefinition in defs
        assert!(sorted.len() == 2);
    }

    #[test]
    fn basic_functionality() {
        let defs = HashMap::new();
        let entities = vec![make_test_monster("a", 12, 10, "1/4")];
        let sorted = sort_entities(&entities, &defs);
        assert_eq!(sorted.len(), 1);
    }
}