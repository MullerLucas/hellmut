use hell_error::HellResult;
use crate::console_log;
use super::WebCtx;
use crate::error::ErrToWebHellErr;



#[derive(Debug)]
pub enum ElementVariant {
    Invalid,
    HtmlElement,
    Button,
    Div,
    Paragraph,
}

impl ElementVariant {
    pub fn tag_name(&self) -> &'static str {
        match self {
            ElementVariant::Invalid     |
            ElementVariant::HtmlElement =>
                panic!("trying to get tag-name for an invalid ElementVariant"),
            ElementVariant::Button    => "button",
            ElementVariant::Div       => "div",
            ElementVariant::Paragraph => "p",
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Element {
    variant: ElementVariant,
    inner: web_sys::Element,
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
    pub fn create(ctx: &WebCtx, variant: ElementVariant) -> HellResult<Self> {
        let name = variant.tag_name();
        let inner: web_sys::Element = ctx.document().create_element(name).to_web_hell_err().unwrap();
        console_log!("create inner element '{name}'");

        Ok(Self {
            variant,
            inner,
        })
    }

    #[inline]
    pub fn create_button(ctx: &WebCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Button)
    }

    #[inline]
    pub fn create_div(ctx: &WebCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Div)
    }

    #[inline]
    pub fn create_paragraph(ctx: &WebCtx) -> HellResult<Self> {
        Self::create(ctx, ElementVariant::Paragraph)
    }

}

impl From<web_sys::HtmlElement> for Element {
    fn from(value: web_sys::HtmlElement) -> Self {
        let inner: web_sys::Element = value.into();

        Self {
            variant: ElementVariant::HtmlElement,
            inner,
        }
    }
}

impl Element {
    pub fn inner(&self) -> &web_sys::Element {
        &self.inner
    }

    pub fn append_child(&mut self, child: &Self) -> HellResult<()> {
        let _ = self.inner.append_child(&child.inner).to_web_hell_err()?;
        Ok(())
    }
}
