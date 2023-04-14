use http_req::{
    request::{Method, Request},
    uri::Uri,
};

fn main() {
    let raw = "https://www-media.stanford.edu";
    let uri = Uri::try_from(raw).unwrap();

    let mut request = Request::new(&uri);
    request.method(Method::GET);

    let mut writer = Vec::new();
    _ = request.send(&mut writer);

    let text = String::from_utf8_lossy(&writer);
    println!("{text}")
}
