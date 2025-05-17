# Rin

## 项目结构

<div style="text-align: center;">
    <img src="rin.png" alt="Rin" style="max-width: 500px; height: auto; display: block; margin: 0 auto;">
</div>

## TODO
在 Go 的 **Gin** 框架中，`gin.Context` 是处理 HTTP 请求的核心结构体，包含了与请求和响应相关的所有信息、字段和方法。它被广泛用于处理路由、解析请求数据、设置响应等操作。以下是对 `gin.Context` 的字段和方法的详细说明，基于 Gin 框架的最新版本（截至 2025 年 5 月，基于 Gin 的 v1.x 版本，具体以 v1.9.x 为准）。

---

### 一、**`gin.Context` 的主要字段**
`gin.Context` 的字段大多是私有的（小写开头），因此不能直接访问。以下是主要字段的说明（通过文档和源码推导）：

1. **私有字段（不可直接访问）**：
    - `w`: `http.ResponseWriter`，用于写入 HTTP 响应。
    - `r`: `*http.Request`，表示当前的 HTTP 请求。
    - `params`: `Params`，存储 URL 路径中的参数（例如 `/user/:id` 中的 `id`）。
    - `handlers`: `HandlerFunc` 切片，存储当前请求的处理函数链（中间件 + 路由处理函数）。
    - `index`: 当前处理函数的索引，用于跟踪中间件执行进度。
    - `keys`: `map[string]interface{}`，用于存储请求上下文中的键值对（通过 `c.Set` 和 `c.Get` 访问）。
    - `errors`: `[]*Error`，存储请求处理过程中的错误。
    - `accepted`: `[]string`，存储客户端接受的 MIME 类型。
    - `queryCache`: 缓存解析后的查询参数。
    - `formCache`: 缓存解析后的表单数据。
    - `engine`: `*Engine`，指向 Gin 的核心引擎实例。

这些字段是私有的，开发者通常通过 `gin.Context` 提供的方法来间接操作它们。

---

### 二、**`gin.Context` 的主要方法**
`gin.Context` 提供了大量方法，用于处理请求、响应和上下文数据。以下是常用的方法，按功能分类：

#### 1. **请求参数处理**
- **`Param(key string) string`**
  获取 URL 路径中的参数（例如 `/user/:id` 中的 `id`）。
  ```go
  id := c.Param("id") // 例如访问 /user/123，则返回 "123"
  ```

- **`Query(key string) string`**
  获取查询参数的值（例如 `?name=John` 中的 `name`），如果不存在返回空字符串。
  ```go
  name := c.Query("name") // 返回 "John"
  ```

- **`DefaultQuery(key, defaultValue string) string`**
  获取查询参数，如果不存在则返回默认值。
  ```go
  name := c.DefaultQuery("name", "Guest") // 如果无 name 参数，返回 "Guest"
  ```

- **`QueryArray(key string) []string`**
  获取查询参数的数组值（例如 `?ids=1&ids=2`）。
  ```go
  ids := c.QueryArray("ids") // 返回 []string{"1", "2"}
  ```

- **`QueryMap(key string) map[string]string`**
  获取查询参数的键值对映射（例如 `?user[name]=John&user[age]=30`）。
  ```go
  user := c.QueryMap("user") // 返回 map[string]string{"name": "John", "age": "30"}
  ```

- **`PostForm(key string) string`**
  获取 POST 表单数据的值。
  ```go
  username := c.PostForm("username")
  ```

- **`DefaultPostForm(key, defaultValue string) string`**
  获取 POST 表单数据，如果不存在则返回默认值。
  ```go
  username := c.DefaultPostForm("username", "Guest")
  ```

- **`PostFormArray(key string) []string`**
  获取 POST 表单数据的数组值。
  ```go
  hobbies := c.PostFormArray("hobbies") // 例如 hobbies=reading&hobbies=coding
  ```

