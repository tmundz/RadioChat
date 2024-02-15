use uuid::Uuid;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

pub type UID = Uuid;
pub type RID = Uuid;

pub struct User {
    uid: UID, // will change to be Name:gen_digits
    u_name: String,
    cur_room: Option<RID>,
}

pub struct Room {
    rid: RID, // will change to be Name:gen_digits
    r_name: String,
    members: Mutex<HashSet<UID>>,
    active_members: Mutex<HashSet<UID>>,
}

pub struct Server {
    rooms: Mutex<HashMap<RID, Room>>, // is this a completed set of rooms
    users: Mutex<HashMap<UID, User>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            rooms: Mutex::new(HashMap::new()),
            users: Mutex::new(HashMap::new()),
        }
    }
    pub async fn add_user(&self, user: User) {
        // add check to confirm Uuid does not exist
        self.users.lock().unwrap().insert(user.uid, user);
    }
    pub async fn delete_user(&self) {}
    pub async fn add_room(&self, room: Room) {
        self.rooms.lock().unwrap().insert(room.rid, room);
    }

    pub async fn delete_room(&self) {}

    //need enable web connection
    //need disable web connection
}
