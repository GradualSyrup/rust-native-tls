extern crate libc;
extern crate nnsdk;

use std::io;
use std::fmt;
use std::error;

use std::marker::PhantomData;

use self::nnsdk::nn;

use {TlsAcceptorBuilder, TlsConnectorBuilder};

mod Context {
    #[repr(C)]
    pub enum SslVersion {
        Auto = 0x1,
        Tls10 = 0x8,
        Tls11 = 0x10,
        Tls12 = 0x20,
    }

    // Estimate through SDK binary usage of the fields
    #[repr(C)]
    pub struct Context {
        _x: u64,
    }

    impl Context {
        pub fn new() -> Self {
            Self {
                _x: 0,
            }
        }
    }

    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7ContextC1Ev"]
        pub fn Context(this: *mut Context);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context6CreateENS1_10SslVersionE"]
        pub fn Create(this: *mut Context, version: SslVersion) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context15ImportClientPkiEPmPKcS4_jj"]
        pub fn ImportClientPki(this: *mut Context, out_store_id: &mut u64, p12_buf: *const u8, password_buf: *const u8, p12_buf_len: u32, password_buf_len: u32) -> i32;
    }
    
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl7Context9ImportCrlEPmPKcj"]
        pub fn ImportCrl(this: *mut Context, out_store_id: &mut u64, crl_der_buf: *const u8, crl_der_buf_len: u32) -> i32;
    }
}
// TODO: Move bindings in nnsdk-rs
mod Connection {
    // Estimate through SDK binary usage of the fields. Might be 0x28?
    #[repr(C)]
    pub struct Connection {
        _x: [u8;0x24],
    }

    impl Connection {
        pub fn new() -> Self {
            Self {
                _x: [0;0x24],
            }
        }
    }

    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10ConnectionC1Ev"]
        pub fn Connection(this: *mut Connection);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection6CreateEPNS0_7ContextE"]
        pub fn Create(this: *mut Connection, context: *const u8) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection19SetSocketDescriptorEi"]
        pub fn SetSocketDescriptor(this: *mut Connection, socket_desc: u32) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection11DoHandshakeEv"]
        pub fn DoHandshake(this: *mut Connection) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection4ReadEPcPij"]
        pub fn Read(this: *const Connection, out_buf: *mut u8, buf_len: usize) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection5WriteEPKcj"]
        pub fn Write(this: *const Connection, buf: *const u8, buf_len: usize) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection7PendingEv"]
        pub fn Pending(this: *const Connection) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection17FlushSessionCacheEv"]
        pub fn FlushSessionCache(this: *const Connection) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection7DestroyEv"]
        pub fn Destroy(this: *const Connection) -> u32;
    }
}

pub struct Error(io::Error);

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        unimplemented!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Identity;

impl Identity {
    pub fn from_pkcs8(pem: &[u8], key: &[u8]) -> Result<Identity, Error> {
        panic!("Not implemented on Nintendo Switch");
    }

    pub fn from_pkcs12(buf: &[u8], pass: &str) -> Result<Identity, Error> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Certificate;

impl Certificate {
    pub fn from_der(buf: &[u8]) -> Result<Certificate, Error> {
        unimplemented!()
    }

    pub fn from_pem(buf: &[u8]) -> Result<Certificate, Error> {
        // Since the Switch cannot do server-side SSL, we do not implement this
        panic!("Not implemented on Nintendo Switch");
    }

    pub fn to_der(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }
}

pub struct MidHandshakeTlsStream<S>(S);

impl<S> fmt::Debug for MidHandshakeTlsStream<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<S> MidHandshakeTlsStream<S> {
    pub fn get_ref(&self) -> &S {
        unimplemented!()
    }

    pub fn get_mut(&mut self) -> &mut S {
        unimplemented!()
    }
}

impl<S> MidHandshakeTlsStream<S>
where
    S: io::Read + io::Write,
{
    pub fn handshake(self) -> Result<TlsStream<S>, HandshakeError<S>> {
        unimplemented!()
    }
}

pub enum HandshakeError<S> {
    Failure(Error),
    WouldBlock(MidHandshakeTlsStream<S>),
}

impl<S> From<io::Error> for HandshakeError<S> {
    fn from(e: io::Error) -> HandshakeError<S> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct TlsConnector;

impl TlsConnector {
    pub fn new(builder: &TlsConnectorBuilder) -> Result<TlsConnector, Error> {
        Ok(TlsConnector)
    }

