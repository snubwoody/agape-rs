use std::any::Any;

/// Emitted when the left mouse button is pressed.
pub struct MouseButtonDown;

/// Emitted when the left mouse button is released.
pub struct MouseButtonUp;

// Marker trait
/// The message trait is implemented for anything which implements
/// `Any`.
pub trait Message: Any {}

impl<T: Any> Message for T {}

#[derive(Default)]
pub struct MessageQueue {
    items: Vec<Box<dyn Message>>,
    frame_count: u32,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment the frame count.
    pub(crate) fn tick(&mut self) {
        self.frame_count += 1;
    }

    /// Returns true if a message of type `M` is in the queue.
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
        let item: Box<dyn Any> = self.items.swap_remove(index);
        item.downcast().ok().map(|m| *m)
    }

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
        // TODO: 2 frames might be better
        if self.frame_count >= 1 {
            self.items.clear();
            self.frame_count = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_queue_tick() {
        let mut messages = MessageQueue::new();
        messages.tick();
        messages.tick();
        assert_eq!(messages.frame_count, 2);
    }

    #[test]
    fn clear_messages() {
        let mut messages = MessageQueue::new();
        messages.add(String::new());
        messages.tick();
        messages.clear();
        assert!(messages.is_empty());
    }
}
