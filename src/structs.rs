use std::thread;
use std::sync::mpsc::channel;

pub struct Counter {
    m_count: i64,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { m_count: 0 }
    }

    fn increment_by(&mut self, increment_by: i64) {
        self.m_count += increment_by;
    }
}

pub struct ThreadRunner {}

impl<'a> ThreadRunner {
    pub fn increment_on_thread(increment_by: i64, increment_times: i64, counter: &mut Counter) {
        let (tx, rx) = channel();
        tx.send(counter)
            .expect("Unable to send on channel");

        let handle = thread::spawn(move|| {
            let inc_counter = rx.recv()
                .expect("Unable to recieve from channel");

            for i in 1..increment_times {
                inc_counter.increment_by(increment_by);
            }
            
        });

        handle.join();
    }

    fn increment_counter(counter_to_incement: &mut Counter) -> &mut Counter {
        counter_to_incement.increment_by(1);

        counter_to_incement
    }
}