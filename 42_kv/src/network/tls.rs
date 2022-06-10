use std::io::Cursor;
use std::sync::Arc;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::rustls::{internal::pemfile, Certificate, ClientConfig, ServerConfig};
use tokio_rustls::rustls::{AllowAnyAuthenticatedClient, NoClientAuth, PrivateKey, RootCertStore};
use tokio_rustls::webpki::DNSNameRef;
use tokio_rustls::TlsConnector;
use tokio_rustls::{
    client::TlsStream as ClientTlsStream, server::TlsStream as ServerTlsStream, TlsAcceptor,
};

use crate::KvError;

/// KV Server自己的ALPN（Application-Layer Protocol Negotiation）
const ALPN_KV: &str = "kv";

/// 存放TLS ServerConfig并提供方法accept，把底层协议转换成TLS
#[derive(Clone)]
pub struct TlsServerAcceptor {
    inner: Arc<ServerConfig>,
}

/// 存放TLS Client并提供方法connect把底层的协议转换成TLS
#[derive(Clone)]
pub struct TlsClientConnector {
    pub config: Arc<ClientConfig>,
    pub domain: Arc<String>,
}

impl TlsClientConnector {
    // 加载 client cert / CA cert,生成ClientConfig
    pub fn new(
        domain: impl Into<String>,
        identity: Option<(&str, &str)>,
        server_ca: Option<&str>,
    ) -> Result<Self, KvError> {
        let mut config = ClientConfig::new();

        // 如果有客户端证书，加载之
        if let Some((cert, key)) = identity {
            let certs = load_certs(cert)?;
            let key = load_key(key)?;
            config.set_single_client_cert(certs, key);
        }

        // 加载本地新人的根证书链
        config.root_store = match rustls_native_certs::load_native_certs() {
            Ok(store) | Err((Some(store), _)) => store,
            Err((None, error)) => return Err(error.into()),
        };

        // 如果有签署服务器的CA证书，则加载它，这样服务器证书不在根证书链
        // 但这个CA证书能验证它，也可以
        if let Some(cert) = server_ca {
            let mut buf = Cursor::new(cert);
            config.root_store.add_pem_file(&mut buf).unwrap();
        }

        Ok(Self {
            config: Arc::new(config),
            domain: Arc::new(domain.into()),
        })
    }

    /// 触发TLS协议，把底层的stream转换成TLS stream
    pub async fn connect<S>(&self, stream: S) -> Result<ClientTlsStream<S>, KvError>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        let dns = DNSNameRef::try_from_ascii_str(self.domain.as_str())
            .map_err(|_| KvError::Internal("Invalid DNS name".into()))?;

        let stream = TlsConnector::from(self.config.clone())
            .connect(dns, stream)
            .await?;

        Ok(stream)
    }
}

impl TlsServerAcceptor {
    /// 加载server cert / CA cert，生成ServerConfig
    pub fn new(cert: &str, key: &str, client_ca: Option<&str>) -> Result<Self, KvError> {
        let certs = load_certs(cert)?;
        let key = load_key(key)?;

        let mut config = match client_ca {
            None => ServerConfig::new(NoClientAuth::new()),
            Some(cert) => {
                // 如果客户端证书是某个CA证书签发的，则吧这个CA证书加载到信任链中
                let mut cert = Cursor::new(cert);
                let mut client_root_cert_store = RootCertStore::empty();
                client_root_cert_store
                    .add_pem_file(&mut cert)
                    .map_err(|_| KvError::CertificateParseError("CA", "cert"))?;

                let client_auth = AllowAnyAuthenticatedClient::new(client_root_cert_store);
                ServerConfig::new(client_auth)
            }
        };

        // 加载服务证书
        config
            .set_single_cert(certs, key)
            .map_err(|_| KvError::CertificateParseError("server", "cert"))?;
        config.set_protocols(&[Vec::from(&ALPN_KV[..])]);

        Ok(Self {
            inner: Arc::new(config),
        })
    }

    // 触发TLS协议，把底层stream 转换成TLS stream
    pub async fn accept<S>(&self, stream: S) -> Result<ServerTlsStream<S>, KvError>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        let acceptor = TlsAcceptor::from(self.inner.clone());
        Ok(acceptor.accept(stream).await?)
    }
}

