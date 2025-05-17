// rin-core/src/error.rs

use thiserror::Error as ThisError; // 使用 ThisError 来 derive 错误 trait
use std::fmt::{self, Display};
use std::error::Error as StdError; // 用于 Anyhow 变体中的 Box<dyn StdError>

/// Rin 框架的通用错误类型。
/// 这是一个枚举，包含了框架中所有预定义的错误类型，
/// 并且可以透明地包装来自其他 crate 或第三方库的错误。
#[derive(Debug, ThisError)] // 使用 thiserror 自动实现 StdError 和 Display
pub enum RinError {
    #[error("Not Found")]
    NotFound, // HTTP 404

    #[error("Method Not Allowed")]
    MethodNotAllowed, // HTTP 405

    #[error("Bad Request: {0}")]
    BadRequest(String), // HTTP 400，客户端请求有误

    #[error("Unauthorized")]
    Unauthorized, // HTTP 401

    #[error("Forbidden")]
    Forbidden, // HTTP 403

    #[error("Internal Server Error: {0}")]
    Internal(String), // HTTP 500，服务器内部错误

    /// 包装来自其他 Crate 或第三方库的、不属于上述分类的错误。
    /// `#[from]` 允许将任何实现了 `std::error::Error + Send + Sync + 'static`
    /// 的类型自动转换为 `RinError::Other`。
    /// `#[transparent]` 属性使得 `RinError::Other` 的 `Display`, `Debug`, `Source`
    /// 实现直接使用其内部错误的值，这对于错误链追踪非常有帮助。
    #[error(transparent)]
    Other(#[from] Box<dyn StdError + Send + Sync + 'static>),

    // 你也可以保留特定的 #[from] 变体，如果某些特定错误你希望有更明确的标签或处理逻辑
    #[error("Anyhow Error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

// 为了保持与之前 `rin_core::Error` 的使用习惯一致，你可以在 `lib.rs` 中将 `RinError` 重新导出为 `Error`。
// 这样用户在导入时仍然可以使用 `use rin_core::Error;`。