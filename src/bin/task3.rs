use std::time::Duration;

struct WaitFor {
    duration: Duration,
    waited: bool,
}

impl Future for WaitFor {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.waited {
            return  std::task::Poll::Ready(());
        }

        self.waited = true;
        let waker = cx.waker().clone();
        let duration = self.duration;
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            waker.wake();
        });
        std::task::Poll::Pending
    }
}

fn wait_for(duration: Duration) -> impl Future<Output = ()> {
    WaitFor { duration, waited: false }
}

#[tokio::main]
async  fn main() {
    println!("Before wait");
    wait_for(Duration::from_secs(2)).await;
    println!("After wait");
}
