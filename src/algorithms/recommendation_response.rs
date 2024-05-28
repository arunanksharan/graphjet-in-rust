use crate::algorithms::recommendation_info::RecommendationInfo;
// pub struct RecommendationResponse<T: RecommendationInfo> {
//     ranked_recommendations: Vec<T>,
// }

// impl<T: RecommendationInfo> RecommendationResponse<T> {
//     // Constructor function to create a new RecommendationResponse with the given recommendations.
//     pub fn new(ranked_recommendations: Vec<T>) -> Self {
//         RecommendationResponse {
//             ranked_recommendations,
//         }
//     }

//     // Method to access the ranked recommendations.
//     // Returns an immutable reference to the vector of ranked recommendations.
//     pub fn get_ranked_recommendations(&self) -> &Vec<T> {
//         &self.ranked_recommendations
//     }
// }
pub trait RecommendationResponse {
    // Explicitly tie the iterator's item lifetime to the lifetime of `self`.
    fn ranked_recommendations<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a dyn RecommendationInfo> + 'a>;
}

pub struct ConcreteRecommendationResponse {
    ranked_recommendations: Vec<Box<dyn RecommendationInfo>>,
}

impl RecommendationResponse for ConcreteRecommendationResponse {
    fn ranked_recommendations<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a dyn RecommendationInfo> + 'a> {
        Box::new(self.ranked_recommendations.iter().map(|r| r.as_ref()))
    }
}
