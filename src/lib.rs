type Task = Box<dyn Fn() -> Result<(), String>>;

pub struct ProgressQueue {
    pub queue: Vec<Task>,
    pub max_length: usize,
    pub tasks_done: usize,
}

impl ProgressQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new(), max_length: 0, tasks_done: 0}
    }

    pub fn enqueue(&mut self, f: Task) {
        self.max_length += 1;
        self.queue.push(f);
        println!("{}, {}", self.tasks_done, self.max_length);
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
            println!("{}, {}", self.tasks_done, self.max_length);
            dequeued()
        })
    }
}