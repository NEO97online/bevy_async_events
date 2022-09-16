use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy_async_event::{AddAsyncEvent, AsyncEventChannel};

#[derive(Clone)]
struct MyEvent {
    value: f32,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_async_event::<MyEvent>()
        .add_system(send_event)
        .add_system(read_event)
        .run();
}

fn send_event(event_sender: Res<AsyncEventChannel<MyEvent>>) {
    let event_sender = event_sender.clone();
    AsyncComputeTaskPool::get()
        .spawn(async move { event_sender.send(MyEvent { value: 10.0 }) })
        .detach();
}

fn read_event(mut event_reader: EventReader<MyEvent>) {
    for event in event_reader.iter() {
        info!("Event: {}", event.value);
    }
}
