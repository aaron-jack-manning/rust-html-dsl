extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, format_ident};

fn split_args(token_stream : TokenStream) -> (Ident, Ident, proc_macro2::TokenStream) {
    let args : Vec<String> =
        token_stream
        .to_string()
        .split(",")
        .map(|x| x.trim().to_string())
        .map(|mut x| {x.retain(|y| !y.is_whitespace()); x})
        .collect();

    (format_ident!("{}", args[0]), format_ident!("{}", args[1]), args[2].parse().unwrap())
}

#[proc_macro]
pub fn parent_node(input : TokenStream) -> TokenStream {
    let (rust_name, macro_name, html_name) = split_args(input);

    let code = quote! {
        pub struct #rust_name {
            attributes : Vec<Box<dyn Attribute>>,
            children : Vec<Box<dyn Node>>,
            css_props : std::option::Option<Style>,
        }

        impl Node for #rust_name {}


        impl ToString for #rust_name {
            fn to_string(&self) -> String {
                let mut formatted = String::from(&format!("<{}", stringify!(#html_name)));
                for attribute in &self.attributes {
                    formatted.push(' ');
                    formatted.push_str(&attribute.to_string());
                }

                if let Some(props) = &self.css_props {
                    formatted.push_str(" style=\"");
                    for prop in &props.0 {
                        formatted.push_str(&prop.to_string());
                    }
                    formatted.push_str("\"");
                }
                formatted.push_str(">");

                for child in &self.children {
                    formatted.push_str(&child.to_string());
                }
                formatted.push_str(&format!("</{}>", stringify!(#html_name)));
                
                formatted
            }
        }

        impl #rust_name {
            pub fn new() -> Self {
                #rust_name {
                    attributes : Vec::new(),
                    children : Vec::new(),
                    css_props : None
                }
            }

            pub fn child<N>(&mut self, child : N)
                where N : Node, N : 'static {
                self.children.push(Box::new(child));
            }

            pub fn attribute<A>(&mut self, attribute : A)
                where A : Attribute, A : 'static {
                self.attributes.push(Box::new(attribute));
            }

            pub fn style(&mut self, style : Style) {
                self.css_props = Some(style);
            }
        }

        #[macro_export]
        macro_rules! #macro_name {
            ([$($a:expr),*][$($b:expr),*][$($c:expr),*]) => {
                {
                    let mut node = #rust_name::new();
                    $(node.attribute($a);)*
                    let mut style = Style::new();
                    $(style.with_prop($b);)*
                    node.style(style);
                    $(node.child($c);)*
                    node
                }
            };
            ([$($a:expr),*]($b:expr)[$($c:expr),*]) => {
                {
                    let mut node = #rust_name::new();
                    $(node.attribute($a);)*
                    node.style($b);
                    $(node.child($c);)*
                    node
                }
            };
            ([$($a:expr),*][$($c:expr),*]) => {
                {
                    let mut node = #rust_name::new();
                    $(node.attribute($a);)*
                    $(node.child($c);)*
                    node
                }
            };
            ([$($c:expr),*]) => {
                {
                    let mut node = #rust_name::new();
                    $(node.child($c);)*
                    node
                }
            }
        }
    };

    code.into()
}

