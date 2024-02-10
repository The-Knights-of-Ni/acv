use imageproc::definitions::Image;
use image::{EncodableLayout, Rgb};
use log::error;

pub trait Output {
    fn output(&mut self, image: crate::Result<Option<Image<Rgb<u8>>>>) -> crate::Result<()>;
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct NoOutput;

impl Output for NoOutput {
    fn output(&mut self, _: crate::Result<Option<Image<Rgb<u8>>>>) -> crate::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "output-unix-socket")]
pub struct SocketOutput {
    socket: std::net::UdpSocket,
    address: std::net::SocketAddr,
}

#[cfg(feature = "output-unix-socket")]
impl SocketOutput {
    pub fn new(address: &str) -> crate::Result<Self> {
        let socket = std::net::UdpSocket::bind(address)?;
        let address = socket.local_addr()?;
        Ok(SocketOutput { socket, address })
    }
}

#[cfg(feature = "output-unix-socket")]
impl Output for SocketOutput {
    fn output(&mut self, image: crate::Result<Option<Image<Rgb<u8>>>>) -> crate::Result<()> {
        match image {
            Ok(Some(image)) => {
                let mut buf: Vec<u8> = Vec::new();
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 75); // TODO: add quality setting and encoder type
                let res = encoder.encode(&image, image.width(), image.height(), image.color());
                if res.is_ok() {
                    let res = self.socket.send_to([0, &buf].concat(), self.address);
                    if res.is_err() {
                        error!("Failed to send image to socket");
                    }
                }
            }
            Ok(None) => {
                let res = self.socket.send_to(&[110, 117, 108, 108], self.address);
                if res.is_err() {
                    error!("Failed to send image to socket");
                }
            }
            Err(e) => {
                let message = format!("{}", e);
                let prefix = [83_u8, 79, 83];
                let bytes = message.as_bytes();
                let res = self.socket.send_to([prefix.as_bytes(), bytes].concat(), self.address);
                if res.is_err() {
                    error!("Failed to send image to socket");
                }
            }
        }
        Ok(())
    }
}

