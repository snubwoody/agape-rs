use std::future::Future;
use helium_renderer::Renderer;
use crystal::Layout;
use tokio::sync::mpsc::{self, Receiver};
use super::Widget;

/// Loads data in the background
pub struct Await<P, C> {
	/// The widget that is displayed while the future is not
	/// ready
    pending: P,
	/// The [`Widget`] that is displayed when the future is complete
    complete: Option<C>,
	/// The receiver to poll every frame
	rx:Receiver<C>
}

impl<P, C> Await<P, C>
where
	C: Widget + Send + 'static
{
    pub fn new<F>(future:F,pending:P) -> Self 
	where F: Future<Output = C> + Send + 'static
	{
		let (tx,rx) = mpsc::channel(1);		
		
		tokio::spawn(async move{
			let data = future.await;
			let res = tx.send(data).await;
			if res.is_err(){
				log::warn!("Error loading data: {:?}",res)
			}
			else {
				log::trace!("Loaded data in Await widget")
			}
		});

        Self{
			pending,
			complete:None,
			rx
		}
    }

    pub fn poll(&mut self) {
		match self.rx.try_recv(){
			Ok(widget) => {
				self.complete = Some(widget);
			},
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
		if let Some(complete) = &self.complete  {
			return complete.id();
		}
		self.pending.id()
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
		if let Some(complete) = &self.complete  {
			return complete.layout(renderer);
		}
		self.pending.layout(renderer)
	}

    fn draw(&self, layout: &dyn Layout, renderer: &mut Renderer) {
		if let Some(complete) = &self.complete  {
			return complete.draw(layout, renderer);
		}
		self.pending.draw(layout, renderer);
	}
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::widgets::Text;
    use super::*;

    #[tokio::test]
    async fn stream() {
		let future = async move {
			let url =
			"https://m.media-amazon.com/images/I/81qJ1ui8bzL._AC_UF1000,1000_QL80_.jpg";
			let response = reqwest::get(url).await.unwrap();
			response.bytes().await.unwrap();
			return Text::new("Hi");
		};
		
		let mut image = Await::new(future, Text::new("Loading"));

		for _ in 0..10{
			image.poll();
			tokio::time::sleep(Duration::from_millis(200)).await;
		}
    }
}
