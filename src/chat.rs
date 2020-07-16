pub mod chat {
    use actix_web::web::Json;
    use actix_web::{HttpRequest, HttpResponse};
    use talkmachine::Message;

    pub struct Chat {
        pub pull: fn(req: HttpRequest, msg: Json<Message>) -> HttpResponse,
        pub push: fn(msg: Json<Message>) -> HttpResponse,
    }

    impl Chat {
        pub fn new() -> Chat {
            let pull = |req: HttpRequest, msg: Json<Message>| {
                println!("REQ: {:#?}", req);
                HttpResponse::Ok().body(&msg.data)
            };

            let push = |msg: Json<Message>| HttpResponse::Ok().body(&msg.data);

            Chat {
                pull: pull,
                push: push,
            }
        }
    }
}
