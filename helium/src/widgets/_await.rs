use std::future::Future;

use super::Widget;

/// Loads data in the background
pub struct Await<F, P, C> {
    future: F,
    pending: P,
    complete: C,
}

impl<F, P, C> Await<F, P, C>
where
    F: Future,
{
    pub fn new() -> Self {
        todo!()
    }

    pub async fn poll(&mut self) {}
}

impl<F, P, C> Widget for Await<F, P, C>
where
    P: Widget,
    C: Widget,
{
    fn id(&self) -> &str {
        // TODO get the child's id
        ""
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {}

    fn draw(&self, layout: &dyn Layout, renderer: &mut Renderer) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stream() {
        let mut image = Await {
            future: async {
                let url =
                    "https://m.media-amazon.com/images/I/81qJ1ui8bzL._AC_UF1000,1000_QL80_.jpg";
                let response = reqwest::get(url).await.unwrap();
            },
            pending: 0,
            complete: 0,
        };
        image.poll();

        dbg!(&response);
    }
}
