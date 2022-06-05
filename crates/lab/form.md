```rust
let count = 0usize.reactive();
input(attr! {count #> value=count.val()})
    .on(Blur, update!(count => |evt| {
}))
    .bind(count.bind(|data, ele| {
        ...
    }))
    .bind(sub!(count M> |ele| {
        ...
    }))

input! {
    #count {} => {}
}
```