use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecommendationStats {
    num_direct_neighbors: i32,
    num_right_nodes_reached: i32,
    num_rhs_visits: i32,
    min_visits_per_right_node: i32,
    max_visits_per_right_node: i32,
    num_right_nodes_filtered: i32,
}

impl RecommendationStats {
    pub fn new() -> Self {
        Self {
            num_direct_neighbors: 0,
            num_right_nodes_reached: 0,
            num_rhs_visits: 0,
            min_visits_per_right_node: i32::MAX,
            max_visits_per_right_node: 0,
            num_right_nodes_filtered: 0,
        }
    }

    pub fn with_values(
        num_direct_neighbors: i32,
        num_right_nodes_reached: i32,
        num_rhs_visits: i32,
        min_visits_per_right_node: i32,
        max_visits_per_right_node: i32,
        num_right_nodes_filtered: i32,
    ) -> Self {
        Self {
            num_direct_neighbors,
            num_right_nodes_reached,
            num_rhs_visits,
            min_visits_per_right_node,
            max_visits_per_right_node,
            num_right_nodes_filtered,
        }
    }

    // Getters
    pub fn num_direct_neighbors(&self) -> i32 {
        self.num_direct_neighbors
    }

    pub fn num_right_nodes_reached(&self) -> i32 {
        self.num_right_nodes_reached
    }

    pub fn num_rhs_visits(&self) -> i32 {
        self.num_rhs_visits
    }

    pub fn min_visits_per_right_node(&self) -> i32 {
        self.min_visits_per_right_node
    }

    pub fn max_visits_per_right_node(&self) -> i32 {
        self.max_visits_per_right_node
    }

    pub fn num_right_nodes_filtered(&self) -> i32 {
        self.num_right_nodes_filtered
    }

    // Setters
    pub fn set_num_direct_neighbors(&mut self, val: i32) {
        self.num_direct_neighbors = val;
    }

    pub fn set_num_right_nodes_reached(&mut self, val: i32) {
        self.num_right_nodes_reached = val;
    }

    pub fn set_num_rhs_visits(&mut self, val: i32) {
        self.num_rhs_visits = val;
    }

    pub fn set_min_visits_per_right_node(&mut self, val: i32) {
        self.min_visits_per_right_node = val;
    }

    pub fn set_max_visits_per_right_node(&mut self, val: i32) {
        self.max_visits_per_right_node = val;
    }

    pub fn set_num_right_nodes_filtered(&mut self, val: i32) {
        self.num_right_nodes_filtered = val;
    }

    // Update methods
    pub fn update_min_visits_per_right_node(&mut self, visits: i32) {
        self.min_visits_per_right_node = i32::min(self.min_visits_per_right_node, visits);
    }

    pub fn update_max_visits_per_right_node(&mut self, visits: i32) {
        self.max_visits_per_right_node = i32::max(self.max_visits_per_right_node, visits);
    }

    pub fn add_to_num_rhs_visits(&mut self, visits: i32) {
        self.num_rhs_visits += visits;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

// impl Default for RecommendationStats {
//     fn default() -> Self {
//         Self {
//             num_direct_neighbors: 0,
//             num_right_nodes_reached: 0,
//             num_rhs_visits: 0,
//             min_visits_per_right_node: i32::MAX, // Assuming you want the default to be the maximum possible integer to simulate "no visits"
//             max_visits_per_right_node: 0,
//             num_right_nodes_filtered: 0,
//         }
//     }
// }

impl fmt::Display for RecommendationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RecommendationStats {{ numDirectNeighbors: {}, numRightNodesReached: {}, numRHSVisits: {}, minVisitsPerRightNode: {}, maxVisitsPerRightNode: {}, numRightNodesFiltered: {} }}",
               self.num_direct_neighbors,
               self.num_right_nodes_reached,
               self.num_rhs_visits,
               self.min_visits_per_right_node,
               self.max_visits_per_right_node,
               self.num_right_nodes_filtered)
    }
}
