use tauri_test::prelude::*;

#[test]
fn hello_world() {
    let app = App::new().unwrap();
    let response = app.invoke("hello_world", ()).unwrap();
    assert_eq!(response, "Hello, world!");
}