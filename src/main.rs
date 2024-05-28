mod algorithms;
use crate::algorithms::ConnectingUsersWithMetadata; // Import the ConnectingUsersWithMetadata struct from the current crate

fn main() {
    println!("Hello, world!");
    let users = vec![1, 2, 3]; // Example User Ids
    let metadata = vec![1625198000, 1625199000, 1625200000]; // Example metadata (e.g., timestamps)
    let data = ConnectingUsersWithMetadata::new(users, metadata);

    println!("{}", data);
    println!("Users: {:?}", data.get_connecting_users());
    println!("Metadata: {:?}", data.get_metadata());
}
