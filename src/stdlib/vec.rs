/// Standard library Vec<T> (growable array) type definition
/// A dynamic array that can grow and shrink at runtime

pub const VEC_DEFINITION: &str = r#"
// Vec<T> - A growable, heap-allocated array
// Provides dynamic sizing and automatic memory management
struct Vec<T> {
    data: [T],        // Internal array storage
    length: i32,      // Current number of elements
    capacity: i32,    // Total allocated capacity
}

impl<T> Vec<T> {
    // Create a new empty Vec
    fn new() -> Vec<T> {
        return Vec {
            data: [],
            length: 0,
            capacity: 0,
        };
    }

    // Create a Vec with a specific initial capacity
    fn with_capacity(capacity: i32) -> Vec<T> {
        return Vec {
            data: [],  // Would allocate array of size capacity
            length: 0,
            capacity: capacity,
        };
    }

    // Create a Vec from an array
    fn from_array(arr: [T]) -> Vec<T> {
        // Would copy array elements
        return Vec {
            data: arr,
            length: 0,  // Would be arr.len()
            capacity: 0,
        };
    }

    // Add an element to the end of the Vec
    fn push(self: &mut Vec<T>, value: T) {
        // If length == capacity, need to grow
        // In real implementation, would:
        // 1. Allocate new array with double capacity
        // 2. Copy existing elements
        // 3. Add new element
        // 4. Update length
        self.length = self.length + 1;
    }

    // Remove and return the last element
    fn pop(self: &mut Vec<T>) -> Option<T> {
        if self.length == 0 {
            return Option::None;
        } else {
            self.length = self.length - 1;
            // Would return the element at self.data[self.length]
            return Option::None;  // Placeholder
        }
    }

    // Get the number of elements in the Vec
    fn len(self: &Vec<T>) -> i32 {
        return self.length;
    }

    // Check if the Vec is empty
    fn is_empty(self: &Vec<T>) -> bool {
        return self.length == 0;
    }

    // Get the current capacity
    fn capacity(self: &Vec<T>) -> i32 {
        return self.capacity;
    }

    // Get an element at a specific index
    fn get(self: &Vec<T>, index: i32) -> Option<&T> {
        if index >= 0 && index < self.length {
            // Would return Some(&self.data[index])
            return Option::None;  // Placeholder
        } else {
            return Option::None;
        }
    }

    // Get a mutable reference to an element
    fn get_mut(self: &mut Vec<T>, index: i32) -> Option<&mut T> {
        if index >= 0 && index < self.length {
            // Would return Some(&mut self.data[index])
            return Option::None;  // Placeholder
        } else {
            return Option::None;
        }
    }

    // Insert an element at a specific index
    fn insert(self: &mut Vec<T>, index: i32, value: T) {
        // Would shift elements right and insert
        // For now, just increment length
        if index >= 0 && index <= self.length {
            self.length = self.length + 1;
        }
    }

    // Remove an element at a specific index
    fn remove(self: &mut Vec<T>, index: i32) -> Option<T> {
        if index >= 0 && index < self.length {
            // Would shift elements left and return removed element
            self.length = self.length - 1;
            return Option::None;  // Placeholder
        } else {
            return Option::None;
        }
    }

    // Clear all elements from the Vec
    fn clear(self: &mut Vec<T>) {
        self.length = 0;
        // Would also reset data array
    }

    // Reserve additional capacity
    fn reserve(self: &mut Vec<T>, additional: i32) {
        let new_capacity = self.length + additional;
        if new_capacity > self.capacity {
            // Would reallocate with new capacity
            self.capacity = new_capacity;
        }
    }

    // Shrink capacity to fit the current length
    fn shrink_to_fit(self: &mut Vec<T>) {
        self.capacity = self.length;
        // Would reallocate to exact size
    }

    // Truncate the Vec to a specific length
    fn truncate(self: &mut Vec<T>, new_len: i32) {
        if new_len < self.length {
            self.length = new_len;
            // Would drop elements beyond new_len
        }
    }

    // Swap two elements by index
    fn swap(self: &mut Vec<T>, i: i32, j: i32) {
        if i >= 0 && i < self.length && j >= 0 && j < self.length {
            // Would swap self.data[i] with self.data[j]
        }
    }

    // Reverse the elements in the Vec
    fn reverse(self: &mut Vec<T>) {
        let i = 0;
        let j = self.length - 1;
        // Would swap elements from both ends moving inward
    }

    // Check if the Vec contains a value
    fn contains(self: &Vec<T>, value: &T) -> bool {
        // Would iterate and compare each element
        return false;
    }

