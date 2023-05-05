use crate::schema::posts::{self, id as pid};
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name=posts)]
pub struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
}

pub fn create_post(conn: &mut MysqlConnection, title: &str, body: &str) -> Post {
    use self::posts::dsl::{id, posts};

    let new_post = NewPost { title, body };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error create post");

    posts.order(id.desc()).first(conn).unwrap()
}

pub fn find_post(conn: &mut MysqlConnection, id: i32) -> Post {
    use self::posts::dsl::posts;
    let res = posts.filter(pid.eq(id)).first(conn).expect("Error find");
    return res;
}

pub fn publish_post(conn: &mut MysqlConnection, id: i32) -> () {
    use self::posts::dsl::{posts, published};

    let res = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(conn);
    // .unwrap();
    println!("{res:?}");
    // return res;
}

pub fn delete_post(conn: &mut MysqlConnection, id: i32) {
    use self::posts::dsl::{posts, id as pid};
    let res = diesel::delete(posts.find(id))
        .execute(conn);
    println!("{res:?}");
}