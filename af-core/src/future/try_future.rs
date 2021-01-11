use super::*;

/// A [`Future`] that returns a result.
pub trait TryFuture<T, E>: Future<Output = Result<T, E>> {}

impl<F, T, E> TryFuture<T, E> for F where F: Future<Output = Result<T, E>> {}

/// Extension methods for futures that return a result.
pub trait TryFutureExt<T, E>: Sized {
  /// Returns a new future that converts the error by applying a function.
  fn map_err<F, M>(self, map: M) -> MapErr<Self, M>
  where
    M: FnOnce(E) -> F,
  {
    MapErr { future: self, map: Some(map) }
  }
}

impl<F, T, E> TryFutureExt<T, E> for F where F: TryFuture<T, E> {}

/// A future returned from [`TryFutureExt::map_err()`].
#[pin_project]
pub struct MapErr<F, M> {
  #[pin]
  future: F,
  map: Option<M>,
}

impl<T, I, O, F, M> Future for MapErr<F, M>
where
  F: Future<Output = Result<T, I>>,
  M: FnOnce(I) -> O,
{
  type Output = Result<T, O>;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context<'_>) -> future::Poll<Self::Output> {
    let this = self.project();

    match this.future.poll(cx) {
      future::Poll::Ready(res) => future::Poll::Ready(res.map_err(this.map.take().unwrap())),
      future::Poll::Pending => future::Poll::Pending,
    }
  }
}
