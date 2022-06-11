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

    let page =
        html!([lang("en")][
            head!([][
                meta!([charset("utf-8")]),
                meta!([name("description"), attr::content("This is a demo of this DSL.")])
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
