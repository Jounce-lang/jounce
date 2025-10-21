/// Standard library HashSet<T> type definition
/// A hash set for fast membership testing

pub const HASHSET_DEFINITION: &str = r#"
// HashSet<T> - A hash-based set for unique values
// Provides fast O(1) membership testing using hashing
struct HashSet<T> {
    map: HashMap<T, bool>,  // Internal implementation using HashMap
}

impl<T> HashSet<T> {
    // Create a new empty HashSet
    fn new() -> HashSet<T> {
        return HashSet::with_capacity(16);
    }

    // Create a HashSet with a specific capacity
    fn with_capacity(capacity: i32) -> HashSet<T> {
        return HashSet {
            map: HashMap::with_capacity(capacity),
        };
    }

    // Insert a value into the set
    // Returns true if the value was newly inserted
    fn insert(self: &mut HashSet<T>, value: T) -> bool {
        match self.map.insert(value, true) {
            Option::None => true,      // New insertion
            Option::Some(_) => false,  // Already existed
        }
    }

    // Remove a value from the set
    // Returns true if the value was present
    fn remove(self: &mut HashSet<T>, value: &T) -> bool {
        match self.map.remove(value) {
            Option::Some(_) => true,
            Option::None => false,
        }
    }

    // Check if a value exists in the set
    fn contains(self: &HashSet<T>, value: &T) -> bool {
        return self.map.contains_key(value);
    }

    // Get the number of elements
    fn len(self: &HashSet<T>) -> i32 {
        return self.map.len();
    }

    // Check if the set is empty
    fn is_empty(self: &HashSet<T>) -> bool {
        return self.map.is_empty();
    }

    // Clear all elements
    fn clear(self: &mut HashSet<T>) {
        self.map.clear();
    }

    // Reserve capacity for additional elements
    fn reserve(self: &mut HashSet<T>, additional: i32) {
        self.map.reserve(additional);
    }

    // Shrink capacity to fit current size
    fn shrink_to_fit(self: &mut HashSet<T>) {
        self.map.shrink_to_fit();
    }

    // Retain only elements matching predicate
    fn retain(self: &mut HashSet<T>, predicate: fn(&T) -> bool) {
        let to_remove = Vec::new();

        for value in self.iter() {
            if !predicate(&value) {
                to_remove.push(value);
            }
        }

        for value in to_remove {
            self.remove(&value);
        }
    }

    // Get iterator over values
    fn iter(self: &HashSet<T>) -> HashSetIterator<T> {
        return HashSetIterator {
            keys: self.map.keys(),
            index: 0,
        };
    }

    // Union: elements in either set
    fn union(self: &HashSet<T>, other: &HashSet<T>) -> HashSet<T> {
        let result = HashSet::new();

        for value in self.iter() {
            result.insert(value);
        }

        for value in other.iter() {
            result.insert(value);
        }

        return result;
    }

    // Intersection: elements in both sets
    fn intersection(self: &HashSet<T>, other: &HashSet<T>) -> HashSet<T> {
        let result = HashSet::new();

        for value in self.iter() {
            if other.contains(&value) {
                result.insert(value);
            }
        }

        return result;
    }

    // Difference: elements in self but not in other
    fn difference(self: &HashSet<T>, other: &HashSet<T>) -> HashSet<T> {
        let result = HashSet::new();

        for value in self.iter() {
            if !other.contains(&value) {
                result.insert(value);
            }
        }

        return result;
    }

    // Symmetric difference: elements in either set but not both
    fn symmetric_difference(self: &HashSet<T>, other: &HashSet<T>) -> HashSet<T> {
        let result = HashSet::new();

        for value in self.iter() {
            if !other.contains(&value) {
                result.insert(value);
            }
        }

        for value in other.iter() {
            if !self.contains(&value) {
                result.insert(value);
            }
        }

        return result;
    }

    // Check if this set is a subset of another
    fn is_subset(self: &HashSet<T>, other: &HashSet<T>) -> bool {
        if self.len() > other.len() {
            return false;
        }

        for value in self.iter() {
            if !other.contains(&value) {
                return false;
            }
        }

        return true;
    }

    // Check if this set is a superset of another
    fn is_superset(self: &HashSet<T>, other: &HashSet<T>) -> bool {
        return other.is_subset(self);
    }

    // Check if sets are disjoint (no common elements)
    fn is_disjoint(self: &HashSet<T>, other: &HashSet<T>) -> bool {
        for value in self.iter() {
            if other.contains(&value) {
                return false;
            }
        }

        return true;
    }

    // Convert to Vec
    fn to_vec(self: &HashSet<T>) -> Vec<T> {
        return self.map.keys();
    }
}

// Iterator for HashSet
struct HashSetIterator<T> {
    keys: Vec<T>,
    index: i32,
}

impl<T> Iterator for HashSetIterator<T> {
    type Item = T;

    fn next(self: &mut HashSetIterator<T>) -> Option<T> {
        if self.index < self.keys.len() {
            let value = self.keys[self.index];
            self.index = self.index + 1;
            return Option::Some(value);
        } else {
            return Option::None;
        }
    }
}

// Implement iteration over HashSet
impl<T> IntoIterator for HashSet<T> {
    type Item = T;
    type IntoIter = HashSetIterator<T>;

    fn into_iter(self: HashSet<T>) -> HashSetIterator<T> {
        return HashSetIterator {
            keys: self.map.keys(),
            index: 0,
        };
    }
}

// Helper functions

// Create HashSet from array
fn from_array<T>(arr: &[T]) -> HashSet<T> {
    let set = HashSet::new();

    for value in arr {
        set.insert(value);
    }

    return set;
}

// Create HashSet from Vec
fn from_vec<T>(vec: &Vec<T>) -> HashSet<T> {
    let set = HashSet::new();

    for i in 0..vec.len() {
        set.insert(vec[i]);
    }

    return set;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashset_definition_exists() {
        assert!(!HASHSET_DEFINITION.is_empty());
    }

    #[test]
    fn test_hashset_definition_contains_struct() {
        assert!(HASHSET_DEFINITION.contains("struct HashSet<T>"));
    }

    #[test]
    fn test_hashset_definition_contains_methods() {
        assert!(HASHSET_DEFINITION.contains("fn new()"));
        assert!(HASHSET_DEFINITION.contains("fn insert("));
        assert!(HASHSET_DEFINITION.contains("fn remove("));
        assert!(HASHSET_DEFINITION.contains("fn contains("));
        assert!(HASHSET_DEFINITION.contains("fn len("));
    }

    #[test]
    fn test_hashset_definition_contains_set_operations() {
        assert!(HASHSET_DEFINITION.contains("fn union("));
        assert!(HASHSET_DEFINITION.contains("fn intersection("));
        assert!(HASHSET_DEFINITION.contains("fn difference("));
        assert!(HASHSET_DEFINITION.contains("fn symmetric_difference("));
    }

    #[test]
    fn test_hashset_definition_contains_set_predicates() {
        assert!(HASHSET_DEFINITION.contains("fn is_subset("));
        assert!(HASHSET_DEFINITION.contains("fn is_superset("));
        assert!(HASHSET_DEFINITION.contains("fn is_disjoint("));
    }

    #[test]
    fn test_hashset_definition_contains_iterator() {
        assert!(HASHSET_DEFINITION.contains("impl<T> IntoIterator for HashSet<T>"));
        assert!(HASHSET_DEFINITION.contains("struct HashSetIterator"));
    }
}
