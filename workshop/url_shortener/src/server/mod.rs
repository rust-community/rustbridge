use nickel::{Nickel, HttpRouter};

pub fn start_server() {
    let mut server = Nickel::new();
    server.get("/admin/:short_url", middleware!("In admin"));
    server.get("/:url", middleware! { |request|
        format!("This is the URL received: {:?}", request.param("url").unwrap())
    });

    server.listen("127.0.0.1:6767");
}
