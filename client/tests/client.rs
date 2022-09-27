//! Client tests

// use client::Client;
// use server::{Config, Server};

// /// Initializes a test
// fn init() -> Client {
//     /// Started tag
//     static STARTED: AtomicBool = AtomicBool::new(false);

//     let was_started = STARTED
//         .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
//         .unwrap();

//     if !was_started {
//         tokio::spawn(async {
//             let srv_cfg = Config::default();
//             let server = Server::new(srv_cfg);
//             server.start().await.unwrap();
//         });
//     }

//     Client::new("http://localhost:6666").unwrap()
// }

// #[tokio::test]
// async fn status() {
//     let client = init();
//     // let _status = client.status().await.unwrap();
// }
