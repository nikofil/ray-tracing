use std::task::{Waker, Context, Poll};
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::thread;
use std::thread::sleep;

use futures::task::{ArcWake, waker_ref};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use futures::future::BoxFuture;
use futures::FutureExt;

struct State {
    completed: bool,
    waker: Option<Waker>,
}

pub struct TimerFuture {
    state: Arc<Mutex<State>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    fn new(duration: Duration) -> TimerFuture {
        let state = Arc::new(Mutex::new(State {
            completed: false,
            waker: None,
        }));

        let thr_state = state.clone();
        thread::spawn(move || {
            sleep(duration);
            let mut state = thr_state.lock().unwrap();
            state.completed = true;
            if let Some(waker) = state.waker.clone() {
                waker.wake();
            }
        });

        TimerFuture{ state }
    }
}

/////

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                println!("b4 pending {:?}", context);
                if let Poll::Pending = future.as_mut().poll(context) {
                    println!("got pending {:?}", context);
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks");
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("wakey");
        arc_self.task_sender.send(arc_self.clone()).expect("too many tasks (resched)");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED: usize = 100;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED);
    (Executor{ready_queue}, Spawner{task_sender})
}

pub fn hi() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("Hello");
        TimerFuture::new(Duration::from_secs(3)).await;
        println!("world");
    });

    println!("ran nao");

    executor.run();
}
