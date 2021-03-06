mod datastore;

use kv::*;
use hyper::service::Service;
use hyper::{Body,Method,StatusCode,Request,Response,Server};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context,Poll};
use crate::datastore::KVStore;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Config::new("./test/example1");

    // Open the key/value store
    let store = Store::new(cfg)?;
    let kvstore: crate::datastore::KVStore = KVStore::new(store);
    
    let addr = ([127,0,0,1],3000).into();

    let server = Server::bind(&addr).serve(MakeSvc { datastore: kvstore });
    println!("Listening on http://{}",addr);

    server.await?;
    Ok(())
}

struct Svc {
    datastore: KVStore,
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(),Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        //fn mk_response(s: String) -> Result<Response<Body>,hyper::Error> {
        //    Ok(Response::builder().body(Body::from(s)).unwrap())
        //}

        let res = match (req.method(),req.uri().path()) {
            (&Method::GET,path) => {
                let str = String::from(format!("hi {}",path));
                Ok(Response::new(Body::from(str)))
            },
            (&Method::POST,_path) => {
                Ok(Response::new(Body::from("hi")))
            },
            (&Method::DELETE,_path) => {
                Ok(Response::new(Body::from("hi")))
            },
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        };

        Box::pin(async { res })
    }
}

struct MakeSvc {
    datastore: KVStore,
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self,_:&mut Context) -> Poll<Result<(),Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let datastore = self.datastore.clone();
        let fut = async move { Ok(Svc{datastore})};
        Box::pin(fut)
    }
}