use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Job {
    pub id: Uuid,
    pub attempts: i32,
    pub max_attempts: i32,
}

pub struct JobChannel {
    pub tx: mpsc::Sender<Job>,
    pub rx: mpsc::Receiver<Job>,
}

impl JobChannel {
    pub fn new(limit: usize) -> Self {
        let (tx, rx) = mpsc::channel(limit);

        Self { tx, rx }
    }
}
