use axum::{extract::State, routing::get};
use std::{error::Error, future::IntoFuture, net::SocketAddr};
mod greeter {
    tonic::include_proto!("greeter");
}
use argh::FromArgs;

use greeter::{
    greeter_client::GreeterClient,
    greeter_server::{Greeter, GreeterServer},
    GreetReply, GreetRequest,
};

#[derive(FromArgs, Clone)]
/// Configure bind ports and grpc destination.
struct Config {
    /// http port to bind to
    #[argh(option, short = 'h')]
    http_port: usize,

    /// how high to go
    #[argh(option, short = 'g')]
    grpc_port: usize,

    /// an optional nickname for the pilot
    #[argh(option, default = "\"localhost:50051\".into()")]
    grpc_dst: String,
}

#[derive(Default)]
struct GreeterStruct;

#[tonic::async_trait]
impl Greeter for GreeterStruct {
    async fn greet(
        &self,
        req: tonic::Request<GreetRequest>,
    ) -> Result<tonic::Response<GreetReply>, tonic::Status> {
        let name = req.into_inner().name;
        println!("In greet(), name: {}", name);

        let reply = GreetReply {
            answer: format!("Hello {name}!"),
        };

        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg: Config = argh::from_env();
    let greeter = GreeterServer::new(GreeterStruct);
    let http_addr: SocketAddr = format!("[::]:{}", cfg.http_port).parse()?;
    let grpc_addr: SocketAddr = format!("[::]:{}", cfg.grpc_port).parse()?;
    let web_app = axum::Router::new()
        .route("/", get(web_root))
        .with_state(cfg);
    let grpc_handle = tonic::transport::Server::builder()
        .add_service(greeter)
        .serve(grpc_addr);
    let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
    let axum_handle = axum::serve(listener, web_app.into_make_service()).into_future();

    log::info!("Serving grpc server {grpc_addr}");
    log::info!("Serving http server {http_addr}");
    tokio::select! {
        _ = grpc_handle=>{},
        _ = axum_handle=>{},
    }

    Ok(())
}

async fn web_root(State(cfg): State<Config>) -> Result<String, String> {
    let mut client = GreeterClient::connect(format!("http://{}", cfg.grpc_dst))
        .await
        .map_err(|_| "error connecting".to_owned())?;
    let request = tonic::Request::new(GreetRequest {
        name: "From Rust".into(),
    });
    let response = client
        .greet(request)
        .await
        .map_err(|_| "error calling".to_owned())?;
    Ok(format!("{:?}", response))
}