- **`PostFormMap(key string) map[string]string`**
  获取 POST 表单数据的键值对映射。
  ```go
  user := c.PostFormMap("user") // 例如 user[name]=John&user[age]=30
  ```

- **`FormFile(name string) (*multipart.FileHeader, error)`**
  获取上传的文件。
  ```go
  file, err := c.FormFile("file")
  ```

- **`MultipartForm() (*multipart.Form, error)`**
  获取多部分表单数据（包括文件和字段）。
  ```go
  form, err := c.MultipartForm()
  ```

#### 2. **请求体绑定**
- **`Bind(obj interface{}) error`**
  根据 `Content-Type` 将请求体绑定到结构体（支持 JSON、XML、YAML 等）。
  ```go
  type User struct {
      Name string `json:"name"`
  }
  var user User
  c.Bind(&user)
  ```

- **`BindJSON(obj interface{}) error`**
  专门绑定 JSON 请求体。
  ```go
  c.BindJSON(&user)
  ```

- **`BindXML(obj interface{}) error`**
  绑定 XML 请求体。
  ```go
  c.BindXML(&user)
  ```

- **`BindYAML(obj interface{}) error`**
  绑定 YAML 请求体。
  ```go
  c.BindYAML(&user)
  ```

- **`ShouldBind(obj interface{}) error`**
  类似 `Bind`，但不会设置响应状态码（用于验证）。
  ```go
  c.ShouldBind(&user)
  ```

- **`ShouldBindJSON(obj interface{}) error`**, **`ShouldBindXML(obj interface{}) error`**, **`ShouldBindYAML(obj interface{}) error`**
  类似 `BindJSON` 等，但不设置响应状态码。

- **`BindQuery(obj interface{}) error`**
  将查询参数绑定到结构体。
  ```go
  type Query struct {
      Name string `form:"name"`
  }
  var query Query
  c.BindQuery(&query)
  ```

- **`ShouldBindQuery(obj interface{}) error`**
  类似 `BindQuery`，但不设置响应状态码。

- **`BindUri(obj interface{}) error`**
  将 URL 路径参数绑定到结构体。
  ```go
  type Params struct {
      ID string `uri:"id"`
  }
  var params Params
  c.BindUri(&params)
  ```

#### 3. **响应处理**
- **`JSON(code int, obj interface{})`**
  返回 JSON 响应，并设置状态码。
  ```go
  c.JSON(200, gin.H{"message": "success"})
  ```

- **`XML(code int, obj interface{})`**
  返回 XML 响应。
  ```go
  c.XML(200, gin.H{"message": "success"})
  ```

- **`YAML(code int, obj interface{})`**
  返回 YAML 响应。
  ```go
  c.YAML(200, gin.H{"message": "success"})
  ```

- **`String(code int, format string, values ...interface{})`**
  返回纯文本响应。
  ```go
  c.String(200, "Hello, %s!", "World")
  ```

- **`HTML(code int, name string, obj interface{})`**
  渲染 HTML 模板。
  ```go
  c.HTML(200, "index.tmpl", gin.H{"title": "Home"})
  ```

- **`Redirect(code int, location string)`**
  重定向到指定 URL。
  ```go
  c.Redirect(302, "/new/path")
  ```

- **`File(filepath string)`**
  返回文件内容。
  ```go
  c.File("./static/file.txt")
  ```

- **`FileAttachment(filepath, filename string)`**
  返回文件作为附件（触发下载）。
  ```go
  c.FileAttachment("./static/file.txt", "download.txt")
  ```

- **`Data(code int, contentType string, data []byte)`**
  返回原始字节数据。
  ```go
  c.Data(200, "text/plain", []byte("Hello, World!"))
  ```

#### 4. **上下文数据管理**
- **`Set(key string, value interface{})`**
  在上下文中存储键值对。
  ```go
  c.Set("userID", 123)
  ```

- **`Get(key string) (interface{}, bool)`**
  获取上下文中的值。
  ```go
  if userID, exists := c.Get("userID"); exists {
      fmt.Println(userID) // 123
  }
  ```

