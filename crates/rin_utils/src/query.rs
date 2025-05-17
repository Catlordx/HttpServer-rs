use async_trait::async_trait;
use rin_core::{Context, RinError};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value}; // We'll use serde_json for deserialization.
// Make sure to add `serde_json = "1.0"` to your Cargo.toml.
use log; // Uncomment if you have the `log` crate configured and want warning messages.

// Helper function for URL decoding.
// This is a basic implementation and may not cover all edge cases
// (e.g., highly malformed sequences or specific encoding standards)
// as robustly as a dedicated library. It handles '+' as space and basic %XX decoding.
fn decode_uri_component_to_string(s: &str) -> String {
    let mut bytes = s.bytes();
    let mut decoded_bytes = Vec::new();

    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let h1 = bytes.next();
                let h2 = bytes.next();
                if let (Some(bh1), Some(bh2)) = (h1, h2) {
                    let hex_str = format!("{}{}", bh1 as char, bh2 as char);
                    if let Ok(byte_val) = u8::from_str_radix(&hex_str, 16) {
                        decoded_bytes.push(byte_val);
                    } else {
                        // Malformed hex sequence, push raw bytes
                        decoded_bytes.push(b'%');
                        decoded_bytes.push(bh1);
                        decoded_bytes.push(bh2);
                    }
                } else {
                    // Malformed % sequence (e.g., % followed by only one hex char), push raw bytes
                    decoded_bytes.push(b'%');
                    if let Some(bh1) = h1 {
                        decoded_bytes.push(bh1);
                    }
                    if let Some(bh2) = h2 {
                        decoded_bytes.push(bh2);
                    }
                }
            }
            b'+' => decoded_bytes.push(b' '),
            _ => decoded_bytes.push(b), // Push other characters directly
        }
    }

    // Attempt to convert the collected bytes to a UTF-8 string.
    // `String::from_utf8_lossy` provides robustness by replacing invalid UTF-8 sequences.
    String::from_utf8_lossy(&decoded_bytes).into_owned()
}

/// 扩展 `Context` 以提供查询参数处理方法。
#[async_trait]
pub trait ContextQueryExt {
    /// 获取单个查询参数的值。
    ///
    /// 此方法返回的是原始的、未经 URL 解码的字符串切片。
    /// 例如，对于查询字符串 `name=John%20Doe`，`query("name")` 将返回 `Some("John%20Doe")`。
    /// 如果需要解码，请手动对返回的 `&str` 调用 `decode_uri_component_to_string` 辅助函数。
    ///
    /// # 参数
    /// - `key`: 要查找的查询参数的键（未解码）。
    fn query(&self, key: &str) -> Option<&str>;

    /// 将所有查询参数反序列化到指定的类型。
    ///
    /// 参数键和值将被 URL 解码，并尝试作为 JSON 对象反序列化到 `T`。
    /// 如果存在重复的查询参数键，只有最后一个值会被保留。
    ///
    /// # Errors
    /// 如果查询参数无法解析或与 `T` 不匹配，则返回 `RinError::BadRequest`。
    fn bind_query<T: DeserializeOwned>(&self) -> Result<T, RinError>;
}

#[async_trait]
impl ContextQueryExt for Context {
    fn query(&self, key: &str) -> Option<&str> {
        // 
        // // 获取请求 URI 中的原始查询字符串。
        // self.request.uri.query().and_then(|query_str| {
        //     // 将查询字符串按 '&' 分割成单独的键值对。
        //     for pair in query_str.split('&') {
        //         // 将每个键值对按第一个 '=' 分割，以区分键和值。
        //         // `splitn(2, '=')` 确保我们只在第一个 '=' 处分割，允许值中包含 '='。
        //         let mut parts = pair.splitn(2, '=');
        // 
        //         // 获取原始键部分。
        //         if let Some(raw_key) = parts.next() {
        //             // 直接比较原始键与传入的 `key` 参数。
        //             // 假设传入的 `key` 未经 URL 编码。
        //             // 如果查询字符串中的键是 URL 编码的，且需要解码后才能与 `key` 比较，
        //             // 则此处的逻辑需要先解码 `raw_key` (这将导致一个 `String` 分配)。
        //             // 鉴于 `Option<&str>` 的返回类型，我们避免为值进行分配。
        //             if raw_key == key {
        //                 // 如果键匹配，则返回原始的值部分作为切片。
        //                 // 值部分可能仍是 URL 编码的，调用者如果需要应自行解码。
        //                 return parts.next();
        //             }
        //         }
        //     }
        //     // 遍历所有键值对后未找到匹配的键，则返回 `None`。
        //     None
        // })
        unimplemented!()
    }

    fn bind_query<T: DeserializeOwned>(&self) -> Result<T, RinError> {
        // // 获取原始查询字符串；如果没有查询参数，则使用空字符串。
        // let query_str = self.request.uri.query().unwrap_or("");
        // let mut json_map = Map::new();
        // 
        // // 遍历查询字符串中的每个键值对。
        // for pair in query_str.split('&') {
        //     let mut parts = pair.splitn(2, '=');
        // 
        //     // 提取原始的键和值字符串。处理键可能为空或值可能缺失的情况（例如，`key=` 或 `key`）。
        //     let key_encoded = parts.next().unwrap_or("");
        //     let value_encoded = parts.next().unwrap_or("");
        // 
        //     // 使用我们的辅助函数对键和值进行 URL 解码。
        //     let key = decode_uri_component_to_string(key_encoded);
        //     let value = decode_uri_component_to_string(value_encoded);
        // 
        //     // 将解码后的键值对插入到 JSON Map 中。
        //     // 默认情况下，所有查询参数值都被视为字符串。
        //     // 注意：如果存在重复的键（例如 `a=1&a=2`），后一个值会覆盖前一个值。
        //     json_map.insert(key, Value::String(value));
        // }
        // 
        // // 将解码后的参数 Map 转换为 `serde_json::Value::Object`。
        // let json_value = Value::Object(json_map);
        // 
        // // 尝试将 JSON 值反序列化为目标类型 `T`。
        // serde_json::from_value(json_value).map_err(|e| {
        //     // 如果反序列化失败，则记录警告（如果 `log` crate 已配置）并返回 `BadRequest` 错误。
        //     log::warn!("Failed to deserialize query parameters: {}", e);
        //     RinError::BadRequest(format!("Invalid query parameters: {}", e))
        // })
        unimplemented!()
    }
}
