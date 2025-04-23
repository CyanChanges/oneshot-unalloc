use std::pin::Pin;

#[cfg(feature = "std")]
fn main() {
    let mut chan = oneshot::channel::<u128>();
    let chan = Pin::new(&mut chan);
    let (sender, receiver) = chan.pair().unwrap();
    std::mem::drop(sender);
    receiver.recv().unwrap_err();
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("This example is only for when the \"sync\" feature is used");
}
