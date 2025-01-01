//! HTTP server that handles requests from the outside world.

use std::net::SocketAddr;

use axum::{
    extract::{Extension, Path, Query}, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::{get, post}, 
    Json, 
    Router, 
    Server
};
use dto::{GetBlocksReq, TxReq, VersionReq};
use log::info;

use crate::biz::{peer_client::PeerClient, state::State, Node};

pub mod dto;

pub async fn run<S: State, P: PeerClient>(addr: SocketAddr, node: Node<S, P>) {
    let router = new_router(node);

    info!("HTTP server listening on {addr}");

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to run http server");
}

pub fn new_router<S: State, P: PeerClient>(node: Node<S, P>) -> Router {
    Router::new()
        .route("/blocks", get(get_blocks::<S, P>))
        .route("/blocks/:height", get(get_block::<S, P>))
        .route("/balances", get(get_balances::<S, P>))
        .route("/account/version", get(get_account_version::<S, P>))
        .route("/transfer", post(transfer::<S, P>))
        .fallback(not_found)
        .layer(Extension(node))
}

async fn get_blocks<S: State, P: PeerClient>(
    Extension(node): Extension<Node<S, P>>,
    Query(params) : Query<GetBlocksReq>,
) -> impl IntoResponse {
    todo!()
}

async fn get_block<S: State, P: PeerClient>(
    Extension(_node): Extension<Node<S, P>>,
    Path(height): Path<u64>,
) -> impl IntoResponse {
    todo!()
}

async fn get_balances<S: State, P: PeerClient>(
    Extension(_node): Extension<Node<S, P>>,
) -> impl IntoResponse {
    todo!()
}

async fn get_account_version<S: State, P: PeerClient>(
    Extension(_node): Extension<Node<S, P>>,
    Query(_params): Query<VersionReq>
) -> impl IntoResponse {
    todo!()
}

async fn transfer<S: State, P: PeerClient>(
    Extension(_node): Extension<Node<S, P>>,
    Json(_tx): Json<TxReq>
) -> impl IntoResponse {
    todo!()
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}