use std::pin::Pin;

#[cfg(feature = "std")]
fn main() {
    let mut chan = oneshot::channel();
    let chan = Pin::new(&mut chan);
    let (sender, receiver) = chan.pair().unwrap();
    assert!(sender.send(19i128).is_ok());
    assert_eq!(receiver.recv(), Ok(19i128));
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("This example is only for when the \"sync\" feature is used");
}
