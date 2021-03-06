extern crate opc;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;
extern crate rand;

use opc::{OpcCodec, Message, Command};
use futures::{stream, Future, Sink, future};

use tokio_io::AsyncRead;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

use std::io;
use std::time::Duration;


fn main() {

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let remote_addr = "192.168.1.230:7890".parse().unwrap();

    let work = TcpStream::connect(&remote_addr, &handle)
        .and_then(|socket| {

            let transport = socket.framed(OpcCodec);

            let messages = stream::unfold(vec![[0,0,0]; 1000], |mut pixels| {

                for pixel in pixels.iter_mut() {
                    for c in 0..2 {
                        pixel[c] = rand::random();
                    }
                };

                let pixel_msg = Message {
                    channel: 0,
                    command: Command::SetPixelColors { pixels: pixels.clone() }
                };

                std::thread::sleep(Duration::from_millis(100));

                Some(future::ok::<_,io::Error>((pixel_msg, pixels)))
            });

            transport.send_all(messages)

        });

    core.run(work).unwrap();
}
