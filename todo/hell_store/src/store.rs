use std::cell::RefCell;
use std::sync::Arc;

use hell_error::HellResult;
use tokio::sync::mpsc;


pub type HellStoreSender<M> = mpsc::Sender<M>;
pub type HellStoreReceiver<M> = mpsc::Receiver<M>;

pub struct HellStore<D, M>
where D: Clone
{
    version: u64,
    rx: HellStoreReceiver<M>,
    data: D,
}

impl<D, M> HellStore<D, M>
where D: Clone
{
    pub fn new(data: D) -> HellStoreHandle<D, M> {
        let (tx, rx) = mpsc::channel::<M>(100);

        let store = Self {
            version: 0,
            rx,
            data,
        };

        let handle = HellStoreHandle {
            store: Arc::new(RefCell::new(store)),
            tx,
        };

        handle
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}

impl<D, M> HellStore<D, M>
where D: Clone,
{
    pub async fn run(&mut self) -> HellResult<()> {
        println!("start running...");

        while let Some(_msg) = self.rx.recv().await {
            todo!();
        }

        println!("stop running...");

        Ok(())
    }
}

// ----------------------------------------------------------------------------

pub struct HellStoreHandle<D: 'static, M>
where D: Clone
{
    store: Arc<RefCell<HellStore<D, M>>>,
    tx: HellStoreSender<M>,
}

impl<D, M> Clone for HellStoreHandle<D, M>
where D: Clone
{
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            tx: self.tx.clone(),
        }
    }
}

impl<D, M> HellStoreHandle<D, M>
where D: Clone
{
    pub async fn run(self) -> HellResult<()> {
        let mut store = self.store.borrow_mut();
        store.run().await
    }

    pub fn data(&self) -> D {
        self.store.borrow().data.clone()
    }
}
