```rust
let count = 0usize.reactive();
input(attr! {count #> value=count.val()})
    .on(Blur, update!(count => |evt| {

}))

input! {
    #count {} => {}
}
```