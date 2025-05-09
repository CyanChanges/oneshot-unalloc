use std::pin::Pin;

#[cfg(feature = "std")]
fn main() {
    use std::thread;
    use std::time::Duration;

    let mut chan = oneshot::channel();
    let chan = Pin::new(&mut chan);
    let (sender, receiver) = chan.pair().unwrap();
    let t = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2));
        sender.send(9u128).unwrap();
    });
    assert_eq!(receiver.recv_timeout(Duration::from_millis(100)), Ok(9));
    t.join().unwrap();
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("This example is only for when the \"sync\" feature is used");
}
