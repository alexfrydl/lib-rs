use super::*;

/// An asynchronous task.
#[must_use = "A task is killed when its Handle is dropped."]
pub struct Handle<T, E> {
  task: async_executor::Task<Output<T, E>>,
}

impl<T, E> Handle<T, E>
where
  T: Send + 'static,
  E: Send + 'static,
{
  /// Kills the task and waits for its future to be dropped.
  pub async fn kill(self) {
    self.task.cancel().await;
  }

  /// Starts a new task that runs a continuation function after this task exits.
  pub fn continue_with<U, F, C>(
    self,
    continuation: impl FnOnce(Output<T, E>) -> C + Send + 'static,
  ) -> Handle<U, F>
  where
    C: Future<Output = Result<U, F>> + Send + 'static,
    U: Send + 'static,
    F: Send + 'static,
  {
    start(async {
      let output = self.await;

      continuation(output).await
    })
  }
}

// Implement From for Handle to convert from async_executor tasks.

impl<T, E> From<async_executor::Task<Output<T, E>>> for Handle<T, E> {
  fn from(task: async_executor::Task<Output<T, E>>) -> Self {
    Self { task }
  }
}

// Implement Future for Handle to poll the underlying task.

impl<T, E> Future for Handle<T, E> {
  type Output = Output<T, E>;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context<'_>) -> future::Poll<Self::Output> {
    let task = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.task) };

    task.poll(cx)
  }
}
