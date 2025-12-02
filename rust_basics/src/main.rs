use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("servidor rodando na porta http://{}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Erro ao blindar a porta : {}", e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Erro no servidor: {}", e);
    }
}
