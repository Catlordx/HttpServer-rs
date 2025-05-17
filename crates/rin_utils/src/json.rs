use async_trait::async_trait;
use bytes::Bytes;
use rin_core::{Context, Method, RinError};
use serde::Serialize;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait ContextJsonExt {
    /// 从请求体中解析 JSON 到指定的类型。
    ///
    /// # Errors
    /// 如果请求体无法读取或不是有效的 JSON，则返回 `RinError::BadRequest`。
    async fn bind_json<T: DeserializeOwned>(&mut self) -> Result<T, RinError>;

    /// 发送 JSON 响应。
    ///
    /// # Errors
    /// 如果 `value` 无法序列化为 JSON，则返回 `RinError::Internal`。
    fn json<T: Serialize>(&mut self, value: &T) -> Result<(), RinError>;
}

#[async_trait]
impl ContextJsonExt for Context {
    async fn bind_json<T: DeserializeOwned>(&mut self) -> Result<T, RinError> {
        unimplemented!()
        // let body_bytes = self.body_bytes().clone(); // Clone for reading
        // if body_bytes.is_empty() {
        //     return Err(RinError::BadRequest(
        //         "Request body is empty for JSON parsing".to_string(),
        //     ));
        // }
        // 
        // serde_json::from_slice(&body_bytes).map_err(|e| {
        //     log::warn!("Failed to parse JSON body: {}", e);
        //     RinError::BadRequest(format!("Invalid JSON format: {}", e))
        // })
    }

    fn json<T: Serialize>(&mut self, value: &T) -> Result<(), RinError> {
        unimplemented!()
        // match serde_json::to_vec(value) {
        //     Ok(json_bytes) => {
        //         self.response.headers_mut().insert(
        //             rin_core::header::CONTENT_TYPE,
        //             rin_core::header::HeaderValue::from_static("application/json"),
        //         );
        //         self.response.set_body(Bytes::from(json_bytes));
        //         Ok(())
        //     }
        //     Err(e) => {
        //         log::error!("Failed to serialize JSON response: {}", e);
        //         Err(RinError::Internal(format!(
        //             "Failed to serialize JSON: {}",
        //             e
        //         )))
        //     }
        // }
    }
}
