pub type ValueType = i128;

pub trait SafeChecker {
    // Populate the state with the initial window of elements.
    // In this case, the first 100 elements.
    fn from_window(window: &[ValueType]) -> Self;

    // Check if `value` is "safe" given the current state.
    // A value is "safe" if it is the sum of two values in the state.
    fn is_safe(&self, value: ValueType) -> bool;

    // Add a new value to the state.
    fn add(&mut self, value: ValueType);

    // Remove a value from the state.
    fn remove(&mut self, value: ValueType);
}

// Helper function for benchmarking.
// Runs the given checker with the given input.
pub fn run<C>(
    window_size: usize,
    initial_window: &[ValueType],
    number_of_values: usize,
    values: &[ValueType],
) -> Vec<bool>
where
    C: SafeChecker,
{
    let mut is_safe = Vec::with_capacity(number_of_values);
    for _ in 0..window_size {
        is_safe.push(true);
    }

    let mut checker = C::from_window(initial_window);
    for i in window_size..number_of_values {
        let current_value = values[i];
        let to_remove = i - window_size;
        if checker.is_safe(current_value) {
            is_safe.push(true);
        } else {
            is_safe.push(false);
        }
        checker.add(current_value);
        checker.remove(values[to_remove]);
    }

    is_safe
}