#[proc_macro]
pub fn void_node(input : TokenStream) -> TokenStream {
    let (rust_name, macro_name, html_name) = split_args(input);

    let code = quote! {
        pub struct #rust_name {
            attributes : Vec<Box<dyn Attribute>>,
            css_props : std::option::Option<Style>,
        }

        impl Node for #rust_name {}

        impl ToString for #rust_name {
            fn to_string(&self) -> String {
                let mut formatted = String::from(&format!("<{}", stringify!(#html_name)));
                for attribute in &self.attributes {
                    formatted.push(' ');
                    formatted.push_str(&attribute.to_string());
                }
                if let Some(props) = &self.css_props {
                    formatted.push_str(" style=\"");
                    for prop in &props.0 {
                        formatted.push_str(&prop.to_string());
                    }
                    formatted.push_str("\"");
                }
                formatted.push_str(">");

                formatted
            }
        }

        impl #rust_name {
            pub fn new() -> Self {
                #rust_name {
                    attributes : Vec::new(),
                    css_props : None
                }
            }

            pub fn attribute<A>(&mut self, attribute : A)
                where A : Attribute, A : 'static {
                self.attributes.push(Box::new(attribute));
            }

            pub fn style(&mut self, style : Style) {
                self.css_props = Some(style);
            }
        }


        #[macro_export]
        macro_rules! #macro_name {
            ([$($a:expr),*][$($b:expr),*]) => {
                {
                    let mut node = #rust_name::new();
                    $(node.attribute($a);)*
                    let mut style = Style::new();
                    $(style.with_prop($b);)*
                    node.style(style);
                    node
                }
            };
            ([$($a:expr),*]($b:expr)) => {
                {
                    let mut node = #rust_name::new();
                    $(node.attribute($a);)*
                    node.style($b);
                    node
                }
            };
            ([$($a:expr),*]) => {
                {
                    let mut node = #rust_name::new();
                    $(node.attribute($a);)*
                    node
                }
            }   
        }
    };

    code.into()
}

#[proc_macro]
pub fn attribute(input : TokenStream) -> TokenStream {
    let (rust_name, function_name, html_name) = split_args(input);

    let code = quote! {
        pub struct #rust_name {
            value : String,
        }

        impl Attribute for #rust_name {}

        impl ToString for #rust_name {
            fn to_string(&self) -> String {
                format!("{}=\"{}\"", remove_whitespace(stringify!(#html_name)), self.value)
            }
        }

        pub fn #function_name(value : &str) -> #rust_name {
            #rust_name {
                value : String::from(value)
            }
        }
    };

    code.into()
}

#[proc_macro]
pub fn css_prop(input : TokenStream) -> TokenStream {
    let (rust_name, function_name, html_name) = split_args(input);

    let code = quote! {
        pub struct #rust_name {
            value : String,
        }

        impl CssProp for #rust_name {}

        impl ToString for #rust_name {
            fn to_string(&self) -> String {
                format!("{}: {};", remove_whitespace(stringify!(#html_name)), self.value)
            }
        }

        pub fn #function_name(value : &str) -> #rust_name {
            #rust_name {
                value : String::from(value)
            }
        }
    };

    code.into()
}