- **`GetString(key string) string`**, **`GetBool(key string) bool`**, **`GetInt(key string) int`**, 等等
  获取特定类型的值，带类型转换。
  ```go
  userID := c.GetInt("userID") // 123
  ```

- **`MustGet(key string) interface{}`**
  获取上下文中的值，如果不存在则 panic。
  ```go
  userID := c.MustGet("userID")
  ```

#### 5. **请求信息**
- **`Request() *http.Request`**
  返回底层的 HTTP 请求对象。
  ```go
  req := c.Request
  ```

- **`ClientIP() string`**
  获取客户端 IP 地址。
  ```go
  ip := c.ClientIP()
  ```

- **`RemoteIP() string`**
  获取远程客户端 IP（可能包含代理信息）。
  ```go
  remoteIP := c.RemoteIP()
  ```

- **`ContentType() string`**
  获取请求的 `Content-Type`。
  ```go
  contentType := c.ContentType()
  ```

- **`GetHeader(key string) string`**
  获取请求头的值。
  ```go
  token := c.GetHeader("Authorization")
  ```

#### 6. **中间件和流程控制**
- **`Next()`**
  调用下一个中间件或处理函数。
  ```go
  c.Next()
  ```

- **`Abort()`**
  中止后续中间件和处理函数的执行。
  ```go
  c.Abort()
  ```

- **`AbortWithStatus(code int)`**
  中止并设置状态码。
  ```go
  c.AbortWithStatus(403)
  ```

- **`AbortWithStatusJSON(code int, obj interface{})`**
  中止并返回 JSON 响应。
  ```go
  c.AbortWithStatusJSON(400, gin.H{"error": "bad request"})
  ```

- **`IsAborted() bool`**
  检查是否已中止。
  ```go
  if c.IsAborted() {
      return
  }
  ```

#### 7. **错误处理**
- **`Error(err error) *Error`**
  添加错误到上下文。
  ```go
  c.Error(errors.New("something went wrong"))
  ```

- **`Errors() []*Error`**
  获取所有错误。
  ```go
  errs := c.Errors
  ```

#### 8. **其他**
- **`Status(code int)`**
  设置响应状态码（不发送响应）。
  ```go
  c.Status(200)
  ```

- **`SetCookie(name, value string, maxAge int, path, domain string, secure, httpOnly bool)`**
  设置 Cookie。
  ```go
  c.SetCookie("session", "abc123", 3600, "/", "example.com", false, true)
  ```

- **`GetRawData() ([]byte, error)`**
  获取原始请求体数据。
  ```go
  data, err := c.GetRawData()
  ```

- **`SaveUploadedFile(file *multipart.FileHeader, dst string) error`**
  保存上传的文件到指定路径。
  ```go
  file, _ := c.FormFile("file")
  c.SaveUploadedFile(file, "./uploads/file.txt")
  ```

---

### 三、**注意事项**
1. **线程安全**：`gin.Context` 是为单个请求设计的，不应在多个 goroutine 中共享。
2. **性能优化**：Gin 的上下文对象是可重用的（通过对象池），因此避免在处理函数中存储对 `gin.Context` 的长期引用。
3. **绑定和验证**：绑定方法（如 `BindJSON`）会根据 `Content-Type` 自动选择解析方式，推荐结合 `validator` 库进行数据验证。
4. **中间件**：`gin.Context` 是中间件的核心，`Set` 和 `Get` 方法常用于在中间件之间传递数据。

---

### 四、**总结**
`gin.Context` 是 Gin 框架的核心，提供了丰富的字段和方法来处理 HTTP 请求和响应。它的方法涵盖了参数解析、请求体绑定、响应生成、上下文管理、错误处理等功能。开发者通常通过这些方法与请求交互，而无需直接访问私有字段。

如果你需要更具体的示例（例如某个方法的用法）或想深入某个功能，可以提供更多细节，我可以进一步帮你！