use nickel::{Nickel, HttpRouter, QueryString};
use serdes::decode;

pub fn start_server() {
    let mut server = Nickel::new();
    server.get("/admin", middleware! { |request|
        let query = request.query().get("url");
        format!("{:?}", query)
    });
    server.get("/:short_path", middleware! { |request|
        format!("This is the URL received: {:?}", request.param("short_path").unwrap())
    });

    server.listen("127.0.0.1:6767");
}
