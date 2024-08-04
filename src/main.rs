use std::sync::mpsc;

fn main() {
    let (tx1, rx1) = mpsc::channel::<()>();
    let (tx2, rx2) = mpsc::channel::<()>();

    let h1 = std::thread::spawn(move || {
        for i in 0..100 {
            // ->h2: hey print something.
            // -> wait for h2's command.

            tx2.send(()).unwrap();
            rx1.recv().unwrap();

            println!("A {i}");
        }
    });

    let h2 = std::thread::spawn(move || {
        for i in 0..100 {
            // -> wait for h1's command.
            rx2.recv().unwrap();
            println!("B {i}");
            tx1.send(()).unwrap();
            // ->h1: print something
        }
    });

    // ...
    // for i in 0..100 {
    //     println!("Z {i}");
    // }

    h1.join().unwrap();
    h2.join().unwrap();
}

