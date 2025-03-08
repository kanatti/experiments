use std::thread;

pub fn main() {
    println!("--- Simple thread spawning ---");
    let thread1 = thread::spawn(f);
    let thread2 = thread::spawn(f);
    println!("Hello from main thread - {:?}", thread::current().id());
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("--- Thread spawning with move closure ---");
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        println!("Numbers: {:?}", numbers); // Ownership moved
    })
    .join()
    .unwrap();

    println!("--- Scoped spawning with reference closure ---");
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("Numbers: {:?}", numbers); // Ownership not moved
        })
        .join()
        .unwrap()
    }); // Blocks

    println!("--- Move required for mutation ---");
    let mut numbers = vec![1, 2, 3];
    thread::spawn(move || {
        numbers.push(4); // Ownership moved and mutable
        println!("Numbers: {:?}", numbers);
    })
    .join()
    .unwrap();

    println!("--- Named threads with ThreadBuiler ---");
    thread::Builder::new()
        .name("thread1".into())
        .spawn(f)
        .unwrap().join().unwrap();
    thread::Builder::new()
        .name("thread2".into())
        .spawn(f)
        .unwrap().join().unwrap();
}

fn f() {
    println!(
        "Hello from thread - ID: {:?}, Name: {:?}",
        thread::current().id(),
        thread::current().name()
    );
}
