use crate::algorithms::recommendation_type::RecommendationType; // Import the missing type

pub trait RecommendationInfo {
    fn get_recommendation_type(&self) -> RecommendationType;
    fn get_weight(&self) -> f64;
}
