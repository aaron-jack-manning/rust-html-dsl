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

        impl ParentNode for #rust_name {
            fn child<N>(&mut self, child : N)
                where N : Node, N : 'static {
                self.children.push(Box::new(child));
            }
        }

        impl StylableNode for #rust_name {
            fn style(&mut self, style : Style) {
                self.css_props = Some(style);
            }
        }
        
        impl AttributableNode for #rust_name {
            fn attribute<A>(&mut self, attribute : A)
                where A : Attribute, A : 'static {
                self.attributes.push(Box::new(attribute));
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

        impl StylableNode for #rust_name {
            fn style(&mut self, style : Style) {
                self.css_props = Some(style);
            }
        }

        impl AttributableNode for #rust_name {
            fn attribute<A>(&mut self, attribute : A)
                where A : Attribute, A : 'static {
                self.attributes.push(Box::new(attribute));
            }
        }

        impl #rust_name {
            pub fn new() -> Self {
                #rust_name {
                    attributes : Vec::new(),
                    css_props : None
                }
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
        ("AlignContent", "align_content", "align-content"),
        ("AlignItems", "align_items", "align-items"),
        ("AlignSelf", "align_self", "align-self"),
        ("All", "all", "all"),
        ("Animation", "animation", "animation"),
        ("AnimationDelay", "animation_delay", "animation-delay"),
        ("AnimationDirection", "animation_direction", "animation-direction"),
        ("AnimationDuration", "animation_duration", "animation-duration"),
        ("AnimationFillMode", "animation_fill_mode", "animation-fill-mode"),
        ("AnimationIterationCount", "animation_iteration_count", "animation-iteration-count"),
        ("AnimationName", "animation_name", "animation-name"),
        ("AnimationPlayState", "animation_play_state", "animation-play-state"),
        ("AnimationTimingFunction", "animation_timing_function", "animation-timing-function"),
        ("BackfaceVisibility", "backface_visibility", "backface-visibility"),
        ("Background", "background", "background"),
        ("BackgroundAttachment", "background_attachment", "background-attachment"),
        ("BackgroundBlendMode", "background_blend_mode", "background-blend-mode"),
        ("BackgroundClip", "background_clip", "background-clip"),
        ("BackgroundColor", "background_color", "background-color"),
        ("BackgroundImage", "background_image", "background-image"),
        ("BackgroundOrigin", "background_origin", "background-origin"),
        ("BackgroundPosition", "background_position", "background-position"),
        ("BackgroundRepeat", "background_repeat", "background-repeat"),
        ("BackgroundSize", "background_size", "background-size"),
        ("Border", "border", "border"),
        ("BorderBottom", "border_bottom", "border-bottom"),
        ("BorderBottomColor", "border_bottom_color", "border-bottom-color"),
        ("BorderBottomLeftRadius", "border_bottom_left_radius", "border-bottom-left-radius"),
        ("BorderBottomRightRadius", "border_bottom_right_radius", "border-bottom-right-radius"),
        ("BorderBottomStyle", "border_bottom_style", "border-bottom-style"),
        ("BorderBottomWidth", "border_bottom_width", "border-bottom-width"),
        ("BorderCollapse", "border_collapse", "border-collapse"),
        ("BorderColor", "border_color", "border-color"),
        ("BorderImage", "border_image", "border-image"),
        ("BorderImageOutset", "border_image_outset", "border-image-outset"),
        ("BorderImageRepeat", "border_image_repeat", "border-image-repeat"),
        ("BorderImageSlice", "border_image_slice", "border-image-slice"),
        ("BorderImageSource", "border_image_source", "border-image-source"),
        ("BorderImageWidth", "border_image_width", "border-image-width"),
        ("BorderLeft", "border_left", "border-left"),
        ("BorderLeftColor", "border_left_color", "border-left-color"),
        ("BorderLeftStyle", "border_left_style", "border-left-style"),
        ("BorderLeftWidth", "border_left_width", "border-left-width"),
        ("BorderRadius", "border_radius", "border-radius"),
        ("BorderRight", "border_right", "border-right"),
        ("BorderRightColor", "border_right_color", "border-right-color"),
        ("BorderRightStyle", "border_right_style", "border-right-style"),
        ("BorderRightWidth", "border_right_width", "border-right-width"),
        ("BorderSpacing", "border_spacing", "border-spacing"),
        ("BorderStyle", "border_style", "border-style"),
        ("BorderTop", "border_top", "border-top"),
        ("BorderTopColor", "border_top_color", "border-top-color"),
        ("BorderTopLeftRadius", "border_top_left_radius", "border-top-left-radius"),
        ("BorderTopRightRadius", "border_top_right_radius", "border-top-right-radius"),
        ("BorderTopStyle", "border_top_style", "border-top-style"),
        ("BorderTopWidth", "border_top_width", "border-top-width"),
        ("BorderWidth", "border_width", "border-width"),
        ("Bottom", "bottom", "bottom"),
        ("BoxShadow", "box_shadow", "box-shadow"),
        ("BoxSizing", "box_sizing", "box-sizing"),
        ("CaptionSide", "caption_side", "caption-side"),
        ("CaretColor", "caret_color", "caret-color"),
        ("Clear", "clear", "clear"),
        ("Clip", "clip", "clip"),
        ("ClipPath", "clip_path", "clip-path"),
        ("Color", "color", "color"),
        ("ColumnCount", "column_count", "column-count"),
        ("ColumnFill", "column_fill", "column-fill"),
        ("ColumnGap", "column_gap", "column-gap"),
        ("ColumnRule", "column_rule", "column-rule"),
        ("ColumnRuleColor", "column_rule_color", "column-rule-color"),
        ("ColumnRuleStyle", "column_rule_style", "column-rule-style"),
        ("ColumnRuleWidth", "column_rule_width", "column-rule-width"),
        ("ColumnSpan", "column_span", "column-span"),
        ("ColumnWidth", "column_width", "column-width"),
        ("Columns", "columns", "columns"),
        ("Content", "content", "content"),
        ("CounterIncrement", "counter_increment", "counter-increment"),
        ("CounterReset", "counter_reset", "counter-reset"),
        ("Cursor", "cursor", "cursor"),
        ("DirectionLevel", "direction_level", "direction-level"),
        ("Display", "display", "display"),
        ("EmptyCells", "empty_cells", "empty-cells"),
        ("Filter", "filter", "filter"),
        ("Flex", "flex", "flex"),
        ("FlexBasis", "flex_basis", "flex-basis"),
        ("FlexDirection", "flex_direction", "flex-direction"),
        ("FlexFlow", "flex_flow", "flex-flow"),
        ("FlexGrow", "flex_grow", "flex-grow"),
        ("FlexShrink", "flex_shrink", "flex-shrink"),
        ("FlexWrap", "flex_wrap", "flex-wrap"),
        ("Float", "float", "float"),
        ("Font", "font", "font"),
        ("FontFamily", "font_family", "font-family"),
        ("FontKerning", "font_kerning", "font-kerning"),
        ("FontSize", "font_size", "font-size"),
        ("FontSizeAdjustBack", "font_size_adjust_back", "font-size-adjust-back"),
        ("FontStretch", "font_stretch", "font-stretch"),
        ("FontStyle", "font_style", "font-style"),
        ("FontVariantCaps", "font_variant_caps", "font-variant-caps"),
        ("FontWeight", "font_weight", "font-weight"),
        ("Grid", "grid", "grid"),
        ("GridArea", "grid_area", "grid-area"),
        ("GridAutoColumns", "grid_auto_columns", "grid-auto-columns"),
        ("GridAutoFlow", "grid_auto_flow", "grid-auto-flow"),
        ("GridAutoRows", "grid_auto_rows", "grid-auto-rows"),
        ("GridColumn", "grid_column", "grid-column"),
        ("GridColumnEndLine", "grid_column_end_line", "grid-column-end-line"),
        ("GridColumnGap", "grid_column_gap", "grid-column-gap"),
        ("GridColumnStart", "grid_column_start", "grid-column-start"),
        ("GridGap", "grid_gap", "grid-gap"),
        ("GridRow", "grid_row", "grid-row"),
        ("GridRowEndLine", "grid_row_end_line", "grid-row-end-line"),
        ("GridRowGap", "grid_row_gap", "grid-row-gap"),
        ("GridRowStart", "grid_row_start", "grid-row-start"),
        ("GridTemplate", "grid_template", "grid-template"),
        ("GridTemplateAreas", "grid_template_areas", "grid-template-areas"),
        ("GridTemplateColumns", "grid_template_columns", "grid-template-columns"),
        ("GridTemplateRows", "grid_template_rows", "grid-template-rows"),
        ("Height", "height", "height"),
        ("Hyphens", "hyphens", "hyphens"),
        ("JustifyContent", "justify_content", "justify-content"),
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
        ("MaxHeight", "max_height", "max-height"),
        ("MaxWidth", "max_width", "max-width"),
        ("MinHeight", "min_height", "min-height"),
        ("MinWidth", "min_width", "min-width"),
        ("ObjectFit", "object_fit", "object-fit"),
        ("ObjectPosition", "object_position", "object-position"),
        ("Opacity", "opacity", "opacity"),
        ("Order", "order", "order"),
        ("Outline", "outline", "outline"),
        ("OutlineColor", "outline_color", "outline-color"),
        ("OutlineOffset", "outline_offset", "outline-offset"),
        ("OutlineStyle", "outline_style", "outline-style"),
        ("OutlineWidth", "outline_width", "outline-width"),
        ("Overflow", "overflow", "overflow"),
        ("OverflowX", "overflow_x", "overflow-x"),
        ("OverflowY", "overflow_y", "overflow-y"),
        ("Padding", "padding", "padding"),
        ("PaddingBottom", "padding_bottom", "padding-bottom"),
        ("PaddingLeft", "padding_left", "padding-left"),
        ("PaddingRight", "padding_right", "padding-right"),
        ("PaddingTop", "padding_top", "padding-top"),
        ("PageBreakAfterBreak", "page_break_after_break", "page-break-after-break"),
        ("PageBreakBeforeBreak", "page_break_before_break", "page-break-before-break"),
        ("PageBreakInsideBreak", "page_break_inside_break", "page-break-inside-break"),
        ("PerspectivePositioned", "perspective_positioned", "perspective-positioned"),
        ("PerspectiveOriginPositioned", "perspective_origin_positioned", "perspective-origin-positioned"),
        ("PointerEvents", "pointer_events", "pointer-events"),
        ("Position", "position", "position"),
        ("Quotes", "quotes", "quotes"),
        ("Right", "right", "right"),
        ("ScrollBehavior", "scroll_behavior", "scroll-behavior"),
        ("TableLayout", "table_layout", "table-layout"),
        ("TextAlign", "text_align", "text-align"),
        ("TextAlignLast", "text_align_last", "text-align-last"),
        ("TextDecoration", "text_decoration", "text-decoration"),
        ("TextDecorationColor", "text_decoration_color", "text-decoration-color"),
        ("TextDecorationLine", "text_decoration_line", "text-decoration-line"),
        ("TextDecorationStyle", "text_decoration_style", "text-decoration-style"),
        ("TextIndent", "text_indent", "text-indent"),
        ("TextJustify", "text_justify", "text-justify"),
        ("TextOverflow", "text_overflow", "text-overflow"),
        ("TextShadow", "text_shadow", "text-shadow"),
        ("TextTransform", "text_transform", "text-transform"),
        ("Top", "top", "top"),
        ("Transform", "transform", "transform"),
        ("TransformOrigin", "transform_origin", "transform-origin"),
        ("TransformStyle", "transform_style", "transform-style"),
        ("Transition", "transition", "transition"),
        ("TransitionDelay", "transition_delay", "transition-delay"),
        ("TransitionDuration", "transition_duration", "transition-duration"),
        ("TransitionProperty", "transition_property", "transition-property"),
        ("TransitionTimingFunction", "transition_timing_function", "transition-timing-function"),
        ("UserSelect", "user_select", "user-select"),
        ("VerticalAlign", "vertical_align", "vertical-align"),
        ("Visibility", "visibility", "visibility"),
        ("WhiteSpaceSpace", "white_space_space", "white-space-space"),
        ("Width", "width", "width"),
        ("WordBreak", "word_break", "word-break"),
        ("WordSpacing", "word_spacing", "word-spacing"),
        ("WordWrap", "word_wrap", "word-wrap"),
        ("WritingMode", "writing_mode", "writing-mode"),
        ("ZIndex", "z_index", "z-index"),
    ];

    let mut code = String::new();

    for (rust_name, function_name, html_name) in idents {
        code.push_str(&format!("css_prop!({}, {}, {});", rust_name, function_name, html_name));
    }

    code.parse().unwrap()
}


