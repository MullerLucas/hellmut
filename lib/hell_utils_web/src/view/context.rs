use std::{cell::{RefCell, Cell}, any::Any, marker::PhantomData, collections::{HashMap, HashSet}, fmt::Display};
use crate::console_debug;

use super::{EventHandler, EventHandlerId, ElementId, Element};


pub type InnerSignalValue = Box<dyn Any>;
pub type InnerEffect      = Box<(dyn Fn() + 'static)>;

// TODO (lm): look up slot-maps
pub struct InnerContext {
    window: web_sys::Window,
    document: web_sys::Document,

    signal_values: RefCell<Vec<InnerSignalValue>>,
    signal_subscribers: RefCell<HashMap<SignalId, HashSet<EffectId>>>,

    effects: RefCell<Vec<InnerEffect>>,
    running_effect: Cell<Option<EffectId>>,

    elements: RefCell<Vec<Element>>,
    element_event_handlers: RefCell<HashMap<ElementId, HashMap<&'static str, EventHandler>>>,
}

impl InnerContext {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        let signals = RefCell::new(Vec::new());
        let signal_subscribers = RefCell::new(HashMap::new());
        let effects = RefCell::new(Vec::new());
        let running_effect = Cell::new(None);
        let elements = RefCell::new(Vec::new());
        let element_event_handlers = RefCell::new(HashMap::new());

        Self {
            window,
            document,

            signal_values: signals,
            signal_subscribers,
            effects,
            running_effect,
            elements,
            element_event_handlers,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Context {
    inner: &'static InnerContext,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runtime").finish()
    }
}

impl Context {
    pub fn new() -> Self {
        let inner = Box::leak(Box::new(InnerContext::new()));
        Self { inner }
    }

    #[inline]
    pub fn window(&self)       -> &web_sys::Window {
        &self.inner.window
    }

    #[inline]
    pub fn document(&self) -> &web_sys::Document {
        &self.inner.document
    }

    #[inline]
    fn signal_values(&self) -> &RefCell<Vec<InnerSignalValue>> {
        &self.inner.signal_values
    }

    #[inline]
    fn effects(&self) -> &RefCell<Vec<InnerEffect>> {
        &self.inner.effects
    }

    #[inline]
    pub fn elements(&self) -> &RefCell<Vec<Element>> {
        &self.inner.elements
    }
}

impl Context {
    pub fn create_signal<T>(&self, val: T) -> Signal<T>
    where T: Clone +'static
    {
        console_debug!("[SIGNAL]: start creating ...");

        let mut signals = self.inner.signal_values.borrow_mut();
        signals.push(Box::new(val));
        let signal_id = SignalId(signals.len() - 1);

        console_debug!("[SIGNAL]: created {}", signal_id.0);

        Signal {
            cx: *self,
            id: signal_id,
            _t: PhantomData,
        }
    }
}

impl Context {
    pub fn create_effect(&self, effect: impl Fn() + 'static) -> EffectId {
        console_debug!("[EFFECT]: start creating");

        let effect_id = {
            let mut effects = self.inner.effects.borrow_mut();
            effects.push(Box::new(effect));
            EffectId(effects.len() - 1)
        };

        self.run_effect(effect_id);

        console_debug!("[EFFECT]: created: '{}'", effect_id);

        effect_id
    }

    fn run_effect(&self, effect_id: EffectId) {
        let prev_effect = self.inner.running_effect.take();
        self.inner.running_effect.set(Some(effect_id));

        console_debug!("[EFFECT] run: '{}'", effect_id);
        let effect = &self.effects().borrow()[effect_id.0];
        effect();

        self.inner.running_effect.set(prev_effect);
    }
}

impl Context {
    pub fn create_next_element_id(&self) -> ElementId {
        ElementId(self.inner.elements.borrow().len())
    }

    pub fn add_element(&self, element: impl Into<Element>) -> ElementId {
        let mut elements = self.inner.elements.borrow_mut();
        elements.push(element.into());
        ElementId(elements.len() - 1)
    }

    pub fn get_element<E>(&self, id: ElementId) -> E
    where E: From<Element>
    {
        E::from(
            self.inner.elements.borrow()[id.0].clone()
        )
    }

    pub fn add_event_handler(&self, element_id: ElementId, event_type: &'static str, event_handler: EventHandler) -> EventHandlerId {
        let mut event_handlers = self.inner.element_event_handlers.borrow_mut();
        let events = event_handlers.entry(element_id).or_insert(HashMap::new());
        events.insert(event_type, event_handler);
        EventHandlerId::new(event_handlers.len() - 1)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectId(usize);

impl Display for EffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(usize);

impl Display for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ----------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Signal<T> {
    cx: Context,
    id: SignalId,
    _t: PhantomData<T>
}

impl<T> Signal<T> {
    pub fn get(&self) -> T
    where T: Clone + 'static
    {
        console_debug!("[SIGNAL]: get: '{}'", self.id);

        // retrieve value
        let value = self.cx.signal_values()
            .borrow()[self.id.0]
            .downcast_ref::<T>()
            .unwrap()
            .clone();

        // run effect initially
        if let Some(running_effect_id) = self.cx.inner.running_effect.get() {
            let mut subs = self.cx.inner.signal_subscribers.borrow_mut();
            let subs = subs.entry(self.id).or_default();
            let _newly_inserted = subs.insert(running_effect_id);
            // assert!(newly_inserted);
        }

        value
    }

    pub fn set(&self, val: T)
    where T: 'static
    {
        console_debug!("[SIGNAL]: set: '{}'", self.id);

        {
            let wrapper = &mut self.cx.signal_values().borrow_mut()[self.id.0];
            let wrapper = wrapper.downcast_mut::<T>().unwrap();
            *wrapper = val;
        }

        let subs = {
            let subs = self.cx.inner.signal_subscribers.borrow();
            subs.get(&self.id).cloned()
        };

        if let Some(subs) = subs {
            for s in subs {
                self.cx.run_effect(s);
            }
        }
    }
}
