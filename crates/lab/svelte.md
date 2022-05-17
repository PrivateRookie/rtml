## hello world

```rust
h1("hello world")
```

## adding data

```rust
let name = "world";

h1(format!("hello, {name}"))
```

## attr

```rust
let src = "/nice.gif"

img(attr! { src=src })

// short hand
img(attr! { src })
```

## style

```rust
p((
    style! {
        color: purple;
        font-family: r#""Comic Sans MS", cursive"#;
        font-size: 2em;
    },
    "this is a paragraph"
))
```

```rust
body((
    p("this is a paragraph"),
    style(
        css! {
            p {
                color: purple;
                font-family: 'Comic Sans MS', cursive;
                font-size: 2em;
            }
        } 
    )
))
```

## nested component

```rust
fn nested() -> P {
    p("this is a paragraph")
}

div((
    nested(),
    p("this is a other paragraph")
))
```

## update

```rust
let count = reactive(0usize);

let btn_label = count.view(|count| {
    let s = if count == 1 { "" } else { "s" };
    format!("Clicked {count} time{s}")
});

button(btn_label).on(Click, move |_| {
    count += 1;
})
button(btn_label).on(Click, count.rel(|&mut data| {
    data += 1;
}))

button(())
    .view(move || {
        let s = if count == 1 { "" } else { "s" };
        format!("Clicked {count} time{s}")
    })
    .on(Click, move || {
        count += 1;
    })
```

## dep reactive

```rust
let count = reactive(0usize);

let double = move || count * 2;

div((
    p(format!("{count} doubled is {double}")),
    button("+1").on(Click, move|count| {
        count += 1
    })
))
```

## subscribe change

```rust
let count = reactive(0usize);

count.before_change(|data| {
    if data >= 10 {
        alert("count is dangerously hight!");
        9 
    } else {
       count
    }
})
```

## struct or vec

```rust
let numbers = reactive![1, 2, 3, 4];

let sum = move || numbers.iter().sum();
let display = move || {
    let arr = numbers.join(" + ");
    format!("{arr} = {sum}")
}

div((
    p(display),
    button("add a number")
        .on(Click, move || {
            numbers += numbers.len() + 1;
        })
))
```

## exported

```rust
// nested
pub fn nested(answer: View<usize>) -> P {
    p("the answer is {answer}")
}

// outer
let answer = 42.reactive();
div(nested(move || answer))
```

## default value

```rust
pub fn nested<V: Option<View<usize>>>(answer: V) -> P {
    if let Some(view) = answer {
        p("the answer is {view}")
    } else {
        p("the answer is a mystery")
    }
}

// outer
let answer = 42.reactive();
div(nested(move || answer))
```

## logical control

```rust
struct User {
    logged_in: bool
}

let user = reactive(User { logged_in: false });
let toggle = move || user.logged_in != user.logged_in;

let btn = move || {
    if user.logged_in {
        button("logout").on(Click, move || toggle())
    } else {
        button("login").on(Click, move || toggle())
    }
}
```

## loop


```rust
struct Cats {
    id: String,
    name: String,
}

let cats = reactive(vec![
    Cat { id: "J---aiyznGQ", name: "Keyboard Cat" },
    Cat { id: "z_AbfPXTKms", name: "Maru" },
    Cat { id: "OUtn3pvWmpg", name: "Henri The Existential Cat" }
]);

div((
    h1("The Famous cats of YouTube"),
    ul(
        move || cats.iter().enumerate().map(|(idx, cat)| {
            let id = cat.id;
            let idx = idx + 1;
            let name = cat.name;
            li((
                a((
                    attr!{ target="_black", href=format!("v={id}") },
                    format!("{idx}: {name}")    
                ))
            ))
        })
    )
))
```

## promise ?

## event modifier

```rust
button("Click me").on(Click)
```

## event forward

https://svelte.dev/tutorial/event-forwarding


## binding value

```rust
let name = "world".to_string().reactive();

div((
    h1(move || format!("hello {name}")),
    input().value(name)
))
```

## specify input type

https://svelte.dev/tutorial/numeric-inputs

