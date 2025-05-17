use crate::context::Context;
use crate::error::RinError;
use crate::response::Response;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

/// `HandlerFunc` 是一个类型别名，表示一个异步的、可发送且同步的闭包，
/// 它接受一个 `Context` 并返回一个 `Result<Response, RinError>`。
/// 这是一个方便的别名，用于 `Box<dyn ...>` 类型。
pub type HandlerFunc = Box<
    dyn Fn(Context) -> Pin<Box<dyn Future<Output = Result<Response, RinError>> + Send + 'static>>
        + Send
        + Sync
        + 'static,
>;

/// `Handler` Trait 定义了可以处理 HTTP 请求的实体。
/// 任何实现了此 Trait 的类型都可以作为请求处理器。
///
/// 异步特性通过 `async_trait` 宏实现。
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    /// 处理给定的 `Context`，并返回一个 `Result<Response, RinError>`。
    async fn handle(&self, ctx: Context) -> Result<Response, RinError>;
}

// 为满足 Handler Trait 要求的函数/闭包自动实现 Handler Trait
// 这样，用户可以直接提供 async fn 或 async move closure
#[async_trait]
impl<F, Fut> Handler for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response, RinError>> + Send + 'static,
{
    async fn handle(&self, ctx: Context) -> Result<Response, RinError> {
        self(ctx).await
    }
}
