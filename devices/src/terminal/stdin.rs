use std::fmt::Debug;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use getch::Getch;

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
        let getch = Getch::new();
        thread::spawn(move || reader_thread(&getch, &sender));

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

fn reader_thread(stdin: &Getch, sender: &SyncSender<u8>) {
    loop {
        if let Ok(ch) = stdin.getch() {
            if sender.send(ch).is_err() {
                break;
            }
        }
    }
}
