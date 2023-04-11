use std::{cell::RefCell, any::Any, marker::PhantomData};
use crate::console_log;


pub type InnerSignal = Box<dyn Any>;

#[derive(Debug)]
pub struct InnerRuntime {
    window: web_sys::Window,
    document: web_sys::Document,

    signals: RefCell<Vec<InnerSignal>>
}

impl InnerRuntime {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        let signals = RefCell::new(Vec::new());

        Self {
            window,
            document,

            signals,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Runtime {
    inner: &'static InnerRuntime,
}

impl Runtime {
    pub fn new() -> Self {
        let inner = Box::leak(Box::new(InnerRuntime::new()));
        Self { inner }
    }

    pub fn window(&self)       -> &web_sys::Window      { &self.inner.window }
    pub fn document(&self)     -> &web_sys::Document    { &self.inner.document }

    pub fn signals(&self) -> &RefCell<Vec<InnerSignal>> {
        &self.inner.signals
    }
}

// ----------------------------------------------------------------------------

impl Runtime {
    pub fn create_signal<T>(&self, val: T) -> Signal<T>
    where T: Clone +'static
    {
        console_log!("create signal");

        let mut signals = self.inner.signals.borrow_mut();
        signals.push(Box::new(val));
        let id = SignalId(signals.len() - 1);

        Signal {
            cx: *self,
            id,
            _t: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct EffectId(usize);
#[derive(Debug, Clone, Copy)]
pub struct SignalId(usize);



#[derive(Debug, Clone, Copy)]
pub struct Signal<T> {
    cx: Runtime,
    id: SignalId,
    _t: PhantomData<T>
}

impl<T> Signal<T> {
    pub fn get(&self) -> T
    where T: Clone + 'static
    {
        self.cx.signals()
            .borrow()[self.id.0]
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }

    pub fn set(&self, val: T)
    where T: 'static
    {
        let wrapper = &mut self.cx.signals().borrow_mut()[self.id.0];
        let wrapper = wrapper.downcast_mut::<T>().unwrap();
        *wrapper= val;
    }
}


