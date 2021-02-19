use std::{
    collections::HashMap,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
    time::Duration,
};

use actix::clock::{interval_at, Instant};
use actix_web::{
    rt,
    web::{Bytes, Data},
    Error,
};

use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    SinkExt, Stream, StreamExt,
};

use crate::MainmanResult;

mod handler;
pub mod routes;

#[derive(Debug)]
pub struct Message<'a, T: serde::Serialize + std::fmt::Debug> {
    pub recipient: i32,
    pub event: Option<&'a str>,
    pub data: &'a T,
}

pub struct Client(Receiver<Bytes>);

pub struct Broadcaster {
    // TODO: currently only supports one connection per account
    clients: HashMap<i32, Sender<Bytes>>,
}

impl Broadcaster {
    fn new() -> Self {
        Broadcaster {
            clients: HashMap::new(),
        }
    }

    pub fn create() -> Data<Mutex<Self>> {
        let broker_mutex = Data::new(Mutex::new(Broadcaster::new()));
        Self::initiliaze_cleanup(broker_mutex.clone());

        broker_mutex
    }

    fn initiliaze_cleanup(broker: Data<Mutex<Self>>) {
        rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(10));
            while task.next().await.is_some() {
                if let Ok(mut broker) = broker.lock() {
                    for (account, client) in broker.clients.clone().iter_mut() {
                        if client
                            .try_send(Bytes::from("data: ping\n\n"))
                            .is_err()
                        {
                            broker.disconnect(account);
                        }
                    }
                }
            }
        });
    }

    pub fn connect(&mut self, account: i32) -> Client {
        let (tx, rx) = channel(100);
        self.clients.insert(account, tx);
        Client(rx)
    }

    pub fn disconnect(&mut self, account: &i32) {
        self.clients.remove(account);
    }

    #[allow(dead_code)]
    pub async fn send<'a, T: serde::Serialize + std::fmt::Debug>(
        &mut self,
        msg: Message<'a, T>,
    ) -> MainmanResult<()> {
        if let Some(client) = self.clients.get_mut(&msg.recipient) {
            let bytes: Bytes = msg.into();
            return Ok(client.send(bytes).await?);
        }
        Ok(())
    }
}

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a, T: serde::Serialize + std::fmt::Debug> Into<Bytes> for Message<'a, T> {
    fn into(self) -> Bytes {
        let mut payload = self
            .event
            .map_or("".to_owned(), |event| format!("event: {}\n", event));

        payload.push_str(&format!("data: {}\n", json!(self.data)));

        Bytes::from(format!("{}\n", payload))
    }
}
