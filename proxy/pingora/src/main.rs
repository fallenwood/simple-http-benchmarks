use async_trait::async_trait;
use http::header;
use pingora::prelude::*;
use std::sync::Arc;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let parts: Vec<_> = session.req_header().headers.get(header::HOST).unwrap().to_str().unwrap().split(':').collect();
        let host = parts[0];

        // println!("host: {}", host);

        if host != "pingora.example" {
            let _ = session.respond_error(404).await;

            // true: early return as the response is already written
            return Ok(true);
        }

        Ok(false)
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        let upstream = self.0
            .select(b"", 256)
            .unwrap();

        let peer = Box::new(HttpPeer::new(upstream, false, "pingora.example".to_string()));

        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request.insert_header("Host", "pingora.example").unwrap();

        Ok(())
    }
}

fn main() {
    let config = "./config.yaml".to_string();
    let mut my_server = Server::new(Some(Opt {
        conf: Some(config),
        upgrade: false,
        daemon: false,
        nocapture: false,
        test: false,
    })).unwrap();

    println!("threads: {}", my_server.configuration.threads);

    my_server.bootstrap();

    let upstreams =
        LoadBalancer::try_from_iter(["127.0.0.1:5000"]).unwrap();

    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));

    lb.add_tcp("0.0.0.0:8080");

    my_server.add_service(lb);

    my_server.run_forever();
}