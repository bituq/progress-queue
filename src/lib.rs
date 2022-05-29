type Task = Box<dyn Fn() -> Result<(), String>>;

#[derive(Default)]
pub struct ProgressQueue {
    pub queue: Vec<Task>,
    pub max_length: usize,
    pub tasks_done: usize,
}

impl ProgressQueue {
    pub fn enqueue(&mut self, f: Task) {
        self.max_length += 1;
        self.queue.push(f);
    }

    fn dequeue(&mut self) -> Option<Task> {
        match self.queue.is_empty() {
            true => None,
            false => Some(self.queue.remove(0))
        }
    }

    pub fn execute(&mut self) -> Option<Result<(), String>> {
        self.dequeue().map(|dequeued| {
            self.tasks_done += 1;
            dequeued()
        })
    }
}

