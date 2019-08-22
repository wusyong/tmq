use futures::Stream;
use zmq::{self, Context as ZmqContext, SocketType};

use crate::poll::EventedSocket;
use crate::Multipart;
use crate::Result;
use std::pin::Pin;
use std::task::{Context, Poll};

pub fn pull(context: &ZmqContext) -> PullBuilder {
    PullBuilder { context }
}

pub struct PullBuilder<'a> {
    context: &'a ZmqContext,
}

pub struct PullBuilderBounded {
    socket: zmq::Socket,
}

impl<'a> PullBuilder<'a> {
    pub fn connect(self, endpoint: &str) -> Result<PullBuilderBounded> {
        let socket = self.context.socket(SocketType::PULL)?;
        socket.connect(endpoint)?;

        Ok(PullBuilderBounded {
            socket: socket.into(),
        })
    }

    pub fn bind(self, endpoint: &str) -> Result<PullBuilderBounded> {
        let socket = self.context.socket(SocketType::PULL)?;
        socket.bind(endpoint)?;

        Ok(PullBuilderBounded {
            socket: socket.into(),
        })
    }
}

impl PullBuilderBounded {
    pub fn finish(self) -> Pull {
        Pull {
            socket: EventedSocket::from_zmq_socket(self.socket),
        }
    }
}

pub struct Pull {
    socket: EventedSocket,
}

impl Pull {
    pub fn get_socket(&self) -> &zmq::Socket {
        &self.socket.0.get_ref().socket
    }
}

impl_stream!(Pull, socket);