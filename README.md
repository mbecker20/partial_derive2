# partial_derive

Like `Partial<T>` of `TypeScript`, makes all the properties of a struct type an optional property.

Provides `Partial` derive macro.

```rust
#[derive(Partial)]
#[derive(Clone)]
struct SomeStruct {
    pub field_one: i32,
    field_two: Vec<bool>,
}
```

generates

```rust
#[derive(Clone)]
struct PartialSomeStruct {
    pub field_one: Option<i32>,
    field_two: Option<Vec<bool>>,
}
```