    pub fn connect<S>(&self, domain: &str, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        // The place where the TCP socket needs to be opened (use the domain for the address), connected then provided to the SSL library
        
        // TODO: Make sure nn::ssl::Initialize has been called before any of this

        //-----------------------------------------------
        // SOCKET
        //-----------------------------------------------
        // TODO: Protocol 6 is TCP. Make constants in nnsdk-rs to facilitate. Same goes for the libc values.
        let tcp_socket = unsafe { nn::socket::Socket(libc::AF_INET, libc::SOCK_STREAM, 6) };
        
        // TODO: Connect the socket to the domain
        // let result = unsafe { nn::socket::Connect(tcp_socket, libc::AF_INET, libc::SOCK_STREAM, 6) };

        println!("TlsConnector::connect: Successfully connected the socket");

        //-----------------------------------------------
        // CONTEXT
        //-----------------------------------------------
        // TODO: Prepare a nn::ssl::Context
        let mut context = Box::new(Context::Context::new());
        // Initialize the class before doing anything
        unsafe { Context::Context(context.as_mut()) };
        let result = unsafe { Connection::Create(context.as_mut(), Context::SslVersion::Auto) };

        println!("TlsConnector::connect: Successfully created the Context");

        //-----------------------------------------------
        // CONNECTION
        //-----------------------------------------------
        let mut connection = Box::new(Connection::Connection::new());
        // Initialize the class before doing anything
        unsafe { Connection::Connection(connection.as_mut()) };

        // TODO: Create the connection by providing it the context
        let result = unsafe { Connection::Create(connection.as_mut(), context.as_ref()) };

        println!("TlsConnector::connect: Successfully created the Connection");

        // Assign the socket to the Connection. After doing so, you musn't use it again or even free it.
        let result = unsafe { Connection::SetSocketDescriptor(connection.as_mut(), tcp_socket as _) };

        println!("TlsConnector::connect: Assigned the socket to the Connection");


        match unsafe { Connection::DoHandshake(connection.as_mut()) } {
            0 => {
            println!("TlsConnector::connect: Connection successfully performed Handshake");
                Ok(TlsStream {
                    connection,
                    _m: PhantomData,
                })
            }
            _ => Err(HandshakeError::Failure(Error(io::Error::new(io::ErrorKind::Other, "TlsConnector::connect: Handshake did not end successfully"))))
        }

    }
}

#[derive(Clone)]
pub struct TlsAcceptor;

impl TlsAcceptor {
    pub fn new(builder: &TlsAcceptorBuilder) -> Result<TlsAcceptor, Error> {
        // Since the Switch cannot do server-side SSL, we do not implement this
        panic!("Not implemented on Nintendo Switch");

    }

    pub fn accept<S>(&self, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        // Since the Switch cannot do server-side SSL, we do not implement this
        panic!("Not implemented on Nintendo Switch");

    }
}

pub struct TlsStream<S> {
    connection: Box<Connection::Connection>,
    _m: PhantomData<S>,
}

impl<S: fmt::Debug> fmt::Debug for TlsStream<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<S> TlsStream<S> {
    pub fn get_ref(&self) -> &S {
        unsafe { & *(self.connection.as_ref() as *const Connection::Connection as *const S) }
    }

    pub fn get_mut(&mut self) -> &mut S {
        unsafe { &mut *(self.connection.as_mut() as *mut Connection::Connection as *mut S) }
    }
}

impl<S: io::Read + io::Write> TlsStream<S> {
    pub fn buffered_read_size(&self) -> Result<usize, Error> {
        // Peek how many bytes are in the buffer
        let size = unsafe { Connection::Pending(self.connection.as_ref()) };
        Ok(size)
    }

    pub fn peer_certificate(&self) -> Result<Option<Certificate>, Error> {
        unimplemented!()
    }

    pub fn tls_server_end_point(&self) -> Result<Option<Vec<u8>>, Error> {
        // Since the Switch cannot do server-side SSL, we do not implement this
        Ok(None)
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        let result = unsafe { Connection::Destroy(self.connection.as_ref()) };
        // Should we take care of this for the user directly in the dependencies?
        // unsafe { nnsdk::ssl::Finalize(); }
        Ok(())
    }
}

impl<S: io::Read + io::Write> io::Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe { Connection::Read(self.connection.as_ref(), buf.as_mut_ptr(), buf.len()) };
        // TODO: If result is < 0, we have an error, deal with that
        Ok(result)
    }
}

impl<S: io::Read + io::Write> io::Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe { Connection::Write(self.connection.as_ref(), buf.as_ptr(), buf.len()) };
        // TODO: If result is < 0, we have an error, deal with that
        Ok(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = unsafe { Connection::FlushSessionCache(self.connection.as_ref()) };
        Ok(())
    }
}
