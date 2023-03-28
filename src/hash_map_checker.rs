use crate::safe_checker::{SafeChecker, ValueType};
use std::collections::HashMap;

pub struct HashMapChecker {
    current_values: HashMap<ValueType, usize>,
}

impl SafeChecker for HashMapChecker {
    fn from_window(window: &[ValueType]) -> Self {
        let mut values = HashMap::new();
        for &v in window {
            let current = values.entry(v).or_insert(0);
            *current += 1;
        }

        Self {
            current_values: values,
        }
    }

    fn is_safe(&self, value: ValueType) -> bool {
        for present_value in self.current_values.keys() {
            // Note that this implementation handles negative values too.
            let desired_value = value - present_value;

            // We must be careful to not count the same value twice in the sum.
            // e.g. if we have 4 in the map, 8 is NOT safe, as we would need to use the same 4 twice.
            if desired_value == *present_value {
                // Unwrap is OK because we know present_value is in the map (we are iterating through
                // map keys).
                let present_count = self.current_values.get(present_value).unwrap();
                // If we have more than 2 entries of that value, we can use those for the sum.
                if *present_count >= 2 {
                    return true;
                } else {
                    // Otherwise, we would be using the same value twice, which is not allowed.
                    // We can still potentially make the sum with other values in the map, so we
                    // continue the iteration.
                    continue;
                }
            }

            match self.current_values.get(&desired_value) {
                None => {
                    continue;
                }
                Some(_) => {
                    return true;
                }
            }
        }

        // We tried each value in the map, and none worked.
        false
    }

    fn add(&mut self, value: ValueType) {
        *self.current_values.entry(value).or_insert(0) += 1;
    }

    fn remove(&mut self, value: ValueType) {
        match self.current_values.get_mut(&value) {
            None => {
                panic!("You tried to remove a non-existent entry in the map. This should not happen if you follow the sliding window correctly.")
            }
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    self.current_values.remove(&value);
                }
            }
        }
    }
}
