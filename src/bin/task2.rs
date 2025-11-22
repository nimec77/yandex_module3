use std::task::{Context, Waker};
use waker_fn::waker_fn;


async fn read_toml() -> String {
    smol::fs::read_to_string("./Cargo.toml").await.unwrap()
}

fn main() {
    let (waker, wait) = make_waker();
    let mut context = Context::from_waker(&waker);

    let future = read_toml();
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Pending => {
                println!("Pending future");
            }
            std::task::Poll::Ready(result) => {
                println!("Ready! value:\n{result}");
                break;
            }
        }
        wait();
    }   
}

fn make_waker() -> (Waker, impl Fn()) {
    let t = std::thread::current();
    let waker = waker_fn(move || t.unpark());
    let wait = move || std::thread::park();
    (waker, wait)
}
