use crate::algorithms::recommendation_request::RecommendationRequest;
use crate::algorithms::recommendation_response::RecommendationResponse;
use rand::rngs::StdRng;
pub trait RecommendationAlgorithm<S: RecommendationRequest, T: RecommendationResponse> {
    /// Computes recommendations based on the given request and a source of randomness.
    ///
    /// # Arguments
    /// * `request` - The data and parameters for generating recommendations.
    /// * `random` - A random number generator for making random choices within the algorithm.
    ///
    /// # Returns
    /// Returns a populated recommendation response.
    fn compute_recommendations(&self, request: &S, random: &mut StdRng) -> T;
}