fn load_certs(cert: &str) -> Result<Vec<Certificate>, KvError> {
    let mut cert = Cursor::new(cert);
    pemfile::certs(&mut cert).map_err(|_| KvError::CertificateParseError("server", "cert"))
}

fn load_key(key: &str) -> Result<PrivateKey, KvError> {
    let mut cursor = Cursor::new(key);

    //先尝试使用PKCS8 加载私钥
    if let Ok(mut keys) = pemfile::pkcs8_private_keys(&mut cursor) {
        if !keys.is_empty() {
            return Ok(keys.remove(0));
        }
    }

    // 再尝试使用RSA key
    if let Ok(mut keys) = pemfile::rsa_private_keys(&mut cursor) {
        if !keys.is_empty() {
            return Ok(keys.remove(0));
        }
    }

    // 不支持的私钥类型
    Err(KvError::CertificateParseError("private", "key"))
}

#[cfg(test)]
pub mod tls_utils {
    use crate::{KvError, TlsClientConnector, TlsServerAcceptor};

    const CA_CERT: &str = include_str!("../../fixtures/ca.cert");
    const CLIENT_CERT: &str = include_str!("../../fixtures/client.cert");
    const CLIENT_KEY: &str = include_str!("../../fixtures/client.key");
    const SERVER_CERT: &str = include_str!("../../fixtures/server.cert");
    const SERVER_KEY: &str = include_str!("../../fixtures/server.key");

    pub fn tls_connector(client_cert: bool) -> Result<TlsClientConnector, KvError> {
        let ca = Some(CA_CERT);
        let client_identity = Some((CLIENT_CERT, CLIENT_KEY));

        match client_cert {
            false => TlsClientConnector::new("kvserver.acme.inc", None, ca),
            true => TlsClientConnector::new("kvserver.acme.inc", client_identity, ca),
        }
    }

    pub fn tls_acceptor(client_cert: bool) -> Result<TlsServerAcceptor, KvError> {
        let ca = Some(CA_CERT);
        match client_cert {
            true => TlsServerAcceptor::new(SERVER_CERT, SERVER_KEY, ca),
            false => TlsServerAcceptor::new(SERVER_CERT, SERVER_KEY, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tls_utils::{tls_acceptor, tls_connector};

    use super::*;
    use anyhow::Result;
    use std::net::SocketAddr;
    use tokio::{
        io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
    };

    #[tokio::test]
    async fn tls_should_work() -> Result<()> {
        let addr = start_server(false).await?;
        let connector = tls_connector(false)?;
        let stream = TcpStream::connect(addr).await?;
        let mut stream = connector.connect(stream).await?;
        stream.write_all(b"hello world!").await?;
        let mut buf = [0; 12];
        stream.read_exact(&mut buf).await?;
        assert_eq!(&buf, b"hello world!");

        Ok(())
    }

    #[tokio::test]
    async fn tls_with_client_cert_should_work() -> Result<()> {
        let addr = start_server(true).await?;
        let connector = tls_connector(true)?;
        let stream = TcpStream::connect(addr).await?;
        let mut stream = connector.connect(stream).await?;
        stream.write_all(b"hello world!").await?;
        let mut buf = [0; 12];
        stream.read_exact(&mut buf).await?;
        assert_eq!(&buf, b"hello world!");

        Ok(())
    }

    #[tokio::test]
    async fn tls_with_bad_domain_should_not_work() -> Result<()> {
        let addr = start_server(false).await?;

        let mut connector = tls_connector(false)?;
        connector.domain = Arc::new("kvserver1.acme.inc".into());
        let stream = TcpStream::connect(addr).await?;
        let result = connector.connect(stream).await;

        assert!(result.is_err());

        Ok(())
    }

    async fn start_server(client_cert: bool) -> Result<SocketAddr> {
        let acceptor = tls_acceptor(client_cert)?;

        let echo = TcpListener::bind("127.0.0.1:9527").await.unwrap();
        let addr = echo.local_addr().unwrap();

        tokio::spawn(async move {
            let (stream, _) = echo.accept().await.unwrap();
            let mut stream = acceptor.accept(stream).await.unwrap();
            let mut buf = [0; 12];
            stream.read_exact(&mut buf).await.unwrap();
            stream.write_all(&buf).await.unwrap();
        });
        Ok(addr)
    }
}
