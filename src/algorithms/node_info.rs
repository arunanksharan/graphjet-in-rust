struct NodeInfo {
    node_id: u64,
    node_metadata: Vec<Vec<i32>>,
    weight: f64,
    num_visits: usize,
    social_proofs: Vec<SmallArrayBasedLongToDoubleMap>,
}

impl NodeInfo {
    // Constructors
    pub fn new(
        node_id: u64,
        node_metadata: Vec<Vec<i32>>,
        weight: f64,
        max_social_proof_type_size: usize,
    ) -> Self {
        Self {
            node_id,
            node_metadata,
            weight,
            num_visits: 1,
            social_proofs: vec![SmallArrayBasedLongToDoubleMap::new(); max_social_proof_type_size],
        }
    }

    pub fn new_with_default_metadata(
        node_id: u64,
        weight: f64,
        max_social_proof_type_size: usize,
    ) -> Self {
        Self {
            node_id,
            node_metadata: vec![vec![0; 1]; 1], // equivalent to EMPTY_NODE_META_DATA
            weight,
            num_visits: 1,
            social_proofs: vec![SmallArrayBasedLongToDoubleMap::new(); max_social_proof_type_size],
        }
    }

    // Getters and setters
    pub fn get_node_id(&self) -> u64 {
        self.node_id
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn add_to_weight(&mut self, increment: f64) {
        self.weight += increment;
        self.num_visits += 1;
    }

    // Social proof methods
    pub fn get_social_proofs(&self) -> &Vec<SmallArrayBasedLongToDoubleMap> {
        &self.social_proofs
    }

    pub fn add_to_social_proof(
        &mut self,
        social_proof_id: u64,
        edge_type: usize,
        edge_metadata: u64,
        social_proof_weight: f64,
    ) -> bool {
        if let Some(proof) = self.social_proofs.get_mut(edge_type) {
            proof.put(social_proof_id, social_proof_weight, edge_metadata);
            true
        } else {
            false
        }
    }

    // Metadata retrieval
    pub fn get_node_metadata(&self, node_metadata_type: usize) -> Option<&[i32]> {
        self.node_metadata
            .get(node_metadata_type)
            .map(|array| array.as_slice())
    }
}

impl PartialEq for NodeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id && self.weight == other.weight
    }
}

impl Eq for NodeInfo {}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeInfo {{ node_id: {}, weight: {}, social_proofs: {:?} }}",
            self.node_id, self.weight, self.social_proofs
        )
    }
}
