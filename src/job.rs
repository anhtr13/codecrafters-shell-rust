use std::{collections::HashSet, fmt::Display, process::Child};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JobStatus {
    Running,
    Done,
    Error,
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "Running"),
            Self::Done => write!(f, "Done"),
            Self::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug)]
pub struct Job {
    pub id: u32, // number in the job queue
    pub child: Child,
    pub command: String,
    pub status: JobStatus,
}

pub struct Jobs {
    inner: Vec<Job>,
    id_pool: HashSet<u32>,
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            inner: Vec::new(),
            id_pool: HashSet::new(),
        }
    }

    pub fn push(&mut self, job: Job) {
        self.id_pool.insert(job.id);
        self.inner.push(job);
    }

    pub fn new_id(&self) -> u32 {
        let mut num = 1;
        while self.id_pool.contains(&num) {
            num += 1;
        }
        num
    }

    pub fn print_done(&mut self) {
        for (i, job) in self.inner.iter().enumerate() {
            let marker = if i + 1 == self.inner.len() {
                "+"
            } else if i + 2 == self.inner.len() {
                "-"
            } else {
                " "
            };
            if job.status == JobStatus::Done {
                println!(
                    "[{}]{}  Done                    {}",
                    job.id, marker, job.command
                );
            }
        }
    }

    pub fn update_status(&mut self) {
        for job in self.inner.iter_mut() {
            match job.child.try_wait() {
                Ok(status) => {
                    if status.is_some() {
                        job.status = JobStatus::Done;
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    job.status = JobStatus::Error;
                }
            }
        }
    }

    pub fn clean_up(&mut self) {
        for job in self.inner.iter() {
            if job.status == JobStatus::Done {
                self.id_pool.remove(&job.id);
            }
        }
        self.inner.retain(|job| job.status == JobStatus::Running);
    }

    pub fn get_ref(&self) -> &[Job] {
        &self.inner
    }
}
