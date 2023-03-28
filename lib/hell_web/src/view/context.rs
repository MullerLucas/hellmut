use std::rc::Rc;

#[derive(Debug)]
pub struct ViewCtxInner {
    window: web_sys::Window,
    document: web_sys::Document,
}

impl ViewCtxInner {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        Self {
            window,
            document,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ViewCtx {
    inner: Rc<ViewCtxInner>,
}

impl ViewCtx {
    pub fn new() -> Self {
        let inner = Rc::new(ViewCtxInner::new());
        Self { inner }
    }

    pub fn window(&self)       -> &web_sys::Window      { &self.inner.window }
    pub fn document(&self)     -> &web_sys::Document    { &self.inner.document }
}

