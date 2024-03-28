mod stuff;

use stuff::fn_holder::FnHolder;
use stuff::bunk_request::BunkRequest;

struct TacoRequest {}
impl BunkRequest<i32> for TacoRequest {
    fn send(&self) -> String {
        "taco".to_string()
    }
}

struct Foo {}
impl Foo {
    fn bar(&self) -> String {
        "bar".to_string()
    }
    async fn bar(&self, s: &str) -> String {
        s.to_string()
    }
}

fn main() {
    println!("Hello, world!");
    let fn_holder = FnHolder {
        id: "hello".to_string(),
        the_fn: |s| s.to_uppercase()
    };
    println!("fn_holder.id: {}", fn_holder.id);
    println!("fn_holder.the_fn: {}", (fn_holder.the_fn)("taco"));
}
