//! Debug middleware that prints debug info to stdout

use super::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Debug;

impl Middleware for Debug {
    fn before(&self, req: &mut Request) -> Result<(), Box<Error + Send>> {
        DebugRequest.before(req)
    }

    fn after(
        &self,
        _req: &mut Request,
        res: Result<Response, Box<Error + Send>>,
    ) -> Result<Response, Box<Error + Send>> {
        res.map(|res| {
            println!("  <- {:?}", res.status);
            for (k, v) in &res.headers {
                println!("  <- {} {:?}", k, v);
            }
            res
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DebugRequest;

impl Middleware for DebugRequest {
    fn before(&self, req: &mut Request) -> Result<(), Box<Error + Send>> {
        println!("  version: {}", req.http_version());
        println!("  method: {:?}", req.method());
        println!("  scheme: {:?}", req.scheme());
        println!("  host: {:?}", req.host());
        println!("  path: {}", req.path());
        println!("  query_string: {:?}", req.query_string());
        println!("  remote_addr: {:?}", req.remote_addr());
        for &(k, ref v) in &req.headers().all() {
            println!("  hdr: {}={:?}", k, v);
        }
        Ok(())
    }
}
