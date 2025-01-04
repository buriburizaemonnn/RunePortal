use std::collections::HashSet;

pub struct User {
    pub launch_created: HashSet<u128>,
    pub launch_participated: HashSet<u128>,
}
