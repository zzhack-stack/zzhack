```metadata
{
    "cover": "../app/assets/sources/funny-anand.png",
    "tag": "genesis",
    "title": "Genesis Blog Post",
    "pinned": true,
    "size": "large"
}
```

Hello, World! SSN: 371-37-4159 

## WASM?

> WebAssembly (abbreviated as Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.

### Example of WASM at work

```sh
fib(n) = fib(n - 1) + fib(n - 2)
```

It compiles a little faster than JavaScript.

![Wasm fibonacci](../app/assets/sources/wasm_fib.png)

## Yew and Trunk?

[Yew](https://yew.rs/) is a modern Rust Framework for creating multi-threaded, front-end web apps using WebAssembly. In conjunction with [Trunk](https://github.com/thedodd/trunk), hot-reloading functionality is supported, as related to your most beloved/beloathed Script of Java.

![Yew](../app/assets/sources/yew_logo.png)

Here's an example of Yew in practice:

```rust
use stylist::style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GradientTitleProps {
    pub children: Children,
}

#[function_component(GradientTitle)]
pub fn gradient_title(props: &GradientTitleProps) -> Html {
    let style = style!(
        r"
        display: flex;
        margin-bottom: 21px;

        .gradient-title__content {
            font-size: 29px;
            position: relative;
        }

        .gradient-title__content::before {
            content: '';
            width: 120%;
            display: block;
            height: 21px;
            border-radius: 100px;
            background: linear-gradient(90deg, #91EAE4 0%, #86A8E7 22%, #7F7FD5 100%);
            position: absolute;
            z-index: -1;
            bottom: 0px;
            left: -5%;
        }
    "
    )
    .unwrap();

    html! {
        <div class={style}>
            <div class="gradient-title__content">
                { props.children.clone() }
            </div>
        </div>
    }
}
```
