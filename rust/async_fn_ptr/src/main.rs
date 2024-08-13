use std::future::Future;
use std::pin::{Pin, pin};
use tokio::main;

async fn other() {

}

async fn caller<F>(
    func: F
) -> u32
where
    F: Fn() -> Pin<Box<dyn Future<Output=u32>>>
{
    func().await
}

async fn outer_caller() {
    let a = 1;
    let b = 12;

    println!("{}", caller(|| Box::pin(async move {
        other().await;
        a
    })).await)

    // println!("{}", caller(||{b}).await);
}

#[main]
async fn main() {
    outer_caller().await;
}

