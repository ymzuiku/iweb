iweb

Easy use web_sys.

```rust
use iweb;

fn main(){
    iweb::body().append(iweb::tag("div").text("Hello world"));
}
```