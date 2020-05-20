use futures::future::Either;
use futures::io;
use futures::prelude::*;
use smol::Async;
use smol::Timer;
use std::net::TcpStream;
use std::time::Duration;

async fn timeout<T>(dur: Duration, f: impl Future<Output = io::Result<T>>) -> io::Result<T> {
    futures::pin_mut!(f);
    match future::select(f, Timer::after(dur)).await {
        Either::Left((out, _)) => out,
        Either::Right(_) => Err(io::ErrorKind::TimedOut.into()),
    }
}

#[test]
fn fd_leak() {
    for _ in 0..1000 {
        smol::run(async {
            std::thread::sleep(Duration::from_secs(1));
            smol::Task::spawn(async {
                let stream = timeout(
                    Duration::from_secs(5),
                    Async::<TcpStream>::connect("172.217.18.174:80"),
                )
                .await
                .unwrap();
                println!("connected");
                drop(stream);
            })
            .await;
        });
    }
}
