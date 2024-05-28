use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pair {
    key: u64,
    metadata: u64,
}

impl Pair {
    pub fn new(key: u64, metadata: u64) -> Self {
        Pair { key, metadata }
    }

    pub fn key(&self) -> u64 {
        self.key
    }

    pub fn metadata(&self) -> u64 {
        self.metadata
    }
}

pub struct SmallArrayBasedLongToDoubleMap {
    add_keys_to_set_threshold: usize,
    keys: Vec<u64>,
    values: Vec<f64>,
    metadata_array: Vec<u64>,
    capacity: usize,
    size: usize,
    unique_keys_size: usize,
    key_set: Option<HashSet<u64>>,
    key_metadata_pair_set: Option<HashSet<Pair>>,
}

impl SmallArrayBasedLongToDoubleMap {
    pub fn new() -> Self {
        Self {
            add_keys_to_set_threshold: 8, // As previously set in the struct definition
            keys: Vec::with_capacity(4),
            values: Vec::with_capacity(4),
            metadata_array: Vec::with_capacity(4),
            capacity: 4,
            size: 0,
            unique_keys_size: 0,
            key_set: None,
            key_metadata_pair_set: None,
        }
    }

    // Getters for keys, values, and metadata
    pub fn keys(&self) -> &[u64] {
        &self.keys
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn metadata(&self) -> &[u64] {
        &self.metadata_array
    }

    // Getters for sizes
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn unique_keys_size(&self) -> usize {
        self.unique_keys_size
    }

    pub fn put(&mut self, key: u64, value: f64, metadata: u64) -> bool {
        let mut is_unique_key = true;

        if self.size < self.add_keys_to_set_threshold {
            for i in 0..self.size {
                if self.keys[i] == key {
                    is_unique_key = false;
                    if self.metadata_array[i] == metadata {
                        return false;
                    }
                }
            }
        } else {
            let pair = Pair { key, metadata };
            let key_set = self.key_set.get_or_insert_with(HashSet::new);
            let pair_set = self.key_metadata_pair_set.get_or_insert_with(HashSet::new);
            if !pair_set.insert(pair) {
                return false;
            }
            is_unique_key = key_set.insert(key);
        }
        if self.size == self.capacity {
            self.capacity *= 2;
            self.keys.resize(self.capacity, 0);
            self.values.resize(self.capacity, 0.0);
            self.metadata_array.resize(self.capacity, 0);
        }
        if is_unique_key {
            self.unique_keys_size += 1;
        }
        self.keys.push(key);
        self.values.push(value);
        self.metadata_array.push(metadata);
        self.size += 1;
        true
    }

    pub fn sort(&mut self) {
        let mut indices: Vec<usize> = (0..self.size).collect();
        indices.sort_by(|&i1, &i2| {
            let val1 = self.values[i1];
            let val2 = self.values[i2];
            // Order by descending
            val2.partial_cmp(&val1).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Reorder keys, values, and metadata according to sorted indices
        self.apply_sort_order(&indices);
    }

    fn apply_sort_order(&mut self, indices: &[usize]) {
        // Create new vectors by applying the sorted order
        let sorted_keys = indices.iter().map(|&i| self.keys[i]).collect::<Vec<_>>();
        let sorted_values = indices.iter().map(|&i| self.values[i]).collect::<Vec<_>>();
        let sorted_metadata = indices
            .iter()
            .map(|&i| self.metadata_array[i])
            .collect::<Vec<_>>();

        // Replace the old vectors with the new sorted vectors
        self.keys = sorted_keys;
        self.values = sorted_values;
        self.metadata_array = sorted_metadata;
    }

    pub fn trim(&mut self, input_capacity: usize) -> bool {
        let new_capacity = std::cmp::min(input_capacity, self.size);

        if new_capacity < self.capacity {
            // Reduce the capacity of the vectors
            self.keys.truncate(new_capacity);
            self.values.truncate(new_capacity);
            self.metadata_array.truncate(new_capacity);

            // Adjust size and uniqueKeysSize
            self.size = new_capacity;
            self.unique_keys_size = std::cmp::min(new_capacity, self.unique_keys_size);
            self.capacity = new_capacity; // Note: this line is actually unnecessary in Rust

            true // Indicate that the capacity was successfully trimmed
        } else {
            false // No trimming was needed
        }
    }

    pub fn contains(&self, key: u64) -> bool {
        if self.size <= self.add_keys_to_set_threshold {
            // Linear search through the keys
            self.keys.iter().any(|&k| k == key)
        } else {
            // Use the HashSet for quick lookup
            self.key_set
                .as_ref()
                .map_or(false, |set| set.contains(&key))
        }
    }

    /// Todo: Update put and trim method to use copy method
    fn copy(&mut self, new_length: usize) {
        // Truncate or extend the vectors to match the new length
        if new_length < self.size {
            self.keys.truncate(new_length);
            self.values.truncate(new_length);
            self.metadata_array.truncate(new_length);
        } else {
            self.keys.resize(new_length, 0);
            self.values.resize(new_length, 0.0);
            self.metadata_array.resize(new_length, 0);
        }

        // Update the size if it's greater than the new length
        self.size = std::cmp::min(self.size, new_length);
    }
}
