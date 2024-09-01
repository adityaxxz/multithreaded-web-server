# Multi-Threaded Web Server

[/rust-book.cs.brown.edu](https://rust-book.cs.brown.edu/ch20-00-final-project-a-web-server.html)

A simple, multi-threaded web server implemented in Rust.

## Features

- Multi-threaded architecture using a thread pool
- TCP listener for handling incoming connections
- Serves static HTML files (index.html, 404.html)
- Basic HTTP response formatting

## Usage

1. Clone the repository
2. Run the server:

   ```
   cargo run
   ```
3. Access the server at `http://localhost:7878`

## Implementation Details

### Thread Pool

- Manages a fixed number of worker threads
- Distributes incoming requests across available threads

### TCP Listener

- Listens for incoming connections on port 7878
- Passes connections to the thread pool for processing

### Request Handling

- Parses incoming HTTP requests
- Serves `index.html` for root path ("/")
- Returns `404.html` for unrecognized paths


### Response Format

- HTTP-Version Status-Code Reason-Phrase CRLF  //example: HTTP/1.1 200 OK\r\n\r\n
- headers CRLF
- message-body

### Sleep Simulation

- Simulates a slow request by sleeping for 5 seconds when accessing "/sleep"

### Graceful Shutdown

- Implements the `Drop` trait for `ThreadPool`
- Sends termination messages to all workers
- Waits for workers to finish their current tasks before shutting down

## Performance Considerations

- Fixed thread pool size to prevent resource exhaustion
- Efficient handling of concurrent requests
- Potential for future improvements (e.g., connection pooling, caching)


## Files

- `src/main.rs`: Main server logic
- `src/lib.rs`: Thread pool implementation
- `index.html`: Home page
- `404.html`: Not Found page

## Contributing

Contributions are welcome! Please submit a pull request or create an issue for any bugs or feature requests.

## License

[MIT License](LICENSE)