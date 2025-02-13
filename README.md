# diesel-testing

Testing traits involving diesel

## The issue

Diesel provides many derive macros which, in the case of concrete structs, greatly simplify the implementation of traits such as `Insertable` - these derive generate many things, including for instance the types for the n-uples of the rows being inserted or extracted.

While this is great for concrete structs, when trying to define a trait `InsertableStruct` for a generic struct that is insertable such as:

```rust
pub trait InsertableStruct: Insertable {}
```

you will find that it actually is not enough for your struct to be actually insertable, but many other additional constraints have to be added.

Unfortunately, at this time, I could not figure out the whole set of constraints.
