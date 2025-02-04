use super::Widget;
use crystal::Layout;
use helium_renderer::Renderer;
use std::future::Future;
use tokio::sync::mpsc::{self, Receiver};

/// A [`Widget`] that runs a [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) in
/// the background allowing you to display a `Widget` while the future is still pending. When the
/// future has completed the completed `Widget` will be displayed instead.
///
/// # Example
///
/// ```
/// use helium::widgets::{Await,Text};
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main(){
/// 	let future = async move{
/// 		tokio::time::sleep(Duration::from_millis(50)).await;
/// 		// Futures must return a widget
/// 		return Text::new("Loaded data!");
/// 	};
///
/// 	let _await = Await::new(future,Text::new("Loading data"));
/// }
/// ```
pub struct Await<P, C> {
    /// The widget that is displayed while the future is not
    /// ready
    pending: P,
    /// The [`Widget`] that is displayed when the future is complete
    complete: Option<C>,
    /// The receiver to poll every frame
    rx: Receiver<C>,
}

impl<P, C> Await<P, C>
where
    C: Widget + Send + 'static,
{
    /// Receives a [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) that is
    /// run in the background, allowing you to display a `pending` [`Widget`] while the future
    /// loads. The future must return a `Widget` which will be displayed once the future is
    /// complete.
    /// The provided future will start running immediately, sometimes you might only see the
    /// completed state, if it completes quick enough.
    ///
    /// # Example
    ///
    /// ```
    /// use helium::widgets::{Await,Text};
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main(){
    /// 	let future = async move{
    /// 		tokio::time::sleep(Duration::from_millis(50)).await;
    /// 		// Futures must return a widget
    /// 		return Text::new("Loaded data!");
    /// 	};
    ///
    /// 	let _await = Await::new(future,Text::new("Loading data"));
    /// }
    /// ```
    pub fn new<F>(future: F, pending: P) -> Self
    where
        F: Future<Output = C> + Send + 'static,
    {
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(async move {
            let data = future.await;
            let res = tx.send(data).await;
            if res.is_err() {
                log::warn!("Error loading data: {:?}", res)
            } else {
                log::trace!("Loaded data in Await widget")
            }
        });

        Self {
            pending,
            complete: None,
            rx,
        }
    }

    /// Continuously check if the receiver has new messages
    pub fn poll(&mut self) {
        match self.rx.try_recv() {
            Ok(widget) => {
                self.complete = Some(widget);
            }
            Err(_) => {}
        }
    }
}

impl<P, C> Widget for Await<P, C>
where
    P: Widget,
    C: Widget + Send + 'static,
{
    fn tick(&mut self) {
        self.poll()
    }

    fn id(&self) -> &str {
        // This is essentially a `Phantom` widget we only return the
        // child's data.
        if let Some(complete) = &self.complete {
            return complete.id();
        }
        self.pending.id()
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        if let Some(complete) = &self.complete {
            return complete.layout(renderer);
        }
        self.pending.layout(renderer)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut Renderer) {
        if let Some(complete) = &self.complete {
            return complete.draw(layout, renderer);
        }
        self.pending.draw(layout, renderer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Text;
    use std::time::Duration;

    #[tokio::test]
    async fn await_works() {
        todo!()
    }
}
