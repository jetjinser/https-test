
use http_req::sslwrapper::{get_receive, send_data};

const CR_LF: &str = "\r\n";

fn parse_msg<S: Into<String>>(
    method: S,
    uri: S,
    version: S,
    headers: Vec<(S, S)>,
    body: Option<&[u8]>,
) -> Vec<u8> {
    let request_line = format!(
        "{} {} {}{}",
        method.into(),
        uri.into(),
        version.into(),
        CR_LF
    );

    let headers: String = headers
        .into_iter()
        .map(|(k, v)| format!("{}: {}{}", k.into(), v.into(), CR_LF))
        .collect();

    let mut request_msg = (request_line + &headers + CR_LF).as_bytes().to_vec();

    if let Some(b) = body {
        request_msg.extend(b);
    }

    request_msg
}

fn main() {
    let host = "hbr.org";
    let port = 443;
    let headers = vec![
        ("accept", "*/*"),
        ("Host", "hbr.org"),
        ("user-agent", "curl/8.0.1"),
        ("Connection", "close"),
    ];
    let msg = parse_msg("GET", "/", "HTTP/1.1", headers, None);
    let body = String::from_utf8_lossy(&msg);
    println!("{}", body);
    send_data(host, port, &body);

    let re = get_receive();
    let ret = String::from_utf8_lossy(&re.rcv_vec);
    println!("{}", ret);
}
