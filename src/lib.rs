use std::collections::HashMap;
use std::time::SystemTime;

pub struct Room {
    id: u64,
    name: String,
    status: u8,
    //0:invalid 1:valid
    kind: u8,
    //0:sys 1:pub 2:private
    adminid: u64,
    cts: SystemTime,
    max: u8,
    //contain number
    forbidword: u8, //0:no 1:yes
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
    forbidword: u8,
    //0:no 1:yes
    circle: HashMap<u64, Vec<u64>>, //<housename,vec<chater>>
}

impl Chater {
    fn new(nickname: String) -> Chater {
        Chater {
            id: 0,
            nickname,
            forbidword: 0,
            friends: Default::default(),
            rooms: vec![],
            circle: Default::default(),
        }
    }

    fn add_friend(&mut self, userid: u64) -> bool {
        self.friends.push(userid);
        true
    }

    fn subt_friend(&mut self, userid: u64) -> bool {
        for (index, uid) in self.friends.iter().enumerate() {
            if uid == userid {
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
            if uid == userid {
                circle.remove(index);
                break;
            }
        }
    }
}