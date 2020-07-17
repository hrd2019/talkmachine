use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Json;
use talkmachine::Message;
use crate::chat::eshandle::ESHandler;
use futures::executor::block_on;

pub mod chat {
    use std::net::ToSocketAddrs;

    use crate::chat::eshandle::ESHandler;
    use actix_web::web::Json;
    use actix_web::{HttpRequest, HttpResponse};
    use futures::executor::block_on;
    use futures::Future;
    use talkmachine::Message;
    use crate::chat::xx;

    pub struct Chat {
        pub pull: fn(msg: Json<Message>) -> HttpResponse,
        pub push: fn(msg: Json<Message>) -> HttpResponse,
    }

    impl Chat {
        pub fn new() -> Chat {
            let pull = |req: HttpRequest, msg: Json<Message>| {
                println!("xxx: {:#?}", req);
                println!("{}", &msg.index);
                let es = ESHandler::new();
                // es.push(&msg);
                let future = es.push(&msg);
                // block the current thread until provided future
                block_on(future);

                HttpResponse::Ok().body(&msg.data)
            };

            let push = |msg: Json<Message>| HttpResponse::Ok().body(&msg.data);

            Chat {
                pull: xx,
                push: push,
            }
        }
    }
}

fn xx(msg: Json<Message>) -> HttpResponse {
    println!("{}", &msg.index);

    // async {
    //     let es = ESHandler::new();
    //     // es.push(&msg);
    //     let future = es.push(&msg).await;
    //     println!("--------------------------------------------{}", future);
    // };
    let es = ESHandler::new();
    eprintln!("{}", es.test(&msg));

    HttpResponse::Ok().body(&msg.data)
}

pub mod eshandle {
    use actix_web::body::Body;
    use actix_web::HttpResponse;
    use elasticsearch::auth::Credentials;
    use elasticsearch::http::transport::Transport;
    use elasticsearch::indices::IndicesExistsParts::Index;
    use elasticsearch::{Elasticsearch, IndexParts, CreateParts};
    use talkmachine::Message;
    use elasticsearch::CreateParts::IndexId;
    use serde_json::{json, Value};
    use futures::executor::block_on;

    pub struct ESHandler {
        es: Elasticsearch,
    }

    impl ESHandler {
        pub fn new() -> ESHandler {
            let credentials = Credentials::Basic("es".into(), "%^CV90)sd138~".into());
            let res = Transport::single_node("http://182.16.39.218:9200");
            match res {
                Ok(transport) => {
                    let client = Elasticsearch::new(transport);

                    ESHandler { es: client }
                }
                Err(e) => panic!(e.to_string()),
            }
        }

        pub async fn push(&self, data: &Message) -> u64 {
            let index = [data.index.trim_start()];
            println!("========={}", data.index);
            let res = self
                .es
                .indices()
                .exists(Index(&index))
                .ignore_unavailable(true)
                .send()
                .await;
            match res {
                Ok(t) => println!("{}", t.status_code().is_success()),
                Err(e) => println!("{}", e),
            }

            100
        }

        pub fn test(&self, data: &Message) -> u64 {
            let index = [data.index.trim_start()];
            async {
                let response = self.es.index(IndexParts::Index("chat"))
                    .body(json!(
    {
    "roomid":121,
    "userid":1,
    "index":"chatx",
    "msg":"hello, ddd",
    "touserid":2,
    "flag":1
}
    )).send().await;
                match response {
                    Ok(test) => println!("{}", test.status_code()),

                    _ => {}
                }
            };

            // async {
            //     println!("{:#?}", self.es.info());
            //     let res = self
            //         .es
            //         .indices()
            //         .exists(Index(&index))
            //         .ignore_unavailable(true)
            //         .send().await;
            //     match res {
            //         Ok(t) => println!("{}", t.status_code().is_success()),
            //         Err(e) => println!("{}", e),
            //         _ => {}
            //     }
            // };

            100
        }
    }
}
