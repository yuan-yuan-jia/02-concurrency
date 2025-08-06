use std::time::Duration;

use concurrency::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        std::thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) -> anyhow::Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            std::thread::sleep(Duration::from_millis(rng.random_range(100..5000)));
            metrics.inc(format!("cal.thread.worker.{idx}"))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> anyhow::Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            std::thread::sleep(Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("req.page.{page}")).unwrap();
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
