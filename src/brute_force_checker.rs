use crate::safe_checker::{SafeChecker, ValueType};
use std::collections::VecDeque;

pub struct BruteForceChecker {
    current_values: VecDeque<ValueType>,
}

impl SafeChecker for BruteForceChecker {
    fn from_window(window: &[ValueType]) -> Self {
        let mut values = VecDeque::new();
        for &v in window {
            values.push_back(v);
        }
        Self {
            current_values: values,
        }
    }

    fn is_safe(&self, value: ValueType) -> bool {
        // Check every possible sum.
        for (i1, v1) in self.current_values.iter().enumerate() {
            for (i2, v2) in self.current_values.iter().enumerate() {
                // Do not count the same value twice.
                if i1 == i2 {
                    continue;
                }

                // Found it, all good.
                if v1 + v2 == value {
                    return true;
                }
            }
        }

        false
    }

    fn add(&mut self, value: ValueType) {
        self.current_values.push_back(value);
    }

    fn remove(&mut self, value: ValueType) {
        // For this particular implementation we already know the value we should remove.
        assert_eq!(value, *self.current_values.front().unwrap());
        self.current_values.pop_front();
    }
}
