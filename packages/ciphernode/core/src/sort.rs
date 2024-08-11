use sha2::{Digest, Sha256};
use std::cmp::Ordering;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn is_node_selected(node_id: u64, seed: &str, total_nodes: u64, num_select: u64) -> bool {
    if num_select >= total_nodes {
        return true;
    }

    // Generate a unique hash for each node
    let input_string = format!("{}:{}", seed, node_id);
    let mut hasher = Sha256::new();
    hasher.update(input_string.as_bytes());
    let hash_result = hasher.finalize();

    // Convert the first 8 bytes of the hash to a u64
    let hash_int = u64::from_be_bytes(hash_result[..8].try_into().unwrap());

    // Calculate the node's rank (1-based)
    let node_rank = (hash_int % total_nodes) + 1;

    // Node is selected if its rank is less than or equal to num_select
    node_rank <= num_select
}

#[cfg(test)]
mod tests {
    use crate::sort::is_node_selected;

    #[test]
    fn test_hashing() {
        let total_nodes = 1000;
        let seed = "myseed123";
        let num_select = 100;

        // Check if node 42 is selected
        let node_id = 42;
        let is_selected = is_node_selected(node_id, seed, total_nodes, num_select);
        println!(
            "Node {} is {}",
            node_id,
            if is_selected {
                "selected"
            } else {
                "not selected"
            }
        );

        // Test to verify exact number of selections
        let selected_count = (1..=total_nodes)
            .filter(|&i| is_node_selected(i, seed, total_nodes, num_select))
            .count();
        println!("Number of selected nodes: {}", selected_count);
        assert_eq!(
            selected_count as u64, num_select,
            "Selection count doesn't match num_select"
        );
    }
}
