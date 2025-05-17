use bytes::Bytes;
use http::{HeaderMap, Method, Uri};
use std::collections::HashMap;
use std::path::PathBuf;

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

#[derive(Debug, Clone)]
pub struct QueryCache {
    // 键是参数名，值是单个值或多个值的枚举
    params: HashMap<String, QueryValue>,
}

// 查询参数值的枚举，处理单值和多值情况
#[derive(Debug, Clone)]
pub enum QueryValue {
    Single(String),
    Multiple(Vec<String>),
}

impl QueryCache {
    // 创建空的查询缓存
    pub fn new() -> Self {
        QueryCache {
            params: HashMap::new(),
        }
    }

    // 从查询字符串解析并填充缓存
    pub fn parse(query: &str) -> Self {
        let mut cache = QueryCache::new();
        if !query.is_empty() {
            for pair in query.split('&') {
                let mut kv = pair.splitn(2, '=');
                let key = kv.next().unwrap_or("");
                let value = kv.next().unwrap_or("");
                cache.insert(key.to_string(), value.to_string());
            }
        }
        cache
    }

    // 插入键值对，支持多值参数
    pub fn insert(&mut self, key: String, value: String) {
        self.params
            .entry(key)
            .and_modify(|v| match v {
                QueryValue::Single(s) => {
                    *v = QueryValue::Multiple(vec![s.clone(), value.clone()]);
                }
                QueryValue::Multiple(vec) => {
                    vec.push(value.clone());
                }
            })
            .or_insert(QueryValue::Single(value));
    }

    // 获取单个值
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).and_then(|v| match v {
            QueryValue::Single(s) => Some(s.as_str()),
            QueryValue::Multiple(vec) => vec.first().map(|s| s.as_str()),
        })
    }

    // 获取所有值（多值参数）
    pub fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get(key).and_then(|v| match v {
            QueryValue::Single(_) => None,
            QueryValue::Multiple(vec) => Some(vec),
        })
    }
}
// 表单缓存结构
#[derive(Debug, Clone)]
pub struct FormCache {
    // 普通字段的键值对
    fields: HashMap<String, FormValue>,
    // 文件上传的键值对
    files: HashMap<String, Vec<FileEntry>>,
}

// 表单字段值的枚举
#[derive(Debug, Clone)]
pub enum FormValue {
    Single(String),
    Multiple(Vec<String>),
}

// 文件上传的元数据
#[derive(Debug, Clone)]
pub struct FileEntry {
    filename: String,
    path: PathBuf, // 临时文件路径
    size: u64,     // 文件大小
    mime_type: String, // MIME 类型
}

impl FormCache {
    // 创建空的表单缓存
    pub fn new() -> Self {
        FormCache {
            fields: HashMap::new(),
            files: HashMap::new(),
        }
    }

    // 插入普通字段
    pub fn insert_field(&mut self, key: String, value: String) {
        self.fields
            .entry(key)
            .and_modify(|v| match v {
                FormValue::Single(s) => {
                    *v = FormValue::Multiple(vec![s.clone(), value.clone()]);
                }
                FormValue::Multiple(vec) => {
                    vec.push(value.clone());
                }
            })
            .or_insert(FormValue::Single(value));
    }

    // 插入文件
    pub fn insert_file(&mut self, key: String, file: FileEntry) {
        self.files
            .entry(key)
            .or_insert_with(Vec::new)
            .push(file);
    }

    // 获取单个字段值
    pub fn get_field(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(|v| match v {
            FormValue::Single(s) => Some(s.as_str()),
            FormValue::Multiple(vec) => vec.first().map(|s| s.as_str()),
        })
    }

    // 获取所有字段值
    pub fn get_field_all(&self, key: &str) -> Option<&Vec<String>> {
        self.fields.get(key).and_then(|v| match v {
            FormValue::Single(_) => None,
            FormValue::Multiple(vec) => Some(vec),
        })
    }

    // 获取文件列表
    pub fn get_files(&self, key: &str) -> Option<&Vec<FileEntry>> {
        self.files.get(key)
    }
}