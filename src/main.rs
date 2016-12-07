pub trait Request<'a> {
    fn new(msg: &'a str) -> Self;
}

pub trait Response<'a> {
    type Request: Request<'a>;
    fn new(msg: &'a str, req: &'a Self::Request) -> Self;
}

pub trait Sink<'a> {
    type Response: Response<'a>;
    fn write(&self, &Self::Response);
}

struct ARequest<'a> {
    msg: &'a str,
}

impl<'a> Request<'a> for ARequest<'a> {
    fn new(msg: &'a str) -> Self {
        ARequest {
            msg: msg,
        }
    }
}

struct AResponse<'a> {
    msg: &'a str,
    req: &'a ARequest<'a>,
}

impl<'a> Response<'a> for AResponse<'a> {
    type Request = ARequest<'a>;
    fn new(msg: &'a str, req: &'a Self::Request) -> Self {
        AResponse {
            msg: msg,
            req: req,
        }
    }
}

struct ASink {}

impl<'a> Sink<'a> for ASink {
    type Response = AResponse<'a>;
    fn write(&self, response: &Self::Response) {}
}

fn main() {
    println!("Hello, world!");
}

struct Encoder<'a, S: for<'b> Sink <'b> + 'a> {
    sink: &'a S,
}

impl<'a, S: for<'b> Sink<'b> + 'a> Encoder<'a, S> {
    fn encode(&self) {
        let req = <S::Response as Response>::Request::new("test");
        let resp = S::Response::new("test", &req);
        self.sink.write(&resp)
    }
}