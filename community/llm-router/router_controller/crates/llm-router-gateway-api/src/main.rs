//! Main
use clap::{arg, command, Parser};
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use llm_router_gateway_api::config::RouterConfig;
use llm_router_gateway_api::proxy::proxy;
use std::fs;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    config_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // cargo run -- --config foobar
    println!("Gateway API is active and running.");
    let args = Args::parse();

    let config_content = fs::read_to_string(args.config_path)?;
    let config: RouterConfig = serde_yaml::from_str(&config_content)?;

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8084));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8084));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let config_clone = config.clone();
        tokio::task::spawn(async move {
            if let Err(err) = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection(io, service_fn(move |req| proxy(req, config_clone.clone())))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[tokio::test]
    async fn test_main_does_not_have_unit_tests() {
        assert_eq!(2, 1 + 1);
    }
}
