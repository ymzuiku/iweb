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

Or use call link:

```rust
use iweb;

fn main(){
    iweb::body().append(&iweb::tag("div").text("Hello world"));
}
```