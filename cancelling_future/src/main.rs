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
    let count_to_10 = count_to(10);
    tokio::pin!(count_to_10);
    tokio::select! {
        _ = count_to(3) => {
            println!("counted to 3");
        },
        _ = &mut count_to_10 => {
            println!("counted to 10");
        },
    };
    println!("stop counting");
    println!("jk, keep counting");
    count_to_10.await;
    println!("finished counting to 10");
}
