use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use regex::{Regex, Captures};
use std::fs;
use std::thread;

fn main() {
    // 绑定本地的8080端口
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 监听TCP请求
    for stream in listener.incoming()  {
        let stream = stream.unwrap();
        thread::spawn(||{ //创建线程处理TCP请求
            handle_request(stream);
        });
        
    }

}
/*
* GET为Http的get请求
* POST为Http的post请求
* ERROR 为错误
* COMMON为非Http请求
*/ 
enum RequestMode {
    GET(String), 
    POST(String),
    ERROR,
    COMMON
}

/**
 * 处理客户端发送过来的请求
 */
fn handle_request(mut stream: TcpStream){
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let request_content = String::from_utf8_lossy(&buffer[..]);
    println!("Request:{}", request_content);

    let mode = analyze(&buffer[..]);
    let reponse_str = match mode {
        RequestMode::GET(path) => { // 根据GET请求中Url的path，查找对应的文件,目前该demo只支持浏览器访问/index.html和404.html
            let file_path = format!("{}.html", path.replace("/", ""));
            println!("file_path:{}", file_path);
            let file = fs::read_to_string(file_path);  //加载对应的页面数据
            let response = match file {
                Ok(str) => format!("HTTP/1.1 200 OK\r\n\r\n{}",str),
                Err(e) => format!("HTTP/1.1 200 OK\r\n\r\n{}",fs::read_to_string("404.html").unwrap()),
            };
            // println!("reponse:{}", response);
            response
        },

        RequestMode::POST(path) => { // 根据POST请求中Url的path，查找对应的文件,目前该demo只支持浏览器访问/index.html和404.html
            let file_path = format!("{}.html", path.replace("/", ""));
            let file = fs::read_to_string(file_path); //加载对应的页面数据
            let response = match file {
                Ok(str) => format!("HTTP/1.1 200 OK\r\n\r\n{}",str),
                Err(e) => format!("HTTP/1.1 200 OK\r\n\r\n{}",fs::read_to_string("404.html").unwrap()),
            };
            response
        },

        RequestMode::ERROR => format!("HTTP/1.1 200 OK\r\n\r\n{}",fs::read_to_string("404.html").unwrap()), //错误异常，默认返回404.html
        RequestMode::COMMON => format!("{} from server \n",String::from_utf8_lossy(&buffer[..]))//错误异常，默认返回404.html
    };

    stream.write(reponse_str.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/*
 * 分析请求数据 
 */
fn analyze(buf: &[u8]) -> RequestMode{
    let request = String::from_utf8_lossy(buf);
    println!("analyze:{}", request);
  
    if request.lines().count() == 0 { //如果请求数据为空，则返回ERROR
        println!("请求数据为空");
        return  RequestMode::ERROR
    }

    let line_str = request.lines().next().unwrap();
    println!("line_str:{}",line_str);
    if let Option::Some(path) = is_get(line_str) {
        return RequestMode::GET(path);
    }

    if let Option::Some(path) = is_post(line_str) {
        return RequestMode::POST(path);
    }

    RequestMode::COMMON
}

/**
 * 判断是否为get请求
 */
fn is_get(url: &str) -> Option<String>{
    //通过正则表达式做个基本判断
    let regex = Regex::new(r"GET ([/\w]*) HTTP/1.1").unwrap();
    
    if regex.is_match(url) {
        let caps = regex.captures(url).unwrap();
        println!("get url:{}", caps.get(1).unwrap().as_str());
        Option::Some(caps.get(1).unwrap().as_str().to_string())
    } else {
        Option::None
    }
}

/**
 * 判断是否为get请求
 */
fn is_post(url: &str) -> Option<String>{
    //通过正则表达式做个基本判断
    let regex = Regex::new(r"POST ([/\w]*) HTTP/1.1").unwrap();
  
    if regex.is_match(url) {
        let caps = regex.captures(url).unwrap();
        println!("get url:{}", caps.get(1).unwrap().as_str());
        Option::Some(caps.get(1).unwrap().as_str().to_string())
    } else {
        Option::None
    }
}

//以下为单元测试

#[test]
fn  test_get() {
    assert_eq!(is_get("GET / HTTP/1.1"), Some("/".to_string()));
    assert_eq!(is_get("GET /123 HTTP/1.1"), Some("/123".to_string()));
    assert_eq!(is_get("GET /index/123 HTTP/1.1"), Some("/index/123".to_string()));
}

#[test]
fn  test_post() {
    assert_eq!(is_post("POST / HTTP/1.1"), Some("/".to_string()));
    assert_eq!(is_post("POST /123 HTTP/1.1"), Some("/123".to_string()));
    assert_eq!(is_post("POST /index/123 HTTP/1.1"), Some("/index/123".to_string()));
}