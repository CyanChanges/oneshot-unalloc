use std::mem;
use std::pin::Pin;

fn main() {
    let mut chan = oneshot::channel();
    let chan = Pin::new(&mut chan);
    let (sender, receiver) = chan.pair().unwrap();
    mem::drop(receiver);
    let send_error = sender.send(5u128).unwrap_err();
    assert_eq!(send_error.into_inner(), 5);
}
