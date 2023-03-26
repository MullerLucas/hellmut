use std::collections::HashMap;

use hell_error::{HellResult, HellError};
use crate::{console_log, view::EventHandler};
use crate::error::ErrToWebHellErr;

use super::{ViewCtx, ElementTree};



#[derive(Debug)]
pub enum ElementVariant {
    Invalid,
    Html,

    Button,
    Div,
    Paragraph,
    Style,
}

impl ElementVariant {
    pub fn tag_name(&self) -> &'static str {
        match self {
            ElementVariant::Invalid     |
            ElementVariant::Html =>
                panic!("trying to get tag-name for an invalid ElementVariant"),
            ElementVariant::Button    => "button",
            ElementVariant::Div       => "div",
            ElementVariant::Paragraph => "p",
            ElementVariant::Style     => "style",
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Element {
    #[allow(unused)]
    variant: ElementVariant,
    inner: web_sys::Element,
    events: HashMap<&'static str, EventHandler>,
    classes: web_sys::DomTokenList,
}

impl ElementTree for Element {
    fn root(&self) -> &Element { self }
}

/// If the inner element is removed the wrapper `Element` can no longer be accessed safely.
/// So the removal of the inner Element should be tied to dropping the outer `Element`.
impl Drop for Element {
    fn drop(&mut self) {
        console_log!("dropping element");
        self.inner.remove();
    }
}

impl Element {
    fn create_internal(variant: ElementVariant, inner: web_sys::Element) -> HellResult<Self> {
        console_log!("create inner element '{variant:?}'");
        let classes = inner.class_list();

        Ok(Self {
            variant,
            inner,
            events: HashMap::default(),
            classes,
        })
    }

    pub fn create(ctx: &ViewCtx, variant: ElementVariant) -> HellResult<Self> {
        let name = variant.tag_name();
        let inner: web_sys::Element = ctx.document().create_element(name).to_web_hell_err().unwrap();
        Self::create_internal(variant, inner)
    }

    #[inline]
    pub fn create_button(ctx: &ViewCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Button)
    }

    #[inline]
    pub fn create_div(ctx: &ViewCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Div)
    }

    #[inline]
    pub fn create_paragraph(ctx: &ViewCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Paragraph)
    }

    #[inline]
    pub fn create_style(ctx: &ViewCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Style)
    }
}

impl TryFrom<web_sys::HtmlElement> for Element {
    type Error = HellError;
    fn try_from(value: web_sys::HtmlElement) -> HellResult<Self> {
        Self::create_internal(
            ElementVariant::Html,
            value.into(),
        )
    }
}

impl Element {
    #[inline]
    pub fn inner(&self) -> &web_sys::Element {
        &self.inner
    }

    pub fn append_child<E>(&mut self, tree: &E) -> HellResult<()>
    where E: ElementTree
    {
        let _ = self.inner.append_child(tree.root().inner()).to_web_hell_err()?;
        Ok(())
    }
}


fn js_array_from_str_slice(val: &[&str]) -> js_sys::Array {
    val.into_iter().fold(js_sys::Array::new_with_length(val.len() as u32), |init, n| {
        let js_val = wasm_bindgen::JsValue::from_str(n);
        let _ = init.push(&js_val);
        init
    })
}

/// Content handling
impl Element {
    pub fn inner_html(&self) -> String {
        self.inner().inner_html()
    }

    pub fn set_inner_html(&mut self, value: &str) {
        self.inner().set_inner_html(value);
    }

    pub fn text_content(&self) -> Option<String> {
        self.inner().text_content()
    }

    pub fn set_text_content(&mut self, value: Option<&str>) {
        self.inner().set_text_content(value);
    }
}

/// Class handling
impl Element {
    pub fn add_class(&mut self, name: &str) -> HellResult<()> {
        self.classes.add_1(name).to_web_hell_err()
    }

    pub fn add_classes(&mut self, names: &[&str]) -> HellResult<()> {
        let names = js_array_from_str_slice(names);
        self.classes.add(&names).to_web_hell_err()
    }

    pub fn remove_class(&mut self, name: &str) -> HellResult<()> {
        self.classes.remove_1(name).to_web_hell_err()
    }

    pub fn remove_classes(&mut self, names: &[&str]) -> HellResult<()> {
        let names = js_array_from_str_slice(names);
        self.classes.remove(&names).to_web_hell_err()
    }

    pub fn contains_class(&mut self, name: &str) -> bool {
        self.classes.contains(name)
    }
}

/// Event-Handling
impl Element {
    pub fn add_event_listener<F>(&mut self, event_type: &'static str, listener: F) -> HellResult<()>
    where F: FnMut() + 'static
    {
        let listener = EventHandler::new(self, event_type, listener)?;
        let old_value = self.events.insert(event_type, listener);
        assert!(old_value.is_none());

        Ok(())
    }

    pub fn remove_event_listener(&mut self, event_type: &'static str) -> HellResult<()> {
        let listener = self.events.remove(event_type).expect("expected handler in event-map");
        self
            .inner()
            .remove_event_listener_with_callback(event_type, &listener.closure_function())
            .to_web_hell_err()
    }
}

// ----------------------------------------------------------------------------
