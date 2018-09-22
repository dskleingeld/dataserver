extern crate actix;
extern crate actix_web;
extern crate rand;
use self::actix::prelude::*;

use self::rand::{Rng, ThreadRng};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};


///// New chat session is created
//#[derive(Message)]
//#[rtype(usize)]
//pub struct Connect {
	//pub addr: Recipient<Message>,
//}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum DataSource {
	Light,
	Humidity,
	Temperature,
	Error,
}

pub fn source_string_to_enum(source_name: &str) -> Result<DataSource, ()> {
	match source_name {
		"Light" => Ok(DataSource::Light),
		"Humidity" => Ok(DataSource::Humidity),
		"Temperature" => Ok(DataSource::Temperature),
		_ => Err(()),
	}
}

#[derive(Message)]
pub struct clientMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct NewData {
    pub from: DataSource,
}

impl Handler<NewData> for DataServer {
    //type Result = usize;
    type Result = usize;

    fn handle(&mut self, msg: NewData, _: &mut Context<Self>) -> Self::Result {
        
        println!("there is new data");
        let subs = self.subs.get(&msg.from).unwrap();
		//println!("{}", subs);
		for client in subs.iter() {
			//println!("{}", client);
            if let Some(session) = self.sessions.get(client) {
				let _ = session.addr.send(clientMessage("test".to_owned()));
			}
		}
	0
    }
}

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<clientMessage>,
}

impl Handler<Connect> for DataServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let id = self.rng.borrow_mut().gen::<usize>();
        self.sessions.insert(id, clientInfo {addr: msg.addr, subs: Vec::new()} );
		
        // send id back
        id
    }
}


#[derive(Message)]
pub struct Disconnect {
    pub id: usize,
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for DataServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");
        // remove address
        if let Some(client_info) = self.sessions.remove(&msg.id) {
			for sub in client_info.subs {
				if let Some(subbed_clients) = self.subs.get_mut(&sub) {
					subbed_clients.remove(&msg.id);
					println!("removed client from: sub:{:?} ", sub);
				}
			}
		}
    }
}

/// New chat session is created
#[derive(Message)]
pub struct SubscribeToSource {
    pub id: usize,
    pub source: DataSource,
}

impl Handler<SubscribeToSource> for DataServer {
    type Result = ();

    fn handle(&mut self, msg: SubscribeToSource, _: &mut Context<Self>) -> Self::Result {
        let SubscribeToSource { id, source } = msg;
        println!("subscribing to source");
		let client_info = self.sessions.get_mut(&id).unwrap();
		client_info.subs.push(source.clone() );
        
        //fix when non lexical borrow checker arrives
        if let Some(subscribers) = self.subs.get_mut(&source){
			subscribers.insert(id);
			return ()
		}
		
		let mut subscribers = HashSet::new();
		subscribers.insert(id);
		self.subs.insert(source, subscribers);
        ()
    }
}

pub struct clientInfo {
	addr: Recipient<clientMessage>,
	subs: Vec<DataSource>,
}

pub struct DataServer {
    sessions: HashMap<usize, clientInfo>,
    #[derive(Debug)] subs: HashMap<DataSource, HashSet<usize>>,
    
    rng: RefCell<ThreadRng>,
}

impl Default for DataServer {
    fn default() -> DataServer {

        DataServer {
            sessions: HashMap::new(),
            subs: HashMap::new(),
            
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for DataServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}