use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    let handle = std::thread::spawn(move || loop {
        let data = rx.recv();

        dbg!(&data);

        if data.is_err() {
            break;
        }
    });

    for i in 0..100 {
        tx.send(i).unwrap();
    }

    drop(tx);

    handle.join().unwrap();

    // exit...
}
