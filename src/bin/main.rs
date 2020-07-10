use IRCServer::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

extern crate num_cpus;

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:6667")?;
	let pool = ThreadPool::new(num_cpus::get());

	for stream in listener.incoming() {

	}

	Ok(())
}