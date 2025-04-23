use std::mem;
use std::pin::Pin;

fn main() {
    let mut chan = oneshot::channel();
    let chan = Pin::new(&mut chan);
    let (sender, receiver) = chan.pair().unwrap();
    assert!(sender.send(19i128).is_ok());
    mem::drop(receiver);
}
