# prism-rs

Rust bindings for [Prism](https://prismjs.com/) syntax highlighting.

## Usage

```rust
use prism_js::{init, highlight}

let mut context = init();

let lang = "haskell";
let code = r#"
fibs = 0 : 1 : zipWith (+) fibs (tail fibs)
main = print $ take 100 fibs
"#;

let html = highlight(&mut context, code, lang);
println!("{}", html.unwrap());
```
which outputs:
```html
<span class="token hvariable">fibs</span> <span class="token operator">=</span>
<span class="token number">0</span> <span class="token operator">:</span> <span
class="token number">1</span> <span class="token operator">:</span> <span
class="token builtin">zipWith</span> <span class="token
punctuation">(</span><span class="token operator">+</span><span class="token
punctuation">)</span> <span class="token hvariable">fibs</span> <span
class="token punctuation">(</span><span class="token builtin">tail</span> <span
class="token hvariable">fibs</span><span class="token punctuation">)</span>
<span class="token hvariable">main</span> <span class="token operator">=</span>
<span class="token builtin">print</span> <span class="token operator">$</span>
<span class="token builtin">take</span> <span class="token number">100</span>
<span class="token hvariable">fibs</span>
```
