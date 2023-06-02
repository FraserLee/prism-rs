use quick_js::Context;

const PRISM_JS: &str = include_str!("../prism/prism.js");

// not publicly exported
type PrismContext = Context;

/// initialize prism.js
pub fn init() -> PrismContext {
    let context = Context::new().unwrap();
    context.eval(PRISM_JS).unwrap();
    // context.eval("function highlight(text, grammar, language) { return Prism.highlight(text, grammer, language); }").unwrap();
    context
}

/// text: the code to be highlighted
/// grammar: the name of the prism.js language definition in the context
/// language: the name of the language definition passed to grammar
///
/// Example:
/// ```rust
/// use prism_js::{init, highlight_internal};
///
/// let mut context = init();
///
/// let text = "var foo = true;";
/// let grammar = "Prism.languages.javascript";
/// let language = "javascript";
///
/// let html = highlight_internal(&mut context, text, grammar, language);
/// assert!(html.is_some());
/// assert!(html.unwrap() == r#"<span class="token keyword">var</span> foo <span class="token operator">=</span> <span class="token boolean">true</span><span class="token punctuation">;</span>"#);
/// ```
pub fn highlight_internal(
    context: &mut PrismContext,
    text: &str,
    grammar: &str,
    language: &str,
) -> Option<String> {
    // context.call_function doesn't work here since the actual value for grammer is too large
    context.set_global("text", text).ok()?;
    context.set_global("language", language).ok()?;
    context
        .eval(&format!("Prism.highlight(text, {}, language)", grammar))
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
}

/// text: the code to be highlighted
/// language: the language to highlight the code in
///
/// Example:
/// ```rust
/// use prism_js::{init, highlight};
///
/// let mut context = init();
///
/// let html = highlight(&mut context, "var foo = true;", "javascript");
/// assert!(html.is_some());
/// assert!(html.unwrap() == r#"<span class="token keyword">var</span> foo <span class="token operator">=</span> <span class="token boolean">true</span><span class="token punctuation">;</span>"#);
/// ```
pub fn highlight(context: &mut PrismContext, text: &str, language: &str) -> Option<String> {
    highlight_internal(context, text, format!("Prism.languages.{}", language).as_str(), language)
}
