use crate::{HttpBody, HttpError, HttpRequest, HttpResponse};
use std::{io::ErrorKind, process::exit};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Interest, Result as IoResult},
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpListener, TcpStream, ToSocketAddrs,
    },
};

const READ_REQUEST_BUFFER_SIZE: usize = 0xff;

pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub async fn new(addr: impl ToSocketAddrs) -> IoResult<HttpServer> {
        Ok(HttpServer {
            listener: TcpListener::bind(addr).await?,
        })
    }

    pub async fn init() -> HttpServer {
        const PORT: u16 = 8080;

        match HttpServer::new(("127.0.0.1", PORT)).await {
            Ok(server) => {
                println!("Server running on http://localhost:{PORT}");
                server
            }
            Err(error) if error.kind() == ErrorKind::PermissionDenied => {
                println!("Port {PORT} is already in use!\nShutting Down....");
                exit(-1);
            }
            Err(error) => {
                panic!("Unknown Error Occurred: {error}");
            }
        }
    }

    pub async fn listen(&self) {
        loop {
            let (mut stream, _) = match self.listener.accept().await {
                Ok(peer) => peer,
                Err(error) => {
                    println!("Failed to connect due to an error: {error}");
                    continue;
                }
            };

            tokio::spawn(async move {
                match HttpServer::handle_request(&mut stream).await {
                    Ok(response) => response,
                    Err(error) => HttpServer::handle_error(&mut stream, error).await,
                }
            });
        }
    }

    async fn handle_error(client: &mut TcpStream, error: HttpError) {
        let response = error.as_response();
        let response_bytes = response.as_bytes();

        client.write(&response_bytes).await.unwrap_or_else(|error| {
            println!("Failed to Write response due to error: {error} \n{response}");
            0
        });
    }

    async fn handle_request(client: &mut TcpStream) -> Result<(), HttpError> {
        let (mut stream_reader, mut stream_writer) = client.split();

        while stream_reader.ready(Interest::READABLE).await?.is_readable() {
            let request_bytes = HttpServer::read_request(&mut stream_reader).await?;

            let mut request = HttpRequest::new(request_bytes)?;

            if let Some(content_length) = request.get_content_length() {
                // Reading Body
                let body = HttpServer::read_body(&mut stream_reader, content_length).await?;
                request.set_body(body);
            }

            HttpServer::respond(&mut stream_writer).await?;

            let keep_alive = request
                .get_header("Connection")
                .map_or(false, |header| header.value == "keep-alive");

            if !keep_alive {
                break;
            }
        }

        client.shutdown().await?;

        Ok(())
    }

    async fn respond(stream_writer: &mut WriteHalf<'_>) -> IoResult<usize> {
        let res = HttpResponse::new(200, None, None);

        Ok(stream_writer.write(&res.as_bytes()).await?)
    }

    async fn read_request(stream_reader: &mut ReadHalf<'_>) -> IoResult<Vec<u8>> {
        let mut read_buffer = [0; READ_REQUEST_BUFFER_SIZE];

        let mut result_buffer = vec![];

        loop {
            let read_bytes_count = stream_reader.peek(&mut read_buffer).await?; // Do not remove from TCP Buffer

            // Actually Remove from TCP buffer now
            if let Some(eof_index) = HttpServer::find_eof(&read_buffer) {
                stream_reader.read(&mut read_buffer[..eof_index]).await?;
                result_buffer.extend(&read_buffer[..eof_index]);
                stream_reader.read(&mut [0; 4]).await?; // Clear EOF
                return Ok(result_buffer);
            } else {
                stream_reader.read(&mut read_buffer).await?;
                result_buffer.extend(&read_buffer[..read_bytes_count]);
            }
        }
    }

    fn find_eof(buffer: &[u8; READ_REQUEST_BUFFER_SIZE]) -> Option<usize> {
        buffer
            .windows(4)
            .position(|window| window == &[13, 10, 13, 10])
    }

    async fn read_body(
        stream_reader: &mut ReadHalf<'_>,
        content_length: usize,
    ) -> IoResult<HttpBody> {
        let mut body_bytes = vec![0; content_length];
        let mut read_bytes_total = 0;
        loop {
            let read_bytes_count = stream_reader
                .read(&mut body_bytes[read_bytes_total..])
                .await?;

            read_bytes_total += read_bytes_count;

            if read_bytes_total >= content_length {
                return Ok(HttpBody::new(body_bytes));
            }
        }
    }
}
