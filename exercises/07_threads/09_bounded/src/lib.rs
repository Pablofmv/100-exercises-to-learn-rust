// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId,()> {
        let (tx, rx) = std::sync::mpsc::channel();

        self.sender.send(Command::Insert {
            draft,
            response_channel: tx,
        })
        .map_err(|_| ())?;
        
        Ok(rx.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ()> {
        let (tx, rx) = std::sync::mpsc::channel();

        self.sender.send(Command::Get{
            id,
            response_channel: tx,
        })
        .map_err(|_| ())?;
        
        Ok(rx.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender
    }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}


use std::sync::mpsc::{self, SyncSender, Receiver};
use std::thread;

#[derive(Debug)]
enum Job {
    SendEmail(String),
    ProcessOrder(u32),
}

#[derive(Clone)]
struct JobClient {
    sender: SyncSender<Job>,
}

impl JobClient {
    fn submit_job(&self, job: Job) -> Result<(), ()> {
        
        let (tx, rx) = std::sync::mpsc::channel();

        self.sender.send(job).map_err(|_| ())?;

        Ok(())
    }
}

fn worker(rx: Receiver<Job>) {
    while let Ok(job) = rx.recv() {
        match job {
            Job::SendEmail(email) => {
                println!("Sending email to {}", email);
            }
            Job::ProcessOrder(id) => {
                println!("Processing order {}", id);
            }
        }
    }
}

fn launch() -> JobClient {
    let (tx, rx) = mpsc::sync_channel(5);

    thread::spawn(move || worker(rx));

    JobClient { sender: tx }
}

fn main() {
    let client = launch();

    client.submit_job(Job::SendEmail("a@test.com".into())).unwrap();
    client.submit_job(Job::ProcessOrder(42)).unwrap();

    drop(client);
}