```rust
let count = 0usize.reactive();

count.before_add_assign(|current, delta| {
    tracing::info!("going to add + {delta}")
    current
})

count.view()
```