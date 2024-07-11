use std::io::{Read, Write};
use std::net::TcpStream;

fn http_get(url: &str) -> Result<String, std::io::Error> {
    // Parse the URL to get host and path
    let url = url
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    println!("url = {}", url);

    //For url = "http://example.com/path/to/resource", url.find('/') would return Some(7), 
    //because '/' is found at index 7.

    //Therefore, (url[..7].to_string(), &url[7..]) becomes (url[..7].to_string(), 
    //"://example.com/path/to/resource").

    //url[..7] gives "http://", which is converted to a String.

    //&url[7..] gives "://example.com/path/to/resource", which is a string slice.

    let (host, path) = match url.find('/') {
        Some(idx) => (&url[..idx], &url[idx..]),
        None => (url, "/"),
    };

    println!("host = {:?}", host);

    // Establish a TCP connection to the host
    let mut stream = TcpStream::connect(format!("{}:80", host))?;

    // Prepare the HTTP request with correct headers
    let user_agent = "curl/7.81.0"; // Use a standard User-Agent header
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nAccept: */*\r\nConnection: close\r\n\r\n",
        path, host, user_agent
    );

    // Send the HTTP request
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Check HTTP status code in response
    let status_line = response.lines().next().unwrap_or("");
    if !status_line.starts_with("HTTP/1.0 200") && !status_line.starts_with("HTTP/1.1 200") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unexpected status: {}", status_line),
        ));
    }

    // Close the TCP connection explicitly (optional, since `stream` will be dropped)
    drop(stream);

    // Return the response
    Ok(response)
}

fn main() {
    let url = "https://books.toscrape.com/catalogue/category/books/sequential-art_5/index.html";
    match http_get(url) {
        Ok(response) => println!("Response:\n{}", &response[..50]),
        Err(e) => eprintln!("Error: {}", e),
    }
}
