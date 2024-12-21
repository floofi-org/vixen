use std::sync::mpsc::{Receiver, SyncSender};
use std::io::{self, Read, Stdin};
use std::thread;


#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct StdinReader {
    receiver: Receiver<u8>,
}

impl StdinReader {
    pub fn spawn(sender: SyncSender<u8>, receiver: Receiver<u8>) -> Self {
        let stdin = io::stdin();

        thread::spawn(move || reader_thread(&stdin, &sender));

        Self {
            receiver,
        }
    }

    pub fn read(&self) -> Option<u8> {
        if let Ok(value) = self.receiver.try_recv() {
            Some(value)
        } else {
            None
        }
    }
}

fn reader_thread(stdin: &Stdin, sender: &SyncSender<u8>) {
    loop {
        let mut buf = [0u8; 1];
        stdin.lock().read_exact(&mut buf).unwrap();

        if sender.send(buf[0]).is_err() {
            break;
        }
    }
}
