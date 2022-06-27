
// Used within macros for HTML names which require -, and thus expand with extra whitespace.
fn remove_whitespace(string : &str) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
}

pub mod attr {
    use crate::remove_whitespace;
    use proc_macros::*;

    pub trait Attribute : ToString {}

    all_attributes!();

    pub struct Custom {
        attr : String,
        value : String,
    }
    pub fn custom(attr : &str, value : &str) -> Custom {
        Custom {
            attr : String::from(attr),
            value : String::from(value),
        }
    }
    impl ToString for Custom {
        fn to_string(&self) -> String {
            format!("{}=\"{}\"", self.attr, self.value)
        }
    }
    impl Attribute for Custom {}

}

// Should probably reference from proc_macros within the macros for every invocation to avoid the
// need for the import.
#[macro_use]
pub mod nodes {
    use proc_macros::*;
    use crate::css::Style;
    use crate::attr::Attribute;

    pub trait Node : ToString {}

    pub trait ParentNode : Node {
        fn child<N>(&mut self, child : N)
            where N : Node, N : 'static;
    }

    pub trait StylableNode : Node {
        fn style(&mut self, style : Style);
    }

    pub trait AttributableNode : Node {
        fn attribute<A>(&mut self, attribute : A)
            where A : Attribute, A : 'static;
    }

    all_nodes!();

    // Text primitive
    pub struct Text {
        value : String
    }
    impl ToString for Text {
        fn to_string(&self) -> String {
            self.value.clone()
        }
    }
    impl Text {
        pub fn new(value : &str) -> Text {
            Text {
                value : String::from(value)
            }
        }
    }
    impl Node for Text {}
    #[macro_export]
    macro_rules! text {
        [$a:expr] => {
            Text::new($a)
        }
    }
}

#[macro_use]
pub mod css {
    use proc_macros::*;
    use crate::remove_whitespace;

    pub trait CssProp : ToString {}

    all_css_props!();

    pub struct Style(pub (in crate) Vec<Box<dyn CssProp + 'static>>);
    impl Style {
        pub fn new() -> Style {
            Style(Vec::new())
        }

        pub fn with_prop<P : 'static>(&mut self, item : P)
            where P : CssProp {
            self.0.push(Box::new(item));
        }
    }

    #[macro_export]
    macro_rules! style {
        ($($a:expr),*) => {
            {
                let mut style = Style::new();
                $(style.with_prop($a);)*
                style
            }
        }
    }
}
