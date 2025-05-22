pub fn run() {
    let listener = TcpListener::bind("192.168.1.5:8080")
        .unwrap_or_else(|_| panic!("Cannot bind to 192.168.1.5:25521"));
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let requset = BufReader::new(&stream).lines().next().unwrap().unwrap();

        let response = match requset.split_whitespace().nth(1).unwrap().trim() {
            "/" => {
                let files = fs::read_dir("res")
                    .unwrap()
                    .map(|inner| inner.unwrap().file_name().to_string_lossy().to_string())
                    .collect::<Vec<String>>();
                let contents = include_str!("../index.html")
                    .lines()
                    .map(|line| {
                        let line = line.trim();
                        if line == "<body>" {
                            let mut line = String::from("<body><ul>");
                            for file in files.clone() {
                                line.push_str(&format!(
                                    "<li><a href=\"./res/{}\">{}</a></li>",
                                    &file, &file
                                ));
                            }
                            line.push_str("</ul>");
                            line
                        } else {
                            String::from(line)
                        }
                    })
                    .collect::<String>();
                let length = contents.len();
                let mut response = Vec::new();
                response.extend_from_slice(
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}")
                        .as_bytes(),
                );
                response
            }
            url => {
                let mut response = Vec::new();

                if url.contains("/res/") {
                    let file = fs::read(format!("./{}", url)).unwrap();
                    let length = file.len();
                    response.extend_from_slice(
                        format!("HTTP/1.1 200 Ok FOUND\r\nContent-Length: {length}\r\n\r\n")
                            .as_bytes(),
                    );
                    response.extend_from_slice(&file);
                    response
                } else {
                    let contents = include_str!("../404.html");
                    let length = contents.len();

                    response.extend_from_slice(format!(
                    "HTTP/1.1 404 ERROENOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}"
                ).as_bytes());
                    response
                }
            }
            _ => Vec::new(),
        }; 
        stream.write_all(&response).unwrap();
    }
}
