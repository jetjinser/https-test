> cargo r

    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `wasmedge target/wasm32-wasi/debug/https-test.wasm`
4006D9003B7F0000:error:0A000410:SSL routines:ssl3_read_bytes:sslv3 alert handshake failure:ssl/record/rec_layer_s3.c:1600:SSL alert number 40
[2023-04-14 11:57:04.795] [error] [WasmEdge Httpsreq] SSL_get_error code -1
[2023-04-14 11:57:04.795] [error] 141
[2023-04-14 11:57:04.795] [error]     When executing function name: "_start"
