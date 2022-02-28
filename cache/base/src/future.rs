use std::future::Future;
use std::pin::Pin;

/// Future for retrieving an entity from a repository.
pub type GetEntityFuture<'a, T, E> =
    Pin<Box<dyn Future<Output = Result<Option<T>, E>> + Send + 'a>>;
