# DIESEL
Diesel 是针对Rust的安全的、可扩展的 ORM 和查询生成器

## 安装
```sh
cargo install diesel --features mysql


cargo install diesel_cli
# 只安装对应依赖的cli
cargo install diesel_cli --no-default-features --features mysql
```

## setup
新建一个`.env`用来保存配置项例如数据库连接
```
DATABASE_URL=postgres://username:password@localhost/diesel_demo
```
然后 setup
```sh
diesel setup
```
这个命令会在数据库不存在时创建数据库，并创建一个新的迁移目录用来管理 schema

现在来创建一个迁移
```sh
diesel migration generate create_posts
```
cli工具会在迁移目录创建两个文件：
```
Creating migrations/20160815133237_create_posts/up.sql
Creating migrations/20160815133237_create_posts/down.sql
```
这使得我们可以根据数据库变化使用`up  down`来跟新或退回数据库到对应版本
接着我们要在对应文件里编写SQL以管理数据库：
```sql
-- up.sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)

-- down.sql
DROP TABLE posts
```
然后应用这个数据库版本：
```sh
diesel migration run
```
如果需要回滚，要确保`down.sql`正确，然后运行以下命令：
```sh
diesel migration redo
```

## 在 Rust 中
首先建立数据库连接
```rs
// src/lib.rs
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("ERROR connecting to {}", database_url))
}
```

接着创建 model
```rs
// src/models.rs

use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}
```
`#[derive(Queryable)]`宏将生成所有查询 Post 表所需要的代码

对于 schema，通常不会手动创建，而是通过 Diesel 生成，在运行`diesel setup`时会生成`diesel.toml`文件，这个文件会帮助 Diesel 在`src/schemas.rs`维护 schema
这个文件类似于：
```rs
diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```
他描述了一个表的所有列
当我们使用数据库迁移（更新或回滚）时都会自动更新

接下来进行数据获取：
```rs
use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
  use self::schema::posts::dsl::*;

  let connection = &mut establish_connection();
  let results = posts
    .filter(published.eq(true))
    .limit(5)
    .load::<Post>(connection)
    .expect("Error loading posts");

  println!("display {} posts", results.len());

  for post in results {
    println!{"{}", post.title};
    println!{"--------------\n"};
    println!{"{}", post.body};
  }
}
```

新建一条数据
```rs
// src/models.rs
use crate::scheam::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str,
}

// src/lib.rs
use self::models::{NewPost, Post};

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
  use crate::schema::posts;

  let new_post = NewPost {title, body};

  diesel::insert_into(post::table)
    .values(&new_post)
    .get_result(conn)
    .expect("Error saving new post")
}


// src/bin/write_post.rs
use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
  let connection = &mut establish_connection();

  let mut title = String::new();
  let mut body = String::new();

  println!("input your title");
  stdin().read_line(&mut title).unwrap();
  let title = title.trim_end();

  println!(
    "\n Ok, input you content"
  )

  stdin().read_to_string(&mut body).unwrap();

  let post = create_post(connection, title, body);

  println!("\n Saved with id: {}", post.id);
}
```

怎么更新呢
```rs
// src/bin/publish_post.rs

use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
  use self::schema::posts::dsl::{posts, published};

  let id = args()
    .nth(1)
    .except("need id")
    .parse::<i32>()
    .except("invalid id");

  let connection = &extablish_connection();


  let post = diesel::update(posts.find(id))
    .set(published.eq(true))
    .get_result::<Post>(connection)
    .unwrap();

  println!("Published post {}", post.title);
}
```

删除他
```rs
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
  use self::schema::posts::dsl::*;

  let target = args().nth(1).except("need target");
  let pattern = format!("%{}%", target);

  let connection = &mut establish_connection();
  let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(cionnection)
    .except("Error delecting posts");

  println!("delete {} posts", num_deleted);
}
```