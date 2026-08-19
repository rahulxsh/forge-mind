use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Job {
    pub id: String,
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
