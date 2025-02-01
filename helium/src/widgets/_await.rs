use std::future::Future;
use helium_renderer::Renderer;
use crystal::Layout;
use tokio::sync::mpsc::{self, Receiver};
use super::Widget;

/// Loads data in the background
pub struct Await<P, C> {
	/// The computation to be run
    //future: Option<F>,
	/// The widget that is displayed while the future is not
	/// ready
    pending: P,
	/// The future that is displayed when the future is complete
    complete: Option<C>,
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
			tx.send(data).await.unwrap();
		});

        Self{
			//future:Some(future),
			pending,
			complete:None,
			rx
		}
    }

    pub fn poll(&mut self) {

		match self.rx.try_recv(){
			Ok(data) => {
				dbg!("Loaded data");
			},
			Err(_) => {
				dbg!("Still waiting");
			}
		}
	}
}

impl<P, C> Widget for Await<P, C>
where
    P: Widget,
    C: Widget,
{
    fn id(&self) -> &str {
        // TODO get the child's id
        self.pending.id()
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
		self.pending.layout(renderer)
	}

    fn draw(&self, layout: &dyn Layout, renderer: &mut Renderer) {
		self.pending.draw(layout, renderer);
	}
}

#[cfg(test)]
mod tests {
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

		image.poll();
		image.poll();
		image.poll();
		image.poll();
		image.poll();
		image.poll();
		image.poll();
		image.poll();
		image.poll();
		image.poll();

    }
}
