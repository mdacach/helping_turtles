## Possible Solutions

### 1. Brute Force
Try out every possible sum of two distinct elements within the sliding window.

### 2. Hash Map
Iterate through the HashMap, and for each element, check if there is another element in the map such that both together
sum to the desired value.

### 3. Two Pointers
Use the two-pointers technique on a sorted vector to find if such sum exists.
Requires that the vector is kept sorted, which adds overhead.

### 4. Two Pointers with BTreeMap (not implemented)
It's possible to avoid having to sort the vector every time in approach 3. by using
a balanced tree.

## Benchmarks

Run `cargo bench` to see the running time of each solution with the inputs given.

Note that 100 is a very small number. It's very fast to process and sort such a small vector, so
the Two Pointers technique ended up being the fastest here.
The situation changes when we consider larger windows.