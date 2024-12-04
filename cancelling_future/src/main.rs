use tokio::time::sleep;
use std::time::Duration;

async fn count_to(num: u8) {
    for i in 1..=num {
        sleep(Duration::from_millis(100)).await;
        println!("{i}");
    }
}

#[tokio::main]
async fn main() {
    println!("start counting");
    // the select! macro polls each
    // future until one of them completes,
    // and then we execute the match arm
    // of the completed future and drop
    // all of the other futures
    tokio::select! {
        _ = count_to(3) => {
            println!("counted to 3");
        },
        _ = count_to(10) => {
            println!("counted to 10");
        },
    };
    println!("stop counting");
    // this sleep is here to demonstrate
    // that the count_to(10) doesn't make
    // any progress after we stop polling
    // it, even if we go to sleep and do
    // nothing else for a while
    sleep(Duration::from_millis(1000)).await;
}
