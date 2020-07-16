use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Room {
    id: u64,
    name: String,
    //status 0:invalid 1:valid,
    status: u8,
    //kind 0:sys 1:pub 2:private,
    kind: u8,
    adminid: u64,
    cts: SystemTime,
    //max contain max number,
    max: u8,
    //forbidword 0:no 1:yes
    forbidword: u8,
}

impl Room {
    fn new(name: String, max: u8) -> Room {
        Room {
            id: 0,
            name: "".to_string(),
            status: 1,
            kind: 1,
            adminid: 0,
            cts: SystemTime::now(),
            max,
            forbidword: 0,
        }
    }
}

pub struct Chater {
    id: u64,
    nickname: String,
    friends: Vec<u64>,
    rooms: Vec<Room>,
    //forbid_word 0:no 1:yes,
    forbid_word: u8,
    //circle <housename,vec<chater>>
    circle: HashMap<u64, Vec<u64>>,
}

impl Chater {
    pub fn new(nickname: String) -> Chater {
        Chater {
            id: 0,
            nickname,
            forbid_word: 0,
            friends: Default::default(),
            rooms: vec![],
            circle: Default::default(),
        }
    }

    pub fn add_friend(&mut self, userid: u64) -> bool {
        self.friends.push(userid);
        true
    }

    fn subt_friend(&mut self, userid: u64) -> bool {
        for (index, uid) in self.friends.iter().enumerate() {
            if *uid == userid {
                self.friends.remove(index);
                break;
            }
        }

        true
    }

    fn add_circle(&mut self, roomid: u64, userid: u64) {
        let mut circle = self.circle.entry(roomid).or_default();
        circle.push(userid);
    }

    fn subt_circle(&mut self, roomid: u64, userid: u64) {
        let mut circle = self.circle.entry(roomid).or_default();
        for (index, uid) in circle.iter().enumerate() {
            if *uid == userid {
                circle.remove(index);
                break;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub index: String,
    // userid: u64,
    // roomid: u64,
    // touserid: u64,
    pub data: String,
}

// pub mod chat {
//     use crate::Message;
//     use actix_web::web::Json;
//     use actix_web::{HttpResponse, Responder};
//
//     pub struct Chat();
//
//     impl Chat {
//         #[post("/chat/pull")]
//         pub async fn pull(msg: Json<Message>) -> impl Responder {
//             HttpResponse::Ok().body(&msg.data)
//         }
//
//         #[post("/chat/push")]
//         pub async fn push(msg: Json<Message>) -> impl Responder {
//             HttpResponse::Ok().body(&msg.data)
//         }
//     }
// }
