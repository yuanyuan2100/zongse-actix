// use std::{io::Cursor,
//         sync::atomic::{AtomicUsize, Ordering}};
// use rocket::{{Request, Data, Response},
//             fairing::{Fairing, Info, Kind},
//             http::{Method, ContentType, Status}};

// pub struct Token(i64);

// #[derive(Default)]
// pub struct Counter {
//     get: AtomicUsize,
//     post: AtomicUsize,
// }

// impl Fairing for Counter {
//     fn info(&self) -> Info {
//         Info {
//             name: "GET/POST Counter",
//             kind: Kind::Request | Kind::Response
//         }
//     }

//     fn on_request(&self, request: &mut Request<'_>, _: &Data) {
//         if request.method() == Method::Get {
//             self.get.fetch_add(1, Ordering::Relaxed);
//         } else if request.method() == Method::Post {
//             self.post.fetch_add(1, Ordering::Relaxed);
//         }
//         println!("{:?}", &self.get);
//     }

//     fn on_response(&self, request: &Request<'_>, response: &mut Response<'_>) {
//         if response.status() != Status::NotFound {
//             return
//         }

//         if request.method() == Method::Get && request.uri().path() == "/counts" {
//             let get_count = self.get.load(Ordering::Relaxed);
//             let post_count = self.post.load(Ordering::Relaxed);

//             let body = format!("Get: {}\nPost: {}", get_count, post_count);
//             response.set_status(Status::Ok);
//             response.set_header(ContentType::Plain);
//             response.set_sized_body(Cursor::new(body));
//         }
//     }
// }