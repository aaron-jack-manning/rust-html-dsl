# Rust HTML DSL

A simple HTML DSL for Rust.

## Design

I created this to be a relatively simple abstraction on HTML such that the code written is close to HTML but gives basic programmatic abstraction such as being able to inline all CSS properties but still reuse them, and generate HTML from other filetypes such as markdown statically, so pure HTML and CSS can still be served. As a consequence of this, there are no runtime checks for injecting things that mess up the HTML in a string. For example using `"` is attributes rather than `&quot;` or injecting HTML tags into a `Text` (`text!`) node.

## Including in a Project

Add to your `Cargo.toml`:

```
dsl = { git = "https://github.com/aaron-jack-manning/rust-html-dsl" }
proc_macros = { git = "https://github.com/aaron-jack-manning/rust-html-dsl" }
```

## Example

### Rust Code

```
use dsl::{
    *,
    nodes::*,
    attr::*,
    css::*,
    css::Style,
};

fn main() {
    let heading_style = style![
        color("red"),
        font_family("monospace")
    ];

    let page = html!([lang("en")][
        head!([][
            meta!([charset("utf-8")]),
            meta!([name("description"), content("This is a demo of this DSL.")])
        ]),
        body!([][
            div!([][
                h4!([](heading_style)[
                    text!["Heading"]
                ]),
                p!([][font_size("14pt")][
                    text!["This is some paragraph text."]
                ])
            ])
        ])
    ]);

    println!("{}", page.to_string());
}
```

### Generated HTML (prettified)
```
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="description" content="This is a demo of this DSL.">
    </head>
    <body>
        <div>
            <h4 style="color: red;font-family: monospace;">
                Heading
            </h4>
            <p style="font-size: 14pt;">
                This is some paragraph text.
            </p>
        </div>
    </body>
</html>
```
