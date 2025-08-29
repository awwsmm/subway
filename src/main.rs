use salvo::catcher::Catcher;
use salvo::prelude::*;
use std::fs;
use std::time::SystemTime;

#[handler]
async fn hello(res: &mut Response) {
    let filename = "resources/hello.html";
    let contents = fs::read_to_string(filename).unwrap();
    res.render(Text::Html(contents))
}

// Added this /sleep test to this commit, to check the multithreaded nature of Salvo. To run, do
//
//   $ curl localhost:8989/sleep
//   Started at SystemTime { tv_sec: 1756476192, tv_nsec: 860726000 }, thought for 4812 ms
//
// Note that handling this request takes about 5 seconds. Submitting the same request N times takes
// significantly less than 5n seconds
//
//   # Fri Aug 29 | 15:05:26 in /Users/andrew/Git/subway
//   $ for req in {1..10}; do; curl localhost:8989/sleep &; done;
//   [2] 80320
//   [3] 80321
//   [4] 80322
//   [5] 80323
//   [6] 80324
//   [7] 80325
//   [8] 80326
//   [9] 80327
//   [10] 80328
//   [11] 80329
//
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 949078000 }, thought for 5870 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 952379000 }, thought for 5893 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 951715000 }, thought for 5953 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 948006000 }, thought for 5985 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 948628000 }, thought for 5986 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 947220000 }, thought for 5996 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 951425000 }, thought for 5995 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 951589000 }, thought for 6007 ms
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 949198000 }, thought for 6068 ms
//
//   # Fri Aug 29 | 15:05:26
//   # Fri Aug 29 | 15:05:32 in /Users/andrew/Git/subway
//   #                       on master*
//   $ Started at SystemTime { tv_sec: 1756476326, tv_nsec: 949026000 }, thought for 6079 ms
//
//   [5]  + 80323 done       curl localhost:8989/sleep

fn fibonacci(n: u64) -> u64 {
     match n {
         1 | 2 => 1,
         _ => fibonacci(n - 1) + fibonacci(n - 2),
     }
}

#[handler]
async fn sleep(res: &mut Response) {
    let now = SystemTime::now();
    fibonacci(45);
    res.render(format!("Started at {:?}, thought for {} ms\n", now, now.elapsed().unwrap().as_millis()))
}

#[handler]
async fn not_found(&self, _req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        let filename = "resources/404.html";
        let contents = fs::read_to_string(filename).unwrap();
        res.render(Text::Html(contents));

        // Skip remaining error handlers
        ctrl.skip_rest();
    }
}

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("0.0.0.0:8989").bind().await;

    let router = Router::new()
        .push(
            Router::with_path("hello").get(hello)
        )
        .push(
            Router::with_path("sleep").get(sleep)
        );

    let catcher = Catcher::default().hoop(not_found);

    Server::new(acceptor).serve(Service::new(router).catcher(catcher)).await;
}