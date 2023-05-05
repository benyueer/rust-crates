# Rocket


## 请求配置
使用宏注册请求
```rs
#[get("/hello")]

```

动态路径（路由参数）
```rs
#[get("/hello/<name>")]
fn hello(name: &str)

/// 多个参数
#[get("/user?<params...>")]
fn user(params: UserParams)

#[get("/user/<params...>")]
fn user(params: UserParams)
```

转发
对于相同路径但参数类型不同的路由，Rocket会尝试转发请求到下一个匹配路由，默认使用 -12 到 -1 的优先级匹配，也可以自定义优先级，都是从小到大匹配
```rs
#[get("/user/<id>")]
fn user(id: usize)

#[get("/user/<id>", rank=2)]
fn user(id: isize)

#[get("/user/<id>", rank = 3)]
fn user(id: &str)
```
如果未明确指定等级，会分配默认等级，当路径或query参数的动态性越强，那么该请求的 rank 越高，即优先级越低



路由守卫
守卫显式的处理输入，可以设置任意数量的守卫，在调用守卫之前，Rocket 会自动调用守卫的`FromRequest`实现，只有所有守卫都通过，才会继续处理请求
所有未在路由属性中注册的路由处理函数的参数都被视为守卫
守卫的执行顺序从左到右，之前的守卫失败就不会继续执行之后的守卫
```rs
#[get("/user/<id>")]
fn user(id: &str, a: A, b: B, c: C)
```

自定义守卫
实现`FromRequest`类型就可以实现一个自定义守卫
```rs
#[get("/user")]
fn user(auth: Auth)


struct Auth {
  pub id: i32,
  pub exp: i64,
  pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
  type Error = ();


  async fn from_request(req: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
    // let state = req.rocket().state::<AppState>.unwrap();
    if auth {
      Outcome::Success(auth)
    } else {
      Outcome::Failure(Status::Forbidden, ())
    }
  }
}
```

使用Cookie
使用`CookieJar`守卫可以操作 Cookie
```rs
use rocket::http::CookirJar;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Option<String> {
  cookies.get("message").map(|crumb| format!{"message: {}", crumb.value()})
}
```


参数和响应格式
使用格式路由参数指定接收和响应格式
```rs
#[post("/user", format="application/json", data="<user>")]
fn new_user(user: User)
```

请求体处理
JSON：使用`format`，使用`Json<T>`守卫
```rs
use rocket::serde::{Deserialize};

#[derive(Deserialize)]
struct User {
  name: String,
  age: i32,
}

#[post("/user", format="json", data="<user>")]
fn user(user: Json<User>)
```

文件：
使用`TempFile`守卫，然后持久化
```rs
use rocket::fs::TempFile;

#[post("/upload", format="plain", data="<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
  file.persist_to(path).await
}
```

流：



表单处理
使用`Form`守卫和`FromForm`宏
```rs
use rocket::form::Form;

#[derive(FromForm)]
struct Task<'r> {
  complete: bool,
  r#type: &'r str,
}

#[post("/todo", data="<task>")]
fn new(task: Form<Task<'_>>)
```
验证
使用`Strict`可在缺失任何字段时抛出错误
```rs
use rocket::form::{Form, Strict};

#[post("/todo", data="<task>")]
fn new(task: Form<Strict<Task<'_>>)

// 或在单个字段上使用严格验证
#[derive(FromForm)]
struct Task {
  name: Strict<String>,
  age: i32,
}
```

默认值
一些具有默认值的类型会取默认值
```rs
#[derive(FromForm)]
struct MyForm {
  maybe_string: Option<String>, // Option -> default None
  ok_or_err: from::Result<_, Vec<String>>, // Result -> default Err(Missing)
  here_or_false: bool, // bool -> default false
}

// 也可使用属性宏覆盖默认值
#[derive(FromForm)]
struct MyForm {
  #[field(default = "hello")]
  greeting: String,

  // 当字面值是 None 时，表示移除该类型的默认值，例如 bool 的 false，这使得在解析时此字段必须有值
  #[field(default = None)]
  is_friendly: bool,
}
```

字段重命名
同样使用属性宏
```rs
#[derive(FromForm)]
struct Exxternal {
  #[field(name = "first-name")]
  first_name: String,

  // 保留多个字段名
  #[field(name="first-name")]
  #[field(name="firstname")]
  #[field(name="first_name")]
  first_name: String,
}
```

字段验证
同样通过属性宏
```rs
#[derive(FromForm)]
struct Person {
  #[field(validate = range(21..))]
  age: u16,
}
```


