use async_std::channel::{Receiver, Sender};
use bevy::prelude::*;

#[derive(Clone)]
pub struct AsyncEventChannel<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}
impl<T> AsyncEventChannel<T> {
    pub fn new() -> Self {
        let (tx, rx) = async_std::channel::unbounded();
        Self { tx, rx }
    }

    pub fn send(&self, event: T) {
        self.tx
            .try_send(event)
            .unwrap_or_else(|error| unreachable!("{:?}", error));
    }
}

fn handle_async_events<T: Send + Sync + 'static>(
    channel: Res<AsyncEventChannel<T>>,
    mut event_writer: EventWriter<T>,
) {
    while let Ok(event) = channel.rx.try_recv() {
        event_writer.send(event);
    }
}

pub trait AddAsyncEvent {
    fn add_async_event<T: Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl AddAsyncEvent for App {
    fn add_async_event<T: Send + Sync + 'static>(&mut self) -> &mut Self {
        self.insert_resource(AsyncEventChannel::<T>::new())
            .add_event::<T>()
            .add_system(handle_async_events::<T>);
        self
    }
}
