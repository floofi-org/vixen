use std::fmt::Debug;
use std::io::{self, Read, Stdin};
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;


#[allow(clippy::module_name_repetitions)]
pub trait StdinReader: Debug {
    fn read(&self) -> Option<u8>;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TerminalStdin {
    receiver: Receiver<u8>,
}

impl TerminalStdin {
    pub fn spawn(sender: SyncSender<u8>, receiver: Receiver<u8>) -> Self {
        let stdin = io::stdin();

        thread::spawn(move || reader_thread(&stdin, &sender));

        Self {
            receiver,
        }
    }
}

impl StdinReader for TerminalStdin {
    fn read(&self) -> Option<u8> {
        if let Ok(char) = self.receiver.try_recv() {
            Some(char)
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
