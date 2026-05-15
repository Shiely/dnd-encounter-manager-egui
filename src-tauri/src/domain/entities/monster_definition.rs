// DOMAIN LAYER: stdlib imports only.
use crate::domain::value_objects::{ability_scores::AbilityScores, challenge_rating::ChallengeRating};

#[derive(Debug, Clone)]
pub struct MonsterDefinition {
    pub id: String,
    pub name: String,
    pub ability_scores: AbilityScores,
    pub challenge_rating: ChallengeRating,
}