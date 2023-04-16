use hell_core::error::HellResult;
use wasm_bindgen::JsCast;
use crate::view::EventHandler;
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

fn create_with<E, F>(cx: Context, f: F) -> HellResult<(E, ElementHandle)>
where E: Into<Element> + Clone,
      F: Fn(ElementHandle) -> HellResult<E>
{
    let id = cx.create_next_element_id();
    let handle = ElementHandle::new(cx, id);

    let element = f(handle)?;

    let _ = cx.add_element(element.clone());
    // let element = cx.get_element(id);
    Ok((element, handle))
}

// ----------------------------------------------------------------------------

fn js_array_from_str_slice(val: &[&str]) -> js_sys::Array {
    val.iter().fold(js_sys::Array::new_with_length(val.len() as u32), |init, n| {
        let js_val = wasm_bindgen::JsValue::from_str(n);
        let _ = init.push(&js_val);
        init
    })
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

    pub fn get<E>(&self) -> E
    where E: From<Element>
    {
        self.cx.get_element(self.id)
    }

    #[inline]
    pub fn get_element(&self) -> Element {
        self.get()
    }

    #[inline]
    pub fn get_html(&self) -> HtmlElement {
        self.get()
    }

    #[inline]
    pub fn get_input(&self) -> HtmlInputElement {
        self.get()
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
    Input,
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
            ElementVariant::Span      => "span",
            ElementVariant::Style     => "style",
            ElementVariant::Input     => "input",
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Element {
    handle: ElementHandle,
    js_element: web_sys::Element,
}

impl Element {
    pub fn value(&self) -> String {
        "test".to_owned()
    }
    pub fn new(cx: Context, handle: ElementHandle, variant: ElementVariant) -> HellResult<Self> {
        let name = variant.tag_name();
        let js_element: web_sys::Element = cx.document().create_element(name).to_web_hell_err()?;

        Ok(Self {
            handle,
            js_element,
        })
    }

    pub fn create(cx: Context, variant: ElementVariant) -> HellResult<CreateResult> {
        let id = cx.create_next_element_id();
        let handle = ElementHandle::new(cx, id);

        let element = Element::new(cx, handle, variant)?;

        let _ = cx.add_element(element.clone());
        Ok((element, handle))
    }

    pub fn as_input_element(&self) -> HellResult<web_sys::HtmlInputElement> {
        Ok(self.js_element().clone().dyn_into::<web_sys::HtmlInputElement>().unwrap())
    }

    pub fn create_html(cx: Context) -> HellResult<(HtmlElement, ElementHandle)> {
        create_with(cx, |handle| {
            HtmlElement::new(cx, handle)
        })
    }

    pub fn create_body(cx: Context) -> HellResult<(HtmlElement, ElementHandle)> {
        create_with(cx, |handle| {
            Ok(HtmlElement {
                handle,
                js_element: cx.document().body().expect("expected there to be a body")
            })
        })
    }

    pub fn create_input(cx: Context) -> HellResult<(HtmlInputElement, ElementHandle)> {
        create_with(cx, |handle| {
            HtmlInputElement::new(cx, handle)
        })
    }
}



declare_create_methods! {
    button: Button,
    div: Div,
    paragraph: Paragraph,
    span: Span,
    style: Style,
}

impl ElementContainer for Element {
    fn handle(&self) -> ElementHandle {
        self.handle
    }

    fn js_element(&self) -> &web_sys::Element {
        self.js_element.as_ref()
    }
}

impl<E> ElementTree for E where E: ElementContainer + 'static {
    fn root(&self) -> &web_sys::Element  {
        self.js_element()
    }
}


pub trait ElementContainer: Clone {
    fn handle(&self) -> ElementHandle;
    fn js_element(&self) -> &web_sys::Element;

    fn append_child<E>(&mut self, tree: &E) -> HellResult<()>
    where E: ElementTree
    {
        let _ = self.js_element().append_child(tree.root()).to_web_hell_err()?;
        Ok(())
    }

    // content operations
    // ------------------

    #[inline]
    fn inner_html(&self) -> String {
        self.js_element().inner_html()
    }

    #[inline]
    fn set_inner_html(&mut self, value: &str) {
        self.js_element().set_inner_html(value);
    }

    #[inline]
    fn text_content(&self) -> Option<String> {
        self.js_element().text_content()
    }

    #[inline]
    fn set_text_content(&mut self, value: Option<&str>) {
        self.js_element().set_text_content(value);
    }

    // class operations
    // ----------------
    fn add_class(&mut self, name: &str) -> HellResult<()> {
        let classes = self.js_element().class_list();
        classes.add_1(name).to_web_hell_err()
    }

    fn add_classes(&mut self, names: &[&str]) -> HellResult<()> {
        let classes = self.js_element().class_list();
        let names = js_array_from_str_slice(names);
        classes.add(&names).to_web_hell_err()
    }

    fn remove_class(&mut self, name: &str) -> HellResult<()> {
        let classes = self.js_element().class_list();
        classes.remove_1(name).to_web_hell_err()
    }

    fn remove_classes(&mut self, names: &[&str]) -> HellResult<()> {
        let classes = self.js_element().class_list();
        let names = js_array_from_str_slice(names);
        classes.remove(&names).to_web_hell_err()
    }

    fn contains_class(&mut self, name: &str) -> bool {
        let classes = self.js_element().class_list();
        classes.contains(name)
    }

    // attribute operations
    // --------------------
    #[inline]
    fn set_attribute(&mut self, name: &str, value: &str) -> HellResult<()> {
        self.js_element().set_attribute(name, value).to_web_hell_err()
    }

    #[inline]
    fn get_attribute(&mut self, name: &str) -> Option<String> {
        self.js_element().get_attribute(name)
    }

    #[inline]
    fn has_attribute(&mut self, name: &str) -> bool {
        self.js_element().has_attribute(name)
    }

    // event operations
    // ----------------
    fn add_event_listener<F>(&self, event_type: &'static str, listener: F) -> HellResult<()>
where F: FnMut() + 'static
    {
        let handle = self.handle();
        let event_handler = EventHandler::new(self.js_element(), event_type, listener)?;
        handle.cx.add_event_handler(handle.id, event_type, event_handler);

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

#[derive(Clone)]
pub struct HtmlElement {
    handle: ElementHandle,
    js_element: web_sys::HtmlElement,
}

impl From<Element> for HtmlElement {
    fn from(element: Element) -> Self {
        Self {
            handle: element.handle(),
            js_element: element.js_element.dyn_into().unwrap(),
        }
    }
}

impl From<HtmlElement> for Element {
    fn from(value: HtmlElement) -> Self {
        Self {
            handle: value.handle,
            js_element: value.js_element.dyn_into().unwrap(),
        }
    }
}

impl HtmlElement {
    pub fn new(cx: Context, handle: ElementHandle) -> HellResult<Self> {
        let variant = ElementVariant::Input;
        let js_element: web_sys::HtmlElement = cx.document()
            .create_element(variant.tag_name())
            .unwrap()
            .dyn_into()
            .unwrap();

        Ok(Self {
            handle,
            js_element,
        })
    }
}

impl ElementContainer for HtmlElement {
    fn handle(&self) -> ElementHandle {
        self.handle
    }

    fn js_element(&self) -> &web_sys::Element {
        &self.js_element
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct HtmlInputElement {
    handle: ElementHandle,
    js_element: web_sys::HtmlInputElement,
}

impl From<Element> for HtmlInputElement {
    fn from(element: Element) -> Self {
        Self {
            handle: element.handle(),
            js_element: element.js_element.dyn_into().unwrap(),
        }
    }
}

impl From<HtmlInputElement> for Element {
    fn from(value: HtmlInputElement) -> Self {
        Self {
            handle: value.handle,
            js_element: value.js_element.dyn_into().unwrap(),
        }
    }
}

impl ElementContainer for HtmlInputElement {
    fn handle(&self) -> ElementHandle {
        self.handle
    }

    fn js_element(&self) -> &web_sys::Element {
        &self.js_element
    }
}

impl HtmlInputElement {
    pub fn new(cx: Context, handle: ElementHandle) -> HellResult<Self> {
        let variant = ElementVariant::Input;
        let js_element: web_sys::HtmlInputElement = cx.document()
            .create_element(variant.tag_name())
            .unwrap()
            .dyn_into()
            .unwrap();

        Ok(Self {
            handle,
            js_element,
        })
    }

    pub fn value(&self) -> String {
        self.js_element.value()
    }
}

// ----------------------------------------------------------------------------
