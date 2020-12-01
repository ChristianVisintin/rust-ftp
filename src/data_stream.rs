use std::io::{Read, Write, Result};
use std::net::TcpStream;
#[cfg(feature = "openssl")]
use openssl::ssl::SslStream;
#[cfg(feature = "rust-tls")]
use rustls::ClientSession;

/// Data Stream used for communications
#[derive(Debug)]
pub enum DataStream {
    Tcp(TcpStream),
    #[cfg(feature = "openssl")]
    Ssl(SslStream<TcpStream>),
    #[cfg(feature = "rust-tls")]
    Ssl(rustls::Stream<'a, ClientSession, TcpStream>)
}

impl DataStream {
    /// Returns a reference to the underlying TcpStream.
    pub fn get_ref(&self) -> &TcpStream {
        match self {
            &DataStream::Tcp(ref stream) => stream,
            #[cfg(feature = "openssl")]
            &DataStream::Ssl(ref stream) => stream.get_ref(),
            #[cfg(feature = "rust-tls")]
            &DataStream::Ssl(ref stream) => stream.get_ref(),
        }
    }
}

#[cfg(feature = "openssl")]
impl DataStream {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStream {
        match self {
            DataStream::Tcp(stream) => stream,
            DataStream::Ssl(stream) => stream.get_ref().try_clone().unwrap(),
        }
    }

    /// Test if the stream is secured
    pub fn is_ssl(&self) -> bool {
        match self {
            &DataStream::Ssl(_) => true,
            _ => false
        }
    }
}

#[cfg(feature = "rust-tls")]
impl DataStream {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStream {
        match self {
            DataStream::Tcp(stream) => stream,
            DataStream::Ssl(stream) => stream.get_ref().try_clone().unwrap(),
        }
    }

    /// Test if the stream is secured
    pub fn is_ssl(&self) -> bool {
        match self {
            &DataStream::Ssl(_) => true,
            _ => false
        }
    }
}

impl Read for DataStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            &mut DataStream::Tcp(ref mut stream) => stream.read(buf),
            #[cfg(feature = "openssl")]
            &mut DataStream::Ssl(ref mut stream) => stream.read(buf),
            #[cfg(feature = "rust-tls")]
            &mut DataStream::Ssl(ref mut stream) => stream.read(buf),
        }
    }
}


impl Write for DataStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            &mut DataStream::Tcp(ref mut stream) => stream.write(buf),
            #[cfg(feature = "openssl")]
            &mut DataStream::Ssl(ref mut stream) => stream.write(buf),
            #[cfg(feature = "rust-tls")]
            &mut DataStream::Ssl(ref mut stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            &mut DataStream::Tcp(ref mut stream) => stream.flush(),
            #[cfg(feature = "openssl")]
            &mut DataStream::Ssl(ref mut stream) => stream.flush(),
            #[cfg(feature = "rust-tls")]
            &mut DataStream::Ssl(ref mut stream) => stream.flush(),
        }
    }
}
