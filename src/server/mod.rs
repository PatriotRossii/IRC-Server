use crate::thread_pool::ThreadPool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, IpAddr};

extern crate num_cpus;

pub struct Server;

impl Server {
	pub fn new() -> Server {
		Server
	}
	pub fn listen(&self, ip: IpAddr, port: u16) -> std::io::Result<()> {
		let listener = TcpListener::bind((ip, port))?;
		let pool = ThreadPool::new(num_cpus::get());

		for stream in listener.incoming() {
			let stream = stream.unwrap();
			pool.execute(|| {
				Server::handle(stream);
			})

		}

		Ok(())

	}
	fn handle(mut stream: TcpStream) {
		let mut buffer = [0; 512];
		stream.read(&mut buffer).unwrap();

		stream.write("".as_bytes()).unwrap();
		stream.flush().unwrap();
	}
}