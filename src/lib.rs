use core::future::Future;
use futures::{Sink, Stream, TryFuture, TryStream};
use std::net::SocketAddr;

pub use url::Url;

pub trait Socket: Stream + TryStream<Ok = Vec<u8>> + Sink<Vec<u8>> {
    type Close: TryFuture<Ok = ()>;

    fn close(self) -> Self::Close;
}

pub trait SocketExt: Socket {
    type SendError;
    type ReceiveError;
    type CloseError;
}

impl<T: Socket> SocketExt for T {
    type SendError = <Self as Sink<Vec<u8>>>::Error;
    type ReceiveError = <Self as TryStream>::Error;
    type CloseError = <Self::Close as TryFuture>::Error;
}

pub trait SocketProvider {
    type Socket: Socket;
    type Connect: Future + TryFuture<Ok = Self::Socket>;

    fn connect(&self, url: Url) -> Self::Connect;
}

pub trait ServerProvider {
    type Listen: Stream + TryStream<Ok = Self::Socket>;
    type Socket: Socket;

    fn listen(&self, addr: SocketAddr) -> Self::Listen;
}

pub trait ServerProviderExt: ServerProvider {
    type ListenError;
}

impl<T: ServerProvider> ServerProviderExt for T {
    type ListenError = <Self::Listen as TryStream>::Error;
}

pub trait SocketProviderExt: SocketProvider {
    type ConnectError;
}

impl<T: SocketProvider> SocketProviderExt for T {
    type ConnectError = <Self::Connect as TryFuture>::Error;
}
