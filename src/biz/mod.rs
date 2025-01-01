use std::marker;

use peer_client::PeerClient;
use state::State;

pub mod peer_client;
pub mod state;

#[derive(Debug, Clone)]
pub struct Node<S: State, P: PeerClient> {
    _marker: marker::PhantomData<(S, P)>
}

impl <S: State, P: PeerClient>Node<S, P> {
    pub fn new() -> Self {
        Node { _marker: marker::PhantomData }
    }
}