use std::sync::Arc;
use std::vec::IntoIter;
use futures::future::join_all;
use tokio::task::{JoinError, JoinHandle};
use std::fmt::{Debug, Formatter};

/*
    Callback takes T, returns a future R
    Callback must be thread safe.
 */
type Callback<T, R = ()> = dyn Fn(Arc<T>)
    -> JoinHandle<R> + Send + Sync + 'static;

/*
    Event has a Vector of sharable Callbacks.
    Each callback is expected to return type R, but internally a future.
 */
pub struct Event<T, R = ()> {
    pub callbacks: Vec<Arc<Callback<T, R>>>,
}

impl<T: Send + Sync + 'static, R: Send + 'static> Event<T, R> {
    pub fn new() -> Self {
        Event {
            callbacks: Vec::new(),
        }
    }

    // Connects a function to the Event
    pub fn connect<F>(&mut self, callback: F) -> usize
    where
            F: Fn(Arc<T>) -> R + Send + Sync + 'static,
    {
        let arc_cb = Arc::new(callback);

        let cb: Arc<Callback<T, R>> = Arc::new(move |data: Arc<T>| {
            let c = Arc::clone(&arc_cb);
            tokio::spawn(async move {
                (c)(data)
            })
        });

        self.callbacks.push(cb);
        self.callbacks.len() - 1
    }

    // Disconnects a function from the event.
    pub fn disconnect(&mut self, id: usize) -> Option<Arc<Callback<T, R>>> {
        Some(self.callbacks.remove(id))
    }

    // Submits a task to every callback.
    pub async fn fire(&self, data: T) -> Vec<Result<R, JoinError>> {
        let arc_data = Arc::new(data);
        let handles = self.callbacks
            .iter()
            .map(|cb|
                cb(Arc::clone(&arc_data))
            ).collect::<Vec<_>>();

        join_all(handles).await
    }
}

// Additional Implementations
// --------------------------------------- //

impl<T, R> Debug for Event<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("connections", &self.callbacks.len())
            .finish()
    }
}

impl<T, R> Clone for Event<T, R> {
    fn clone(&self) -> Self {
        Event {
            callbacks: self.callbacks.clone()
        }
    }
}

impl<T, R> IntoIterator for Event<T, R>
where
    T: Send + 'static,
    R: Send + 'static
{
    type Item = Arc<Callback<T, R>>;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.callbacks.into_iter()
    }
}

impl<T: Send + Sync + 'static, R: Send + 'static> Default for Event<T, R> {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------- //