# Rust HTML DSL

A simple HTML DSL for Rust.

## Example

### Rust
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

## Future Development

This project is quite simple, and will likely stay very simple by design, but I do plan on adding additional traits such that some subset of the HTML spec can be statically analysed, i.e. invalid children for their corresponding parent will be a type error. I have a working example of this but there's a lot of boilerplate so I am trying to implement without compromising maintainability.
