use interprocess::local_socket::LocalSocketListener;
use jki_core::{agent::{Request, Response}, paths::JkiPath};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread;

fn main() -> io::Result<()> {
    let socket_path = JkiPath::agent_socket_path();
    let name = socket_path.to_str().unwrap();

    // Remove existing socket file if it exists (for Unix)
    if socket_path.exists() && !cfg!(windows) {
        let _ = std::fs::remove_file(&socket_path);
    }

    let listener = LocalSocketListener::bind(name)?;
    println!("jki-agent listening on {:?}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(s) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(stream: interprocess::local_socket::LocalSocketStream) -> io::Result<()> {
    handle_client_io(stream)
}

fn handle_client_io<S: Read + Write>(stream: S) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        let _n = match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        if line.trim().is_empty() { continue; }

        let req: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response::Error(format!("Invalid request: {}", e));
                let mut s = reader.into_inner();
                s.write_all(format!("{}\n", serde_json::to_string(&resp).unwrap()).as_bytes())?;
                s.flush()?;
                reader = BufReader::new(s);
                continue;
            }
        };

        let resp = match req {
            Request::Ping => Response::Pong,
            Request::GetOTP { account_id } => {
                Response::OTP(format!("OTP-for-{}", account_id))
            }
        };

        let resp_json = serde_json::to_string(&resp).unwrap();
        let mut s = reader.into_inner();
        s.write_all(format!("{}\n", resp_json).as_bytes())?;
        s.flush()?;
        
        // Re-wrap for next iteration
        reader = BufReader::new(s);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct MockStream {
        input: Cursor<Vec<u8>>,
        output: Vec<u8>,
    }
    impl Read for MockStream {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.input.read(buf) }
    }
    impl Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.output.write(buf) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    #[test]
    fn test_handle_client_ping() {
        let req = Request::Ping;
        let mut input_data = serde_json::to_vec(&req).unwrap();
        input_data.push(b'\n');
        
        let mut stream = MockStream { input: Cursor::new(input_data), output: Vec::new() };
        handle_client_io(&mut stream).unwrap();

        let resp_str = String::from_utf8(stream.output).unwrap();
        let resp: Response = serde_json::from_str(&resp_str).unwrap();
        match resp {
            Response::Pong => {},
            _ => panic!("Expected Pong, got {:?}", resp),
        }
    }

    #[test]
    fn test_handle_client_malformed_json() {
        let input_data = b"not a json\n";
        let mut stream = MockStream { input: Cursor::new(input_data.to_vec()), output: Vec::new() };
        handle_client_io(&mut stream).unwrap();

        let resp_str = String::from_utf8(stream.output).unwrap();
        let resp: Response = serde_json::from_str(&resp_str).unwrap();
        match resp {
            Response::Error(msg) => assert!(msg.contains("Invalid request")),
            _ => panic!("Expected Error response, got {:?}", resp),
        }
    }
}
