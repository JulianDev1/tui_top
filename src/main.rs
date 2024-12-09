mod event;

fn main() {
    let eh = event::EventHandler;

    loop {
        let res = eh.handle_events(std::time::Duration::from_millis(1000));
        println!("{:?}", res);
    }
}