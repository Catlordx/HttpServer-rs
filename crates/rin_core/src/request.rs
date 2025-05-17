use bytes::Bytes;
use http::{HeaderMap, Method, Uri};
use std::collections::HashMap;

/// 表示一个高层次的 HTTP 请求。
#[derive(Debug, Clone)] // Clone 是为了在 Context 中传递 ownership 或做一些内部克隆
pub struct Request {
    /// HTTP 方法 (GET, POST, etc.)
    pub method: Method,
    /// 请求的 URI (路径, 查询参数等)
    pub uri: Uri,
    /// 请求头集合
    pub headers: HeaderMap,
    /// 请求体原始字节数据
    pub body_bytes: Bytes,
    /// 路径参数 (例如 /users/:id 中的 id，由路由器填充)
    pub params: HashMap<String, String>,
}

impl Request {
    /// 创建一个新的空请求。通常由服务器解析后填充。
    pub fn new(method: Method, uri: Uri, headers: HeaderMap, body_bytes: Bytes) -> Self {
        Request {
            method,
            uri,
            headers,
            body_bytes,
            params: HashMap::new(), // 初始为空，由路由器填充
        }
    }

    /// 获取请求的 URL 查询字符串。
    pub fn query_str(&self) -> Option<&str> {
        self.uri.query()
    }
}
