
#[tokio::main(flavor="multi_thread",worker_threads=7)]
async fn main() {
    test_something().await;
}

async fn test_something(){
    for i in 1..=10{
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Seconds passed: {i}");
    }
}