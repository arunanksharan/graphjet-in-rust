use std::collections::HashSet;

struct RecommendationRequest {
    query_node: u64,
    to_be_filtered: HashSet<u64>,
    social_proof_types: Vec<u8>,
}

impl RecommendationRequest {
    const CLICK_SOCIAL_PROOF_TYPE: u8 = 0;
    const FAVORITE_SOCIAL_PROOF_TYPE: u8 = 1;
    const RETWEET_SOCIAL_PROOF_TYPE: u8 = 2;
    const REPLY_SOCIAL_PROOF_TYPE: u8 = 3;
    const AUTHOR_SOCIAL_PROOF_TYPE: u8 = 4;
    const IS_MENTIONED_SOCIAL_PROOF_TYPE: u8 = 5;
    const IS_MEDIATAGGED_SOCIAL_PROOF_TYPE: u8 = 6;
    const QUOTE_SOCIAL_PROOF_TYPE: u8 = 7;
    const UNFAVORITE_SOCIAL_PROOF_TYPE: u8 = 8;
    const MAX_SOCIAL_PROOF_TYPE_SIZE: usize = 9;
    const DEFAULT_MIN_USER_SOCIAL_PROOF_SIZE: usize = 1;
    const DEFAULT_RECOMMENDATION_RESULTS: usize = 100;
    const MAX_EDGES_PER_NODE: usize = 500;
    const MAX_RECOMMENDATION_RESULTS: usize = 2500;

    fn new(
        query_node: u64,
        to_be_filtered: std::collections::HashSet<u64>,
        social_proof_types: Vec<u8>,
    ) -> Self {
        RecommendationRequest {
            query_node,
            to_be_filtered,
            social_proof_types,
        }
    }

    pub fn get_query_node(&self) -> u64 {
        self.query_node
    }

    pub fn get_to_be_filtered(&self) -> &std::collections::HashSet<u64> {
        &self.to_be_filtered
    }

    pub fn get_social_proof_types(&self) -> &[u8] {
        &self.social_proof_types
    }
}
