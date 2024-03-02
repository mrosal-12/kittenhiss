extern crate core;

use std::collections::HashMap;
use extendr_api::{
    prelude::*,
    wrapper::*,
};
use nats::{self, Connection};

mod wcqr;

pub const TYPE_FIELDS: HashMap<&str, HashMap<&str, &str>> = HashMap::from(todo!());

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

struct NatsServer {
    con: Result<Connection, _>
}

#[extendr] 
impl NatsServer {
    fn new() -> Self {
        NatsServer {con: nats::connect("nats://localhost:4443")}
    }
    fn send(&self, entry: Entry) -> &'static str {
        match self.con {
            Ok(c) => match c.publish("push", entry) {
                Ok(_) => "Sent",
                _ => "Couldn't Send",
            }
            _ => "Invalid Connection"
        }
    }
    fn pull(&self, entry: Entry) -> Option<Entry> {
        if let Ok(c) = self.con {match c.request("pull", entry) {
            Ok(str) => Some(Entry::from(str.data)),
            _ => None
        }} else {None}
    }
}

pub struct Entry {
    pub typ: String,
    pub source: String,
    pub event: String,
    pub team: usize,
    pub matchn: usize,
    pub alliance: usize,
    pub data: Vec<Data>
}

impl Entry {
    fn from(str: Vec<u8>) {
        let st = String::from_utf8(str).unwrap_or_else(||"".to_string());
        let mut iter = string_to_vec(st).into_iter();
        let typ = iter.next().unwrap();
        let source = iter.next().unwrap();
        let event = iter.next().unwrap();
        let team = iter.next().unwrap().parse::<usize>().unwrap_or_else(|_| 0);
        let matchn = iter.next().unwrap().parse::<usize>().unwrap_or_else(|_| 0);
        let alliance = iter.next().unwrap().parse::<usize>().unwrap_or_else(|_| 0);
        let mut data =  Vec::new();
        for (_, t) in *TYPE_FIELDS.get(typ.as_str()).unwrap_or_else(|_| vec) {
            data.push(match t {
                "int" => Data::Integer(scn.next().unwrap().parse::<i32>().unwrap_or_else(|_| 0)),
                "flt" => Data::Float(scn.next().unwrap().parse::<f64>().unwrap_or_else(|_| 0)),
                "bool" => Data::Boolean(if let Ok(1) = scn.next().unwrap().parse::<i32>() {true} else {false}),
                "txt" => Data::Text(scn.next().unwrap()),
                "pos" => Data::Position(scn.next().unwrap()),
                "notes" => Data::Text(scn.nth(scn.len())),
                _ => Data::Null,
            });
        }
        Self {
            typ: typ,
            source: source,
            event: event,
            team: team,
            matchn: matchn,
            alliance: alliance,
            data: data,
        }
    }
}

#[extendr]
impl Entry {
    fn new(typ: &str, source: &str, event: &str, team: usize, matchn: usize, alliance: usize) -> Self {
        Self {
            typ: typ.to_string(),
            source: source.to_string(),
            event: event.to_string(),
            team: team,
            matchn: matchn,
            alliance: alliance,
            data: data,
        }
    }
    fn scan_qr(typ: &str) -> Self {
        let mut scn = string_to_vec(wcqr::scan()).into_iter();
        let source = scn.next().unwrap();
        let event = scn.next().unwrap();
        scn.next();
        let matchn = scn.next().unwrap().parse::<usize>().unwrap_or_else(|_| 0);
        let team = scn.next().unwrap().parse::<usize>().unwrap_or_else(|_| 0);
        let alliance = match scn.next().unwrap().to_str() {
            "r1" | "r2" | "r3" => 0,
            _ => 1,
        };
        let mut data =  Vec::new();
        for (_, t) in *TYPE_FIELDS.get(typ).unwrap_or_else(|_| vec) {
            data.push(match t {
                "int" => Data::Integer(scn.next().unwrap().parse::<i32>().unwrap_or_else(|_| 0)),
                "flt" => Data::Float(scn.next().unwrap().parse::<f64>().unwrap_or_else(|_| 0)),
                "bool" => Data::Boolean(if let Ok(1) = scn.next().unwrap().parse::<i32>() {true} else {false}),
                "txt" => Data::Text(scn.next().unwrap()),
                "pos" => Data::Position(scn.next().unwrap()),
                "notes" => Data::Text(scn.nth(scn.len())),
                _ => Data::Null,
            });
        }
        Self {
            typ: typ.to_string(),
            source: source,
            event: event,
            team: team,
            matchn: matchn,
            alliance: alliance,
            data: data,
        }
    }
}

impl AsRef<[u8]> for Entry {
    fn as_ref(&self) -> &[u8] {
        let mut str = 
            self.typ + " ";
        str.push_str((self.source + " ").as_str());
        str.push_str((self.event + " ").as_str());
        str.push_str((self.team.to_string() + " ").as_str());
        str.push_str((self.matchn.to_string() + " ").as_str());
        str.push_str((self.alliance.to_string() + " ").as_str());
        for d in self.data {
            str.push_str(match d {
                Data::Boolean(b) => if b {"true "} else {"false "},
                Data::Float(f) => (f.to_string() + " ").as_str(),
                Data::Integer(i) => (i.to_string() + " ").as_str(),
                Data::Position(a) | Data::Text(a) => a.as_str(),
                _ => "",
            });
        }
        str.as_ref()
    }
}

pub enum Data {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Text(String),
    Position(String),
    Null,
}

#[extendr]
impl Data {
    fn get_type(&self) -> i32 {
        match self {
            Data::Boolean => 0,
            Data::Integer(_) => 1,
            Data::Float(_) => 2,
            Data::Text(_) => 3,
            Data::Position(_) => 4,
            Data::Null => 5,
        }
    }
    fn get_int(&self) -> i32 {
        if let Data::Integer(i) = self {i} else {0}
    }
    fn get_float(&self) -> f32 {
        if let Data::Integer(i) = self {i} else {0}
    }
    fn get_bool(&self) -> bool {
        if let Data::Integer(i) = self {i} else {0}
    }
    fn get_txt(&self) -> &'static str {
        if let Data::Integer(i) = self {i} else {0}
    }
    fn get_pos(&self) -> &'static str {
        if let Data::Integer(i) = self {i} else {0}
    }
}

fn string_to_vec(s: String) -> Vec<String> {
    let mut vec = Vec::new();
    vec.push(String::new());
    for c in s.chars() {
        if c == ' ' {vec.push(String::new());}
        else {vec[vec.len()].push(c);}
    }
    vec
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod khssr;
    fn hello_world; 
}
