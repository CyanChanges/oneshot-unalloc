use std::pin::Pin;

#[cfg(feature = "std")]
fn main() {
    use std::thread;
    use std::time::Duration;
    
    let mut chan = oneshot::channel::<u128>();
    let chan = Pin::new(&mut chan);
    let (sender, receiver) = chan.pair().unwrap();
    let t = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2));
        std::mem::drop(sender);
    });
    assert!(receiver.recv_ref().is_err());
    t.join().unwrap();
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("This example is only for when the \"sync\" feature is used");
}
