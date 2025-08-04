use anyhow::anyhow;
use rand::Rng;
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join errorA:{:?}", e))?;
    println!("secret: {}", secret);
    Ok(())
}

fn producer(id: usize, tx: mpsc::Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let mut rng = rand::rng();
        let value = rng.random_range(0..usize::MAX);
        tx.send(Msg::new(id, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", id);
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_test() -> anyhow::Result<()> {
        Ok(())
    }
}
