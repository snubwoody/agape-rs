use std::any::Any;
use std::fmt::Debug;

/// Emitted when the left mouse button is pressed.
#[derive(Debug)]
pub struct MouseButtonDown;

/// Emitted when the left mouse button is released.
#[derive(Debug)]
pub struct MouseButtonUp;

// Marker trait
/// The message trait is implemented for anything which implements
/// `Any`.
pub trait Message: Any + Debug {}
impl<T: Any + Debug> Message for T {}

#[derive(Debug)]
struct MessageNode {
    frame_delta: u32,
    inner: Box<dyn Message>,
}

#[derive(Default, Debug)]
pub struct MessageQueue {
    items: Vec<Box<dyn Message>>,
}

impl MessageQueue {
    /// Create a new message queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment the frame count.
    pub(crate) fn tick(&mut self) {
        // self.items.iter_mut().for_each(|node| node.frame_delta += 1);
    }

    /// Returns `true` if a message of type `M` is in the queue.
    ///
    /// # Example
    /// ```
    /// use agape::MessageQueue;
    ///
    /// let mut messages = MessageQueue::new();
    /// messages.add(String::new());
    ///
    /// assert!(messages.has::<String>());
    /// ```
    pub fn has<M: Message>(&self) -> bool {
        self.get::<M>().is_some()
    }

    pub fn add<M: Message>(&mut self, item: M) {
        // Don't insert the same resource twice
        if self.get::<M>().is_none() {
            self.items.push(Box::new(item));
        }
    }

    pub fn set<M: Message>(&mut self, item: M) {
        self.remove::<M>();
        self.items.push(Box::new(item));
    }

    /// Remove and return a message of type `M` from the queue.
    pub fn remove<M: 'static>(&mut self) -> Option<M> {
        let index = self
            .items
            .iter()
            .map(|i| i.as_ref() as &dyn Any)
            .position(|i| i.is::<M>())?;
        let item = self.items.swap_remove(index);
        (item as Box<dyn Any>).downcast().ok().map(|m| *m)
    }

    /// Remove the message at the index.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    pub fn remove_index(&mut self, index: usize) {
        self.items.swap_remove(index);
    }

    /// Get a message from the message queue.
    ///
    /// ```
    /// use agape::MessageQueue;
    ///
    /// #[derive(Debug,Copy, Clone)]
    /// struct DummyMessage;
    ///
    /// let mut message_queue = MessageQueue::new();
    /// message_queue.add(DummyMessage);
    /// assert!(message_queue.get::<DummyMessage>().is_some())
    /// ```
    pub fn get<T: 'static>(&self) -> Option<&T> {
        for item in &self.items {
            let item = item.as_ref() as &dyn Any;
            match item.downcast_ref::<T>() {
                Some(item) => return Some(item),
                None => continue,
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_queue_tick() {
        let mut messages = MessageQueue::new();
        messages.add(MouseButtonDown);
        messages.tick();
        messages.tick();
        // assert_eq!(messages.items[0].frame_delta, 2);
    }

    #[test]
    fn clear_messages() {
        let mut messages = MessageQueue::new();
        messages.add(String::new());
        messages.tick();
        messages.tick();
        messages.tick();
        messages.clear();
        assert!(messages.is_empty());
    }
}
