// #[cfg(Test)]
use crate::database::conn;

pub mod Test {
    use crate::{database::conn, models::post::{create_post, find_post, publish_post, delete_post}};

    #[test]
    fn insert() {
        let conn = &mut conn();

        let title = String::from("this is title");
        let body = String::from("this is body");

        let post = create_post(conn, &title, &body);

        println!("{post:?}");
    }

    #[test]
    fn find() {
        let conn = &mut conn();
        let post = find_post(conn, 1i32);
        println!("{post:?}");
    }

    #[test]
    fn update() {
        let conn = &mut conn();
        let post = publish_post(conn, 3i32);
        println!("{post:?}");
    }

    #[test]
    fn delete() {
        let conn = &mut conn();
        delete_post(conn, 1i32);
    }
}
