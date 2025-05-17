use std::collections::HashMap;
use std::any::{Any, TypeId}; // 用于存储用户自定义数据
use crate::request::Request;
use crate::response::Response;
// 用于辅助 Any 到 Box<Any> 的转换，如果需要的话

/// 请求处理的上下文。
/// 包含了请求、响应、路径参数以及用于中间件通信的任意数据。
#[derive(Debug)]
pub struct Context {
    pub request: Request,
    pub response: Response,
    // 路径参数，例如 /users/:id 中的 id
    pub params: HashMap<String, String>,
    // 用于存储请求范围内的任意数据，供中间件和处理函数之间传递信息
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    // 内部错误，如果处理过程中发生了错误，可以在这里设置
    // pub error: Option<RinError>, // 也可以通过 Result<Response, RinError> 返回
}

impl Context {
    /// 创建一个新的上下文。
    /// 通常由服务器在接收到请求时创建。
    pub fn new(request: Request, response: Response) -> Self {
        Context {
            request,
            response,
            params: HashMap::new(),
            data: HashMap::new(),
            // error: None,
        }
    }

    /// 获取路径参数的值。
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    }

    /// 获取请求的 Method。
    pub fn method(&self) -> &http::Method {
        &self.request.method
    }

    /// 获取请求的 Uri。
    pub fn uri(&self) -> &http::Uri {
        &self.request.uri
    }

    /// 获取请求头。
    pub fn headers(&self) -> &http::HeaderMap {
        &self.request.headers
    }

    /// 获取请求体的原始字节数据。
    pub fn body_bytes(&self) -> &bytes::Bytes {
        &self.request.body_bytes
    }

    /// 设置响应状态码。
    pub fn set_status(&mut self, status: http::StatusCode) {
        self.response.set_status(status);
    }

    /// 设置响应体。
    pub fn set_body(&mut self, body: impl Into<bytes::Bytes>) {
        self.response.set_body(body);
    }

    /// 获取响应头可变引用。
    pub fn headers_mut(&mut self) -> &mut http::HeaderMap {
        self.response.headers_mut()
    }

    /// 将数据存储到上下文中，供后续处理函数或中间件使用。
    /// `T` 必须是 `'static` 并且 `Send + Sync`。
    pub fn set<T: Any + Send + Sync + 'static>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// 从上下文中获取存储的数据。
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())
            .and_then(|boxed_value| boxed_value.downcast_ref::<T>())
    }

    /// 从上下文中获取存储数据的可变引用。
    pub fn get_mut<T: Any + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>())
            .and_then(|boxed_value| boxed_value.downcast_mut::<T>())
    }

    // TODO: 实现更多 Gin-like 的便捷方法，例如：
    // - HTML 渲染方法 (需要渲染器集成)
    // - 重定向方法
    // - 文件下载方法
    // - 获取客户端 IP
    // - 获取请求 ID (如果使用)
}