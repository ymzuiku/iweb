iweb

Easy use web_sys.

```rust
use iweb;

fn main(){
    let div = iweb::tag("div");
    div = div.text("hello");
    iweb::body().append(&div);
}
```

Or

```rust
use iweb;

fn main(){
    iweb::body().append(&iweb::tag("div").text("Hello world"));
}
```