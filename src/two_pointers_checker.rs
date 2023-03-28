use crate::safe_checker::{SafeChecker, ValueType};

// INVARIANT: current_values must always be **sorted**.
// (We sort it after each operation).
pub struct TwoPointersChecker {
    current_values: Vec<ValueType>,
}

impl SafeChecker for TwoPointersChecker {
    fn from_window(window: &[ValueType]) -> Self {
        let mut values = Vec::new();
        for &v in window {
            values.push(v);
        }
        values.sort_unstable();
        Self {
            current_values: values,
        }
    }

    fn is_safe(&self, value: ValueType) -> bool {
        let mut left = 0_usize;
        let mut right = self.current_values.len() - 1;

        // Standard two pointers technique.
        // Refer to: https://www.geeksforgeeks.org/two-pointers-technique/
        while left < right {
            let this_sum = self.current_values[left] + self.current_values[right];
            if this_sum == value {
                return true;
            }

            if this_sum > value {
                right -= 1; // As the array is sorted, this decreases the sum.
            } else {
                left += 1;
            }
        }

        false
    }

    fn add(&mut self, value: ValueType) {
        self.current_values.push(value);
        self.current_values.sort_unstable(); // Faster.
    }

    fn remove(&mut self, value: ValueType) {
        let pos = self
            .current_values
            .iter()
            .position(|&x| x == value)
            .unwrap();
        self.current_values.swap_remove(pos);
        self.current_values.sort_unstable();
    }
}
