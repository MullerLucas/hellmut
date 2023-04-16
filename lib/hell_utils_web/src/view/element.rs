use hell_core::error::HellResult;
use crate::{console_debug, view::EventHandler};
use crate::error::ErrToWebHellErr;

use super::{Context, ElementTree};

// ----------------------------------------------------------------------------

macro_rules! declare_create_methods {
    ($($fn_name:ident : $enum_name:ident),* $(,)?) => {
        paste::paste! {
            impl Element {
                $(
                    #[inline]
                    pub fn [< create_ $fn_name >] (cx: Context) -> HellResult<CreateResult> {
                        Self::create(cx, ElementVariant::$enum_name)
                    }
                )*
            }
        }
    };
}


// ----------------------------------------------------------------------------

type CreateResult = (Element, ElementHandle);

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(pub usize);

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct ElementHandle {
    cx: Context,
    id: ElementId,
}

impl ElementHandle {
    pub fn new(cx: Context, id: ElementId) -> Self {
        Self { cx, id }
    }

    pub fn get(&self) -> Element {
        self.cx.get_element(self.id)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum ElementVariant {
    Invalid,
    Html,

    Button,
    Div,
    Paragraph,
    Style,
    Span,
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
            ElementVariant::Span      => "span",
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Element {
    handle: ElementHandle,
    #[allow(unused)]
    variant: ElementVariant,
    sys: web_sys::Element,
}

impl ElementTree for Element {
    fn root(&self) -> &Element { self }
}

/// If the inner element is removed the wrapper `Element` can no longer be accessed safely.
/// So the removal of the inner Element should be tied to dropping the outer `Element`.
// impl Drop for Element {
//     fn drop(&mut self) {
//         console_debug!("dropping element");
//         self.inner.remove();
//     }
// }

impl Element {
    fn create_element_internal(cx: Context, variant: ElementVariant, sys: web_sys::Element) -> HellResult<ElementHandle> {
        console_debug!("create inner element '{variant:?}'");

        let id = cx.create_next_element_id();
        let handle = ElementHandle::new(cx, id);
        let element = Self {
            handle,
            variant,
            sys,
        };
        let _ = cx.add_element(element);
        let handle = ElementHandle::new(cx, id);

        Ok(handle)
    }

    #[inline]
    fn create_internal(cx: Context, variant: ElementVariant, sys: web_sys::Element) -> HellResult<CreateResult> {
        let handle = Self::create_element_internal(cx, variant, sys)?;
        let element = handle.get();
        Ok((element, handle))
    }

    pub fn create(cx: Context, variant: ElementVariant) -> HellResult<CreateResult> {
        let name = variant.tag_name();
        let inner: web_sys::Element = cx.document().create_element(name).to_web_hell_err().unwrap();
        Self::create_internal(cx, variant, inner)
    }

    pub fn try_from_html(cx: Context, value: web_sys::HtmlElement) -> HellResult<CreateResult> {
        Self::create_internal(
            cx,
            ElementVariant::Html,
            value.into(),
        )
    }
}

declare_create_methods! {
    button: Button,
    div: Div,
    paragraph: Paragraph,
    style: Style,
    span: Span,
}

impl Element {
    pub fn handle(&self) -> ElementHandle {
        self.handle
    }

    #[inline]
    pub fn sys(&self) -> &web_sys::Element {
        &self.sys
    }

    pub fn append_child<E>(&mut self, tree: &E) -> HellResult<()>
    where E: ElementTree
    {
        let _ = self.sys.append_child(tree.root().sys()).to_web_hell_err()?;
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
        self.sys().inner_html()
    }

    pub fn set_inner_html(&mut self, value: &str) {
        self.sys().set_inner_html(value);
    }

    pub fn text_content(&self) -> Option<String> {
        self.sys().text_content()
    }

    pub fn set_text_content(&mut self, value: Option<&str>) {
        self.sys().set_text_content(value);
    }
}

// Class handling
impl Element {
    pub fn add_class(&mut self, name: &str) -> HellResult<()> {
        let classes = self.sys.class_list();
        classes.add_1(name).to_web_hell_err()
    }

    pub fn add_classes(&mut self, names: &[&str]) -> HellResult<()> {
        let classes = self.sys.class_list();
        let names = js_array_from_str_slice(names);
        classes.add(&names).to_web_hell_err()
    }

    pub fn remove_class(&mut self, name: &str) -> HellResult<()> {
        let classes = self.sys.class_list();
        classes.remove_1(name).to_web_hell_err()
    }

    pub fn remove_classes(&mut self, names: &[&str]) -> HellResult<()> {
        let classes = self.sys.class_list();
        let names = js_array_from_str_slice(names);
        classes.remove(&names).to_web_hell_err()
    }

    pub fn contains_class(&mut self, name: &str) -> bool {
        let classes = self.sys.class_list();
        classes.contains(name)
    }
}


// Event-Handling
impl Element {
    pub fn add_event_listener<F>(&self, event_type: &'static str, listener: F) -> HellResult<()>
    where F: FnMut() + 'static
    {
        let event_handler = EventHandler::new(self.sys(), event_type, listener)?;
        self.handle.cx.add_event_handler(self.handle.id, event_type, event_handler);

        Ok(())
    }

    // pub fn remove_event_listener(&mut self, event_type: &'static str) -> HellResult<()> {
    //     let listener = self.events.remove(event_type).expect("expected handler in event-map");
    //     self
    //         .inner()
    //         .remove_event_listener_with_callback(event_type, &listener.closure_function())
    //         .to_web_hell_err()
    // }
}

// ----------------------------------------------------------------------------

// ----------------------------------------------------------------------------
