use imageproc::definitions::Image;
use image::{ColorType, EncodableLayout, Rgb};
use log::error;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;

pub trait Output {
    fn output(&mut self, image: crate::Result<Option<Image<Rgb<u8>>>>, color_type: ColorType) -> crate::Result<()>;
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct NoOutput;

impl Output for NoOutput {
    fn output(&mut self, _: crate::Result<Option<Image<Rgb<u8>>>>, _: ColorType) -> crate::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "output-udp")]
pub struct UdpOutput {
    socket: tokio::net::UdpSocket,
}

#[cfg(feature = "output-udp")]
impl UdpOutput {
    pub async fn new<A: ToSocketAddrs>(address: A, target: A) -> crate::Result<Self> {
        let socket = tokio::net::UdpSocket::bind(address).await?;
        socket.connect(target).await?;
        Ok(Self { socket })
    }

    pub fn from_socket(socket: tokio::net::UdpSocket) -> Self {
        Self { socket }
    }
}

#[cfg(feature = "output-udp")]
impl Output for UdpOutput {
    // TODO: don't use write_all
    fn output(&mut self, image: crate::Result<Option<Image<Rgb<u8>>>>, color_type: ColorType) -> crate::Result<()> {
        match image {
            Ok(Some(image)) => {
                let mut buf: Vec<u8> = Vec::new();
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 75); // TODO: add quality setting and encoder type
                let res = encoder.encode(&image, image.width(), image.height(), color_type);
                if res.is_ok() {
                    let prefix = [0_u8, 0, 0, 0];
                    self.socket.send(&([&prefix, buf.as_slice()].concat())).await?;
                }
            }
            Ok(None) => {
                self.socket.send(&[110, 117, 108, 108]).await?;
            }
            Err(e) => {
                let message = format!("{}", e);
                let prefix = [83_u8, 79, 83];
                let bytes = message.as_bytes();
                self.socket.send(&[prefix.as_bytes(), bytes].concat()).await?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "output-unix-stream")]
pub struct StreamOutput {
    socket: tokio::net::UnixStream,
}

#[cfg(feature = "output-unix-stream")]
impl StreamOutput {
    pub async fn new(address: &str) -> crate::Result<Self> {
        let socket = tokio::net::UnixStream::connect(address).await?;
        Ok(Self { socket })
    }

    pub fn from_socket(socket: tokio::net::UnixStream) -> crate::Result<Self> {
        Ok(Self { socket })
    }
}

#[cfg(feature = "output-unix-stream")]
impl Output for StreamOutput {
    fn output(&mut self, image: crate::Result<Option<Image<Rgb<u8>>>>, color_type: ColorType) -> crate::Result<()> {
        match image {
            Ok(Some(image)) => {
                let mut buf: Vec<u8> = Vec::new();
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 75); // TODO: add quality setting and encoder type
                let res = encoder.encode(&image, image.width(), image.height(), color_type);
                if res.is_ok() {
                    let prefix = [0_u8, 0, 0, 0];
                    let res = self.socket.write_all(&([&prefix, buf.as_slice()].concat()));
                }
            }
            Ok(None) => {
                let res = self.socket.write_all(&[110, 117, 108, 108]);
            }
            Err(e) => {
                let message = format!("{}", e);
                let prefix = [83_u8, 79, 83];
                let bytes = message.as_bytes();
                let res = self.socket.write_all(&[prefix.as_bytes(), bytes].concat());
            }
        }
        Ok(())
    }
}

