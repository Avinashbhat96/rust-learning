# To initialize the project
cargo new <name | repo> 
# To run the project
cargo run -q

# Ownership 

- Each value is owned by a single variable, struct, vec, etc... at a time
- Reassigning the value to other variable, moves the value from old variable to the new variable. This makes the old variable to contain "no values"

# References
Using a reference by '&' gives us a read only references (immutable). 
- We cant move a value while references to the value exists, as we loose the source.
- we can also pass '&mut' so that the function can change the values in the called function
- if we have read only reference active, then we cant create a mutable reference. only one mutable reference at a time
- Rust minimizes unwanted changes to data


# Copy-able values
Some types of values like numbers booleans etc are going to appear to break the rules of ownership
- all numbers - i32, u32, f32
- bool
- char
- Arrays
- References
- Tuples