    // Find the first index of a value
    fn index_of(self: &Vec<T>, value: &T) -> Option<i32> {
        // Would iterate and return index if found
        return Option::None;
    }

    // Append another Vec to this one
    fn append(self: &mut Vec<T>, other: &mut Vec<T>) {
        // Would move all elements from other to self
        self.length = self.length + other.length;
        other.length = 0;
    }

    // Split the Vec at an index
    fn split_off(self: &mut Vec<T>, at: i32) -> Vec<T> {
        if at >= 0 && at <= self.length {
            let new_vec = Vec::new();
            // Would move elements from at..length to new_vec
            self.length = at;
            return new_vec;
        } else {
            return Vec::new();
        }
    }

    // Resize the Vec to a new length, filling with a value
    fn resize(self: &mut Vec<T>, new_len: i32, value: T) {
        if new_len > self.length {
            // Would fill with copies of value
        } else if new_len < self.length {
            // Would truncate
        }
        self.length = new_len;
    }

    // Deduplicate consecutive equal elements
    fn dedup(self: &mut Vec<T>) {
        // Would remove consecutive duplicates
        // Requires T to implement equality
    }

    // Retain only elements that match a predicate
    fn retain(self: &mut Vec<T>, predicate: fn(&T) -> bool) {
        // Would keep only elements where predicate returns true
        // Would update length accordingly
    }

    // Convert to an array (if possible)
    fn to_array(self: Vec<T>) -> [T] {
        return self.data;
    }

    // Get first element
    fn first(self: &Vec<T>) -> Option<&T> {
        if self.length > 0 {
            return self.get(0);
        } else {
            return Option::None;
        }
    }

    // Get last element
    fn last(self: &Vec<T>) -> Option<&T> {
        if self.length > 0 {
            return self.get(self.length - 1);
        } else {
            return Option::None;
        }
    }

    // Get first element (mutable)
    fn first_mut(self: &mut Vec<T>) -> Option<&mut T> {
        if self.length > 0 {
            return self.get_mut(0);
        } else {
            return Option::None;
        }
    }

    // Get last element (mutable)
    fn last_mut(self: &mut Vec<T>) -> Option<&mut T> {
        if self.length > 0 {
            return self.get_mut(self.length - 1);
        } else {
            return Option::None;
        }
    }

    // ITERATOR METHODS - Functional programming support

