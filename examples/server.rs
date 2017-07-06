extern crate h2;
extern crate http;
extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate io_dump;
extern crate env_logger;

use h2::server;

use http::{response, status};

use futures::*;

use tokio_core::reactor;
use tokio_core::net::TcpListener;

pub fn main() {
    let _ = env_logger::init();

    let mut core = reactor::Core::new().unwrap();;
    let handle = core.handle();

    let listener = TcpListener::bind(
        &"127.0.0.1:5928".parse().unwrap(),
        &handle).unwrap();

    println!("listening on {:?}", listener.local_addr());

    let server = listener.incoming().for_each(move |(socket, _)| {
        let socket = io_dump::Dump::to_stdout(socket);

        let connection = server::handshake(socket)
            .then(|res| {
                let conn = res.unwrap();

                println!("H2 connection bound");

                // Receive a request
                conn.into_future()
                    .then(|res| {
                        let (frame, conn) = res.unwrap();
                        println!("Zomg frame; {:?}", frame);

                        let mut response = response::Head::default();
                        response.status = status::NO_CONTENT;

                        conn.send_response(1, response, true)
                    })
            })
            .then(|res| {
                let _ = res.unwrap();
                Ok(())
            })
            ;

        handle.spawn(connection);
        Ok(())
    });

    core.run(server).unwrap();
}