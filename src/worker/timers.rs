use super::*;

use tokio::timer::{Delay, Interval};

use std::time::{Duration, Instant};

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9c2f708c83903668510a4c75c5f7c6db

thread_local! {
    static TIMER_HANDLES: RefCell<HashMap<i32, oneshot::SpawnHandle<(), ()>>> = RefCell::new(HashMap::new());
}

#[v8_fn]
pub fn set_timer_handler(args: FunctionCallbackInfo) {
    let mut context = get_context();
    let mut global = context.global();
    let func = args.get(0).unwrap().to_function();
    global.set_private(context, "timer_handler", func);
}

pub fn call_timer_handler(mut context: Local<V8::Context>, args: Vec<&IntoValue>) {
    let null = Isolate::null();
    context
        .global()
        .get_private(context, "timer_handler")
        .to_function()
        .call(context, &null, args);
}

#[v8_fn]
pub fn set_timeout(args: FunctionCallbackInfo) {
    let id = args.get(0).unwrap().to_number().value() as i32;
    let ms = args.get(1).unwrap().to_number().value() as u64;
    let start = Instant::now() + Duration::from_millis(ms);
    let task = Delay::new(start)
        .and_then(move |_| {
            handle_scope!({
                let context = get_context();
                call_timer_handler(context, vec![&id]);
            });
            future::ok(())
        })
        .map_err(|e| {
            panic!("Unexpected error when setting timeout: {}", e);
        });
    let exec = current_thread::TaskExecutor::current();
    let handle = futures::sync::oneshot::spawn(task, &exec);

    TIMER_HANDLES.with(|h| (*h.borrow_mut()).insert(id, handle));
}

// id, ms
#[v8_fn]
pub fn set_interval(args: FunctionCallbackInfo) {
    let id = args.get(0).unwrap().to_number().value() as i32;
    let ms = args.get(1).unwrap().to_number().value() as u64;
    let start = Instant::now() + Duration::from_millis(ms);
    let task = Interval::new(start, Duration::from_millis(ms))
        .for_each(move |_| {
            handle_scope!({
                let context = get_context();
                call_timer_handler(context, vec![&id]);
            });
            future::ok(())
        })
        .map_err(|e| {
            // Error can only happen here if we can't instantiate Intervals.
            panic!("{}", e);
        });
    let exec = current_thread::TaskExecutor::current();
    let handle = futures::sync::oneshot::spawn(task, &exec);

    TIMER_HANDLES.with(|h| (*h.borrow_mut()).insert(id, handle));
}

// id
#[v8_fn]
pub fn clear_timer(args: FunctionCallbackInfo) {
    let id = args.get(0).unwrap().to_number().value() as i32;
    TIMER_HANDLES.with(|h| (*h.borrow_mut()).remove(&id));
}