    // Map: Transform each element using a function
    fn map<U>(self: &Vec<T>, f: fn(&T) -> U) -> Vec<U> {
        let result = Vec::with_capacity(self.length);

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    result.push(f(value));
                },
                Option::None => {},
            }
        }

        return result;
    }

    // Filter: Keep only elements matching a predicate
    fn filter(self: &Vec<T>, predicate: fn(&T) -> bool) -> Vec<T> {
        let result = Vec::new();

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    if predicate(value) {
                        result.push(*value);
                    }
                },
                Option::None => {},
            }
        }

        return result;
    }

    // Find: Find first element matching predicate
    fn find(self: &Vec<T>, predicate: fn(&T) -> bool) -> Option<&T> {
        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    if predicate(value) {
                        return Option::Some(value);
                    }
                },
                Option::None => {},
            }
        }

        return Option::None;
    }

    // Filter-map: Combine filter and map
    fn filter_map<U>(self: &Vec<T>, f: fn(&T) -> Option<U>) -> Vec<U> {
        let result = Vec::new();

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    match f(value) {
                        Option::Some(mapped) => result.push(mapped),
                        Option::None => {},
                    }
                },
                Option::None => {},
            }
        }

        return result;
    }

    // Reduce/Fold: Combine elements into a single value
    fn reduce<U>(self: &Vec<T>, initial: U, f: fn(U, &T) -> U) -> U {
        let accumulator = initial;

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    accumulator = f(accumulator, value);
                },
                Option::None => {},
            }
        }

        return accumulator;
    }

    // For-each: Execute a function for each element
    fn for_each(self: &Vec<T>, f: fn(&T)) {
        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => f(value),
                Option::None => {},
            }
        }
    }

    // Any: Check if any element matches predicate
    fn any(self: &Vec<T>, predicate: fn(&T) -> bool) -> bool {
        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    if predicate(value) {
                        return true;
                    }
                },
                Option::None => {},
            }
        }

        return false;
    }

    // All: Check if all elements match predicate
    fn all(self: &Vec<T>, predicate: fn(&T) -> bool) -> bool {
        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    if !predicate(value) {
                        return false;
                    }
                },
                Option::None => {},
            }
        }

        return true;
    }

    // Take: Take first n elements
    fn take(self: &Vec<T>, n: i32) -> Vec<T> {
        let result = Vec::new();
        let count = if n < self.length { n } else { self.length };

        for i in 0..count {
            match self.get(i) {
                Option::Some(value) => result.push(*value),
                Option::None => {},
            }
        }

        return result;
    }

    // Skip: Skip first n elements
    fn skip(self: &Vec<T>, n: i32) -> Vec<T> {
        let result = Vec::new();

        for i in n..self.length {
            match self.get(i) {
                Option::Some(value) => result.push(*value),
                Option::None => {},
            }
        }

        return result;
    }

    // Chunk: Split Vec into chunks of size n
    fn chunks(self: &Vec<T>, chunk_size: i32) -> Vec<Vec<T>> {
        let result = Vec::new();
        let mut current_chunk = Vec::new();

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    current_chunk.push(*value);

                    if current_chunk.len() == chunk_size {
                        result.push(current_chunk);
                        current_chunk = Vec::new();
                    }
                },
                Option::None => {},
            }
        }

        // Add remaining elements as last chunk
        if !current_chunk.is_empty() {
            result.push(current_chunk);
        }

        return result;
    }

    // Flat-map: Map and flatten results
    fn flat_map<U>(self: &Vec<T>, f: fn(&T) -> Vec<U>) -> Vec<U> {
        let result = Vec::new();

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    let mapped = f(value);
                    for j in 0..mapped.len() {
                        match mapped.get(j) {
                            Option::Some(item) => result.push(*item),
                            Option::None => {},
                        }
                    }
                },
                Option::None => {},
            }
        }

        return result;
    }

    // Partition: Split into two Vecs based on predicate
    fn partition(self: &Vec<T>, predicate: fn(&T) -> bool) -> (Vec<T>, Vec<T>) {
        let true_vec = Vec::new();
        let false_vec = Vec::new();

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    if predicate(value) {
                        true_vec.push(*value);
                    } else {
                        false_vec.push(*value);
                    }
                },
                Option::None => {},
            }
        }

        return (true_vec, false_vec);
    }

    // Zip: Combine two Vecs into Vec of tuples
    fn zip<U>(self: &Vec<T>, other: &Vec<U>) -> Vec<(T, U)> {
        let result = Vec::new();
        let min_len = if self.length < other.length { self.length } else { other.length };

        for i in 0..min_len {
            match (self.get(i), other.get(i)) {
                (Option::Some(a), Option::Some(b)) => {
                    result.push((*a, *b));
                },
                _ => {},
            }
        }

        return result;
    }

    // Enumerate: Create Vec of (index, value) tuples
    fn enumerate(self: &Vec<T>) -> Vec<(i32, T)> {
        let result = Vec::new();

        for i in 0..self.length {
            match self.get(i) {
                Option::Some(value) => {
                    result.push((i, *value));
                },
                Option::None => {},
            }
        }

        return result;
    }
}

// Implement Iterator for Vec<T>
impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIterator<T>;

    fn into_iter(self: Vec<T>) -> VecIterator<T> {
        return VecIterator {
            vec: self,
            index: 0,
        };
    }
}

struct VecIterator<T> {
    vec: Vec<T>,
    index: i32,
}

impl<T> Iterator for VecIterator<T> {
    type Item = T;

    fn next(self: &mut VecIterator<T>) -> Option<T> {
        if self.index < self.vec.length {
            let value = self.vec.get(self.index);
            self.index = self.index + 1;
            // Would return the actual value
            return Option::None;  // Placeholder
        } else {
            return Option::None;
        }
    }
}

// Implement indexing for Vec<T>
// vec[i] would desugar to vec.get(i).unwrap()
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_definition_exists() {
        assert!(!VEC_DEFINITION.is_empty());
    }

    #[test]
    fn test_vec_definition_contains_struct() {
        assert!(VEC_DEFINITION.contains("struct Vec<T>"));
    }

    #[test]
    fn test_vec_definition_contains_methods() {
        assert!(VEC_DEFINITION.contains("fn new()"));
        assert!(VEC_DEFINITION.contains("fn push("));
        assert!(VEC_DEFINITION.contains("fn pop("));
        assert!(VEC_DEFINITION.contains("fn len("));
        assert!(VEC_DEFINITION.contains("fn is_empty("));
    }

    #[test]
    fn test_vec_definition_contains_iterator() {
        assert!(VEC_DEFINITION.contains("impl<T> IntoIterator for Vec<T>"));
        assert!(VEC_DEFINITION.contains("impl<T> Iterator for VecIterator<T>"));
    }
}