## 响应配置

## 状态管理

添加状态
使用`manage`添加状态
但添加之前要定义一个状态，Rocket 会自动对程序进行多线程处理，因此需要保证状态是线程安全的
```rs
use std::sync::atomic::AtomicUsize;

struct HitCount {
  count: AtomicUsize,
}

rocket::build().manage(HitCount{ count: AtomicUsize::new(0)});


use rocket::State;

#[get("/count")]
fn count(hit_count: &State<HitCount>) -> String {
  let current_count = hit_count.count.load(Ordering::Relaxed);
}
```

状态本身也是一个请求守卫，可以在其他守卫中获取
```rs
struct Item<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Item<'r> {
  type Error = ();

  async fn from_request(request: &'r Request<'r>) -> request::Outcome<Self, ()> {
    // 使用 Request::guard()
    let outcome = request.guard::<&State<MyConfig>>().await.map(|my_config| Item(&my_config.user_val));

    // 使用 Rocket::state()
    let outcome = request.rocket().state::<MyConfig>().map(|my_config| Item(&my_config.user_val)).or_forward(());

    outcome
  }
}
```

请求本地状态


## 中间件
使用中间件可以在请求的生命周期内对请求进行额外处理

与其他框架中间件有所不同：
- Fairing 无法直接终止请求
- 不能讲任意的、非请求的数据注入到请求中
- 可以阻止程序启动
- 可以检查和修改程序配置



注册中间件
使用`attach`
```rs
#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(req_fairing)
    .attach(res_fairing)
}
```
中间件的执行顺序就是他们的注册顺序

中间件回调
Rocket 会针对5个事件触发中间件
- Ignite  on_ignite  应用启动期间，可以在这时候解析验证应用配置、更改配置或设置状态等
- Liftoff  on_liftoff 在应用启动后触发，可以获得Rocket实例
- Request   on_request  收到请求后立即调用请求回调，可以随意修改和查看请求数据，但不会终止或响应请求
- Response  on_response   当准备发送响应时会触发响应回调，可以修改所有响应数据，例如修改响应头等
- Shutdown  on_shutdown   在应用关闭时触发

所有中间件都要实现`Fairing`特征，必须实现`info`方法，该方法返回`Info`，Rocket 使用此结构组册中间件的名称和回调生命周期
所有中间件都必须实现`Send + Sync + 'static`，也就是可跨线程发送且线程安全，且只有静态引用

示例：
我们实现一个记录请求类型数量的中间件，可以用全局状态和路由守卫实现，但这样会在每个请求上都加上守卫，不够优雅，使用中间件则是更好的选择
```rs
use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};

struct Counter {
  get: AtomicUsize,
  post: AtomicUsize,
}

#[rocket::async_trait]
impl Fairing for Counter {
  fn info(&self) -> Info {
    Info {
      name: "GET/POST Counter",
      kind: Kind::Request | Kind::Response
    }
  } 

  async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
    match request.method() {
      Method::Get => self.get.fetch_add(1, Oridering::Relaxed),
      Method::Post => self.post.fetch_add(1, Oridering::Relaxed),
      _ => return
    }
  }

  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    if response.status() != Status::NotFound {
      return 
    }

    if request.method() == Method::Get && request.uri().path() == "/counts" {
      let get_count = self.get.load(Oridering::Relaxed);
      let post_count = self.post.load(Oridering::Relaxed);
      let body = format!{"GET: {}\nPOST: {}", get_count, post_count};

      response.set_status(Status::Ok);
      response.set_header(ContentType::Plain);
      response.set_sized_body(body.len(), Cursor::new(body));
    }
  }
}
```

Ad-Hoc 中间件
对于简单情况，实现一个完整中间件很麻烦，于是就出现了 Ad-Hoc 中间件，他可以用一个闭包或普通函数创建中间件
```rs
use rocket::fairing::AdHoc;
use rocket::http::Method;

rocket::build()
  .attach(AdHoc::on_liftoff("liftoff Printer", |_| Box::pin(async move{
    println!("...annnddd we have liftoff!");
  })))
  .attach(AdHoc::on_request("put Rewriter", |req, _| Box::pin(async move{
    req.set_method(Method::Put);
  })));
```

## 配置
Rocket 配置基于 Figment

使用`Config::figment()`编辑配置项

使用配置文件：在根目录新建`rocket.toml`
使用环境变量：Rocket 会寻找所有以`ROCKET_`开始的变量