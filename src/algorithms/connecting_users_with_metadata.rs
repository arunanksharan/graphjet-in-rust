// use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

// Longlist assumed to be analogous to Vec<i64> for storing long integers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectingUsersWithMetadata {
    pub connecting_users: Vec<i64>,
    pub metadata: Vec<i64>,
}

impl ConnectingUsersWithMetadata {
    // Constructor
    pub fn new(users: Vec<i64>, metadata: Vec<i64>) -> Self {
        ConnectingUsersWithMetadata {
            connecting_users: users,
            metadata,
        }
    }

    // Method to get connecting users
    pub fn get_connecting_users(&self) -> &Vec<i64> {
        &self.connecting_users
    }

    // Method to get metadata
    pub fn get_metadata(&self) -> &Vec<i64> {
        &self.metadata
    }
}

// Implementing Display trait for print capability
impl fmt::Display for ConnectingUsersWithMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "connectingUsers = {:?}, metadata = {:?}",
            self.connecting_users, self.metadata
        )
    }
}

// Implemeting Hash trait for custom hashing
// impl Hash for ConnectingUsersWithMetadata {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.connecting_users.hash(state);
//         self.metadata.hash(state);
//     }
// }

// &self - points to instance | &Self points to type
// PartialEq -> eq => implicitly works with same type
// No concept of null in Rust

// This is exactly what the derive macro does - no need for manual implementation
// impl PartialEq for ConnectingUsersWithMetadata {
//     fn eq(&self, other: &Self) -> bool {
//         self.connecting_users == other.connecting_users && self.metadata == other.metadata
//     }
// }

// impl Eq for ConnectingUsersWithMetadata {}
