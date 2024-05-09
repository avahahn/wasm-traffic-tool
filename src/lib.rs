pub use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam, Method,
};

struct Component;

impl wasi::exports::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");
        ResponseOutparam::set(outparam, Ok(resp));
        let out = body.write().expect("outgoing stream");

        for (k, v) in request.headers().entries() {
            let val = String::from_utf8(v).expect("non utf8 value in headers");
            out.blocking_write_and_flush(format!("{} = {}\n", k, val).as_bytes())
                .expect("printing headers");
        }

        if let Method::Post = request.method() {
            let body = request.consume()
                .expect("consuming request");
            out.blocking_write_and_flush(b"> have body\n")
                .expect("writing response");
            let stream = body.stream()
                    .expect("getting stream");
            out.blocking_write_and_flush(b"Post Accepted\nBody:\n")
                .expect("writing response");

            while let Ok(somedata) = stream.blocking_read(1) {
                out.blocking_write_and_flush(&somedata).expect("writing response");
            }
            out.blocking_write_and_flush(b"\n").expect("writing response");
        } else {
            out.blocking_write_and_flush(b"Hello, wasi:http/proxy world!\n")
                .expect("writing response");
        }

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}

wasi::http::proxy::export!(Component);
