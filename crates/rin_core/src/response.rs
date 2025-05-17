use http::{StatusCode, HeaderMap};
use bytes::Bytes;
use std::convert::Into;

/// 表示一个高层次的 HTTP 响应。
#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl Response {
    /// 创建一个带有默认状态码 200 OK 的新响应。
    pub fn new() -> Self {
        Response {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    /// 设置响应状态码。
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// 设置响应体。
    pub fn with_body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = body.into();
        self
    }

    /// 设置响应头。
    pub fn with_header(mut self, key: http::header::HeaderName, value: http::header::HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// 获取响应头可变引用。
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// 设置响应体。
    pub fn set_body(&mut self, body: impl Into<Bytes>) {
        self.body = body.into();
    }

    /// 设置响应状态码。
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}


/// 允许各种类型转换为 `Response`，简化处理函数的返回类型。
///
/// 例如，一个字符串可以直接转换为 200 OK 的文本响应。
#[async_trait::async_trait] // IntoResponse 可能需要 async 适配，但这里是同步的，为了未来兼容性保留
pub trait IntoResponse {
    /// 将自身转换为 `Response`。
    fn into_response(self) -> Response;
}

// 实现常见的 From<T> for Response，并将其封装在 IntoResponse Trait 中
impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new().with_body(self).with_header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("text/plain; charset=utf-8"),
        )
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::new().with_body(self).with_header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("text/plain; charset=utf-8"),
        )
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        Response::new().with_status(self)
    }
}

// TODO: 可以为 Result<T, E> 实现 IntoResponse，以便处理函数直接返回 Result
// impl<T: IntoResponse, E: Into<Error>> IntoResponse for Result<T, E> {
//     fn into_response(self) -> Response {
//         match self {
//             Ok(r) => r.into_response(),
//             Err(e) => {
//                 // 这里需要将错误转换为一个适当的 HTTP 响应，例如 500 Internal Server Error
//                 // 这部分通常由框架的错误处理器来做，而不是 IntoResponse 自身
//                 // 但如果需要简单的默认行为，可以在这里实现
//                 log::error!("Unhandled error in IntoResponse: {}", e.into()); // 使用 from for Error
//                 Response::new().with_status(StatusCode::INTERNAL_SERVER_ERROR)
//             }
//         }
//     }
// }