## 组件内数据共享

```rust
pub fn count(init: usize) -> Div {
    let card = div(()).inject(init);
    let mark = card.mark();
    card.content((
        p(mark.view(|data| data)),
        button("incr").link(mark.clone()).onClick(Click, |(_, num)| {
            num.data += 1;
            num.update();
        }),
        button("dec").link(mark.clone()).onClick(Click, |(_, num)| {
            if num.data >= 1 {
                num.data -= 1;
                num.update();
            }
        }),
    ))
}
```

## 复杂数据交互


```rust

```