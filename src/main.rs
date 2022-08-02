use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use color_eyre::Result;
use std::sync::{Arc, Mutex, MutexGuard};
use std::env;
//use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    //dotenv().ok();

    let conns = Arc::new(Mutex::new(0));

    let address = env::var("ADDRESS").unwrap();
    let port = env::var("PORT").unwrap();
    //let address = dotenv::var("ADDRESS").unwrap();
    //let port = dotenv::var("PORT").unwrap();

    let address = format!("{}:{}",address, port);
    let listener = TcpListener::bind(&address).await?;
    println!("Listening on {}", &address);

    // accept connections and process them serially
    loop {
        match listener.accept().await {
            Err(e) => println!("couldn't get client: {:?}", e),

            Ok((mut socket, addr)) => {
                println!("new client: {:?}", addr);
                let conns = conns.clone();
                increment_conns(&*conns).await;

                tokio::spawn(async move {
                    let mut buf = [0; 128];
                    // In a loop, read data from the socket and write the data back.
                    loop {
                        let conns = conns.clone();

                        let n = match socket.read(&mut buf).await {
                            // socket closed
                            Ok(n) if n == 0 => {
                                println!("stream is empty");
                                decrement_conns(&*conns).await;
                                println!("connections {}", *conns.lock().unwrap());
                                return
                            }
                            Ok(n) => n,
                            Err(e) => {
                                //eprintln!("failed to read from socket; err = {:?}", e);
                                decrement_conns(&*conns).await;
                                return;
                            }
                        };

                        // Write the data back
                        //if let Err(e) = socket.write_all(&buf[0..n]).await {
                        let data = format!("Current: {}", &*conns.lock().unwrap());
                        if let Err(e) = socket.write_all(data.as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            decrement_conns(&*conns).await;
                            return;
                        }
                    }
                });
            }
        }
        let conns = conns.clone();
        println!("connections {}", *conns.lock().unwrap());
    }
}

async fn increment_conns(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;
}

async fn decrement_conns(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock -= 1;
}