#[proc_macro]
pub fn all_nodes(_ : TokenStream) -> TokenStream {

    let parent_idents = vec![
        ("A", "a", "a"),
        ("Abbr", "abbr", "abbr"),
        ("Address", "address", "address"),
        ("Article", "article", "article"),
        ("Aside", "aside", "aside"),
        ("Audio", "audio", "audio"),
        ("B", "b", "b"),
        ("Bdi", "bdi", "bdi"),
        ("Bdo", "bdo", "bdo"),
        ("BlockQuote", "blockquote", "blockquote"),
        ("Body", "body", "body"),
        ("Button", "button", "button"),
        ("Canvas", "canvas", "canvas"),
        ("Caption", "caption", "caption"),
        ("Cite", "cite", "cite"),
        ("Code", "code", "code"),
        ("ColGroup", "colgroup", "colgroup"),
        ("Data", "data", "data"),
        ("DataList", "datalist", "datalist"),
        ("Dd", "dd", "dd"),
        ("Del", "del", "del"),
        ("Details", "details", "details"),
        ("Dfn", "dfn", "dfn"),
        ("Dialog", "dialog", "dialog"),
        ("Div", "div", "div"),
        ("Dl", "dl", "dl"),
        ("Dt", "dt", "dt"),
        ("Em", "em", "em"),
        ("FieldSet", "fieldset", "fieldset"),
        ("FigCaption", "figcaption", "figcaption"),
        ("Figure", "figure", "figure"),
        ("Footer", "footer", "footer"),
        ("Form", "form", "form"),
        ("H1", "h1", "h1"),
        ("H2", "h2", "h2"),
        ("H3", "h3", "h3"),
        ("H4", "h4", "h4"),
        ("H5", "h5", "h5"),
        ("H6", "h6", "h6"),
        ("Head", "head", "head"),
        ("Header", "header", "header"),
        ("Html", "html", "html"),
        ("I", "i", "i"),
        ("IFrame", "iframe", "iframe"),
        ("Ins", "ins", "ins"),
        ("Kbd", "kbd", "kbd"),
        ("Label", "label", "label"),
        ("Legend", "legend", "legend"),
        ("Li", "li", "li"),
        ("Main", "main", "main"),
        ("Map", "map", "map"),
        ("Mark", "mark", "mark"),
        ("Meter", "meter", "meter"),
        ("Nav", "nav", "nav"),
        ("NoScript", "noscript", "noscript"),
        ("Object", "object", "object"),
        ("Ol", "ol", "ol"),
        ("OptGroup", "optgroup", "optgroup"),
        ("Option", "option", "option"),
        ("Output", "output", "output"),
        ("P", "p", "p"),
        ("Picture", "picture", "picture"),
        ("Pre", "pre", "pre"),
        ("Progress", "progress", "progress"),
        ("Q", "q", "q"),
        ("Rp", "rp", "rp"),
        ("Rt", "rt", "rt"),
        ("Ruby", "ruby", "ruby"),
        ("S", "s", "s"),
        ("SAmp", "samp", "samp"),
        ("Script", "script", "script"),
        ("Section", "section", "section"),
        ("Select", "select", "select"),
        ("Small", "small", "small"),
        ("Span", "span", "span"),
        ("Strong", "strong", "strong"),
        ("Sub", "sub", "sub"),
        ("Summary", "summary", "summary"),
        ("Sup", "sup", "sup"),
        ("Svg", "svg", "svg"),
        ("Table", "table", "table"),
        ("TBody", "tbody", "tbody"),
        ("Td", "td", "td"),
        ("Template", "template", "template"),
        ("TextArea", "textarea", "textarea"),
        ("TFoot", "tfoot", "tfoot"),
        ("Th", "th", "th"),
        ("THead", "thead", "thead"),
        ("Time", "time", "time"),
        ("Title", "title", "title"),
        ("Tr", "tr", "tr"),
        ("U", "u", "u"),
        ("Ul", "ul", "ul"),
        ("Var", "var", "var"),
        ("Video", "video", "video"),
    ];

    let void_idents = vec![
        ("Area", "area", "area"),
        ("Base", "base", "base"),
        ("Br", "br", "br"),
        ("Col", "col", "col"),
        ("Embed", "embed", "embed"),
        ("Hr", "hr", "hr"),
        ("Img", "img", "img"),
        ("Input", "input", "input"),
        ("Link", "link", "link"),
        ("Meta", "meta", "meta"),
        ("Param", "param", "param"),
        ("Source", "source", "source"),
        ("Track", "track", "track"),
        ("Wbr", "wbr", "wbr"),
    ];

    let mut code = String::new();

    for (rust_name, macro_name, html_name) in parent_idents {
        code.push_str(&format!("parent_node!({}, {}, {});", rust_name, macro_name, html_name));
    }

    for (rust_name, macro_name, html_name) in void_idents {
        code.push_str(&format!("void_node!({}, {}, {});", rust_name, macro_name, html_name));
    }

    code.parse().unwrap()
}

