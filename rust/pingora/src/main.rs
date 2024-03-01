use async_trait::async_trait;
use pingora::prelude::*;
use bytes::Bytes;
use std::sync::Arc;
use pingora::services::listening::Service;
use pingora::apps::http_app::ServeHttp;
use pingora::protocols::http::ServerSession;
use http::{Response, StatusCode, header};

pub struct HttpEchoApp;

#[async_trait]
impl ServeHttp for HttpEchoApp {
  async fn response(&self, _http_stream: &mut ServerSession) -> Response<Vec<u8>> {
    let body = Bytes::from("Hello World!");

    Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "text/plaintext")
      .header(header::TRANSFER_ENCODING, "Chunked")
      .body(body.to_vec())
      .unwrap()
  }
}

pub fn main() {
  let config = "./config.yaml".to_string();
  let mut my_server = Server::new(Some(Opt {
    conf: Some(config),
    upgrade: false,
    daemon: false,
    nocapture: false,
    test: false,
  })).unwrap();
  my_server.bootstrap();

  let mut echo_service_http = Service::new("Echo Service HTTP".to_string(), Arc::new(HttpEchoApp {}));
  echo_service_http.add_tcp("0.0.0.0:5000");

  my_server.add_service(echo_service_http);
  my_server.run_forever();
}

// pub fn main() {
//   let mut my_server = Server::new(None).unwrap();
//   my_server.bootstrap();

//   let mut echo_service_http = Service::new("Echo Service HTTP".to_string(), Arc::new(HttpEchoApp {}));
//   echo_service_http.add_tcp("0.0.0.0:5000");

//   my_server.add_service(echo_service_http);
//   my_server.run_forever();
// }