#[proc_macro]
pub fn all_attributes(_ : TokenStream) -> TokenStream {

    let idents = vec![
        ("Accept", "accept", "accept"),
        ("AcceptCharset", "accept_charset", "accept-charset"),
        ("AccessKey", "accesskey", "accesskey"),
        ("Action", "action", "action"),
        ("Alt", "alt", "alt"),
        ("Async", "r#async", "async"),
        ("AutoComplete", "autocomplete", "autocomplete"),
        ("AutoFocus", "autofocus", "autofocus"),
        ("AutoPlay", "autoplay", "autoplay"),
        ("CharSet", "charset", "charset"),
        ("Checked", "checked", "checked"),
        ("Cite", "cite", "cite"),
        ("Class", "class", "class"),
        ("Cols", "cols", "cols"),
        ("ColSpan", "colspan", "colspan"),
        ("Content", "content", "content"),
        ("ContentEditable", "contenteditable", "contenteditable"),
        ("Controls", "controls", "controls"),
        ("Coords", "coords", "coords"),
        ("Data", "data", "data"),
        ("DateTime", "datetime", "datetime"),
        ("Default", "default", "default"),
        ("Defer", "defer", "defer"),
        ("Dir", "dir", "dir"),
        ("DirName", "dirname", "dirname"),
        ("Disabled", "disabled", "disabled"),
        ("Download", "download", "download"),
        ("Draggable", "draggable", "draggable"),
        ("EncType", "enctype", "enctype"),
        ("For", "r#for", "for"),
        ("Form", "form", "form"),
        ("FormAction", "formaction", "formaction"),
        ("Headers", "headers", "headers"),
        ("Height", "height", "height"),
        ("Hidden", "hidden", "hidden"),
        ("High", "high", "high"),
        ("Href", "href", "href"),
        ("HrefLang", "hreflang", "hreflang"),
        ("HttpEquiv", "http_equiv", "http-equiv"),
        ("Id", "id", "id"),
        ("IsMap", "ismap", "ismap"),
        ("Kind", "kind", "kind"),
        ("Label", "label", "label"),
        ("Lang", "lang", "lang"),
        ("List", "list", "list"),
        ("Loop", "r#loop", "loop"),
        ("Low", "low", "low"),
        ("Max", "max", "max"),
        ("MaxLength", "maxlength", "maxlength"),
        ("Media", "media", "media"),
        ("Method", "method", "method"),
        ("Min", "min", "min"),
        ("Multiple", "multiple", "multiple"),
        ("Muted", "muted", "muted"),
        ("Name", "name", "name"),
        ("NoValidate", "novalidate", "novalidate"),
        ("OnAbort", "onabort", "onabort"),
        ("OnAfterPrint", "onafterprint", "onafterprint"),
        ("OnBeforePrint", "onbeforeprint", "onbeforeprint"),
        ("OnBeforeUnload", "onbeforeunload", "onbeforeunload"),
        ("OnBlur", "onblur", "onblur"),
        ("OnCanPlay", "oncanplay", "oncanplay"),
        ("OnCanPlaythrough", "oncanplaythrough", "oncanplaythrough"),
        ("OnChange", "onchange", "onchange"),
        ("OnClick", "onclick", "onclick"),
        ("OnContextMenu", "oncontextmenu", "oncontextmenu"),
        ("OnCopy", "oncopy", "oncopy"),
        ("OnCueChange", "oncuechange", "oncuechange"),
        ("OnCut", "oncut", "oncut"),
        ("OndblClick", "ondblclick", "ondblclick"),
        ("OnDrag", "ondrag", "ondrag"),
        ("OnDragEnd", "ondragend", "ondragend"),
        ("OnDragEnter", "ondragenter", "ondragenter"),
        ("OnDragLeave", "ondragleave", "ondragleave"),
        ("OnDragOver", "ondragover", "ondragover"),
        ("OnDragStart", "ondragstart", "ondragstart"),
        ("OnDrop", "ondrop", "ondrop"),
        ("OndurationChange", "ondurationchange", "ondurationchange"),
        ("OnEmptied", "onemptied", "onemptied"),
        ("OnEnded", "onended", "onended"),
        ("OnError", "onerror", "onerror"),
        ("OnFocus", "onfocus", "onfocus"),
        ("OnHashChange", "onhashchange", "onhashchange"),
        ("OnInput", "oninput", "oninput"),
        ("OnInvalid", "oninvalid", "oninvalid"),
        ("OnKeydown", "onkeydown", "onkeydown"),
        ("OnKeypress", "onkeypress", "onkeypress"),
        ("OnKeyup", "onkeyup", "onkeyup"),
        ("OnLoad", "onload", "onload"),
        ("OnLoadedData", "onloadeddata", "onloadeddata"),
        ("OnLoadedMetadata", "onloadedmetadata", "onloadedmetadata"),
        ("OnLoadStart", "onloadstart", "onloadstart"),
        ("OnMouseDown", "onmousedown", "onmousedown"),
        ("OnMouseMove", "onmousemove", "onmousemove"),
        ("OnMouseOut", "onmouseout", "onmouseout"),
        ("OnMouseOver", "onmouseover", "onmouseover"),
        ("OnMouseUp", "onmouseup", "onmouseup"),
        ("OnMouseWheel", "onmousewheel", "onmousewheel"),
        ("OnOffline", "onoffline", "onoffline"),
        ("OnOnline", "ononline", "ononline"),
        ("OnPageHide", "onpagehide", "onpagehide"),
        ("OnPageShow", "onpageshow", "onpageshow"),
        ("OnPaste", "onpaste", "onpaste"),
        ("OnPause", "onpause", "onpause"),
        ("OnPlay", "onplay", "onplay"),
        ("OnPlaying", "onplaying", "onplaying"),
        ("OnPopState", "onpopstate", "onpopstate"),
        ("OnProgress", "onprogress", "onprogress"),
        ("OnRateChange", "onratechange", "onratechange"),
        ("OnReset", "onreset", "onreset"),
        ("OnResize", "onresize", "onresize"),
        ("OnScroll", "onscroll", "onscroll"),
        ("OnSearch", "onsearch", "onsearch"),
        ("OnSeeked", "onseeked", "onseeked"),
        ("OnSeeking", "onseeking", "onseeking"),
        ("OnSelect", "onselect", "onselect"),
        ("OnStalled", "onstalled", "onstalled"),
        ("OnStorage", "onstorage", "onstorage"),
        ("OnSubmit", "onsubmit", "onsubmit"),
        ("OnSuspend", "onsuspend", "onsuspend"),
        ("OnTimeUpdate", "ontimeupdate", "ontimeupdate"),
        ("OnToggle", "ontoggle", "ontoggle"),
        ("OnUnload", "onunload", "onunload"),
        ("OnVolumeChange", "onvolumechange", "onvolumechange"),
        ("OnWaiting", "onwaiting", "onwaiting"),
        ("OnWheel", "onwheel", "onwheel"),
        ("Open", "open", "open"),
        ("Optimum", "optimum", "optimum"),
        ("Pattern", "pattern", "pattern"),
        ("PlaceHolder", "placeholder", "placeholder"),
        ("Poster", "poster", "poster"),
        ("Preload", "preload", "preload"),
        ("Readonly", "readonly", "readonly"),
        ("Rel", "rel", "rel"),
        ("Required", "required", "required"),
        ("Reversed", "reversed", "reversed"),
        ("Rows", "rows", "rows"),
        ("RowSpan", "rowspan", "rowspan"),
        ("Sandbox", "sandbox", "sandbox"),
        ("Scope", "scope", "scope"),
        ("Selected", "selected", "selected"),
        ("Shape", "shape", "shape"),
        ("Size", "size", "size"),
        ("Sizes", "sizes", "sizes"),
        ("Span", "span", "span"),
        ("SpellCheck", "spellcheck", "spellcheck"),
        ("Src", "src", "src"),
        ("SrcDoc", "srcdoc", "srcdoc"),
        ("SrcLang", "srclang", "srclang"),
        ("SrcSet", "srcset", "srcset"),
        ("Start", "start", "start"),
        ("Step", "step", "step"),
        ("Style", "style", "style"),
        ("TabIndex", "tabindex", "tabindex"),
        ("Target", "target", "target"),
        ("Title", "title", "title"),
        ("Translate", "translate", "translate"),
        ("Type", "r#type", "type"),
        ("UseMap", "usemap", "usemap"),
        ("Value", "value", "value"),
        ("Width", "width", "width"),
        ("Wrap", "wrap", "wrap"),
    ];

    let mut code = String::new();

    for (rust_name, function_name, html_name) in idents {
        code.push_str(&format!("attribute!({}, {}, {});", rust_name, function_name, html_name));
    }

    code.parse().unwrap()
}

#[proc_macro]
pub fn all_css_props(_ : TokenStream) -> TokenStream {

    let idents = vec![
        ("Background", "background", "background"),
        ("BackgroundAttachment", "background_attachment", "background-attachment"),
        ("BackgroundColor", "background_color", "background-color"),
        ("BackgroundImage", "background_image", "background-image"),
        ("BackgroundPosition", "background_position", "background-position"),
        ("BackgroundRepeat", "background_repeat", "background-repeat"),
        ("Border", "border", "border"),
        ("BorderBottom", "border_bottom", "border-bottom"),
        ("BorderBottomColor", "border_bottom_color", "border-bottom-color"),
        ("BorderBottomStyle", "border_bottom_style", "border-bottom-style"),
        ("BorderBottomWidth", "border_bottom_width", "border-bottom-width"),
        ("BorderColor", "border_color", "border-color"),
        ("BorderLeft", "border_left", "border-left"),
        ("BorderLeftColor", "border_left_color", "border-left-color"),
        ("BorderLeftStyle", "border_left_style", "border-left-style"),
        ("BorderLeftWidth", "border_left_width", "border-left-width"),
        ("BorderRight", "border_right", "border-right"),
        ("BorderRightColor", "border_right_color", "border-right-color"),
        ("BorderRightStyle", "border_right_style", "border-right-style"),
        ("BorderRightWidth", "border_right_width", "border-right-width"),
        ("BorderStyle", "border_style", "border-style"),
        ("BorderTop", "border_top", "border-top"),
        ("BorderTopColor", "border_top_color", "border-top-color"),
        ("BorderTopStyle", "border_top_style", "border-top-style"),
        ("BorderTopWidth", "border_top_width", "border-top-width"),
        ("BorderWidth", "border_width", "border-width"),
        ("Clear", "clear", "clear"),
        ("Clip", "clip", "clip"),
        ("Color", "color", "color"),
        ("Cursor", "cursor", "cursor"),
        ("Display", "display", "display"),
        ("Filter", "filter", "filter"),
        ("Float", "float", "float"),
        ("Font", "font", "font"),
        ("FontFamily", "font_family", "font-family"),
        ("FontSize", "font_size", "font-size"),
        ("FontVariant", "font_variant", "font-variant"),
        ("FontWeight", "font_weight", "font-weight"),
        ("Height", "height", "height"),
        ("Left", "left", "left"),
        ("LetterSpacing", "letter_spacing", "letter-spacing"),
        ("LineHeight", "line_height", "line-height"),
        ("ListStyle", "list_style", "list-style"),
        ("ListStyleImage", "list_style_image", "list-style-image"),
        ("ListStylePosition", "list_style_position", "list-style-position"),
        ("ListStyleType", "list_style_type", "list-style-type"),
        ("Margin", "margin", "margin"),
        ("MarginBottom", "margin_bottom", "margin-bottom"),
        ("MarginLeft", "margin_left", "margin-left"),
        ("MarginRight", "margin_right", "margin-right"),
        ("MarginTop", "margin_top", "margin-top"),
        ("Overflow", "overflow", "overflow"),
        ("Padding", "padding", "padding"),
        ("PaddingBottom", "padding_bottom", "padding-bottom"),
        ("PaddingLeft", "padding_left", "padding-left"),
        ("PaddingRight", "padding_right", "padding-right"),
        ("PaddingTop", "padding_top", "padding-top"),
        ("PageBreakAfter", "page_break_after", "page-break-after"),
        ("PageBreakBefore", "page_break_before", "page-break-before"),
        ("Position", "position", "position"),
        ("StrokeDasharray", "stroke_dasharray", "stroke-dasharray"),
        ("StrokeDashoffset", "stroke_dashoffset", "stroke-dashoffset"),
        ("TextAlign", "text_align", "text-align"),
        ("TextDecoration", "text_decoration", "text-decoration"),
        ("TextIndent", "text_indent", "text-indent"),
        ("TextTransform", "text_transform", "text-transform"),
        ("Top", "top", "top"),
        ("VerticalAlign", "vertical_align", "vertical-align"),
        ("Visibility", "visibility", "visibility"),
        ("Width", "width", "width"),
        ("ZIndex", "z_index", "z-index"),
    ];

    let mut code = String::new();

    for (rust_name, function_name, html_name) in idents {
        code.push_str(&format!("css_prop!({}, {}, {});", rust_name, function_name, html_name));
    }

    code.parse().unwrap()
}

//#[proc_macro_derive(Something)]
//pub fn something(input : TokenStream) -> TokenStream {
//    let ast : DeriveInput = syn::parse(input).unwrap();
//
//    let identifier = ast.ident;
//
//    let expanded = quote! {
//        impl Something for #identifier {}
//    };
//    
//    expanded.into()
//}


