// src/job.rs
use once_cell::sync::Lazy;
use std::{sync::Mutex, thread};

pub struct Job {
    pub id: u32,
    pub thread_id: thread::ThreadId,
    pub command: String,
}

// グローバルなジョブリスト
static JOBS: Lazy<Mutex<Vec<Job>>> = Lazy::new(|| Mutex::new(Vec::new()));
static JOB_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

// 実装すべき関数
pub fn create_job(command: String, thread_id: thread::ThreadId) -> u32 {
    // job_idを作成
    let job_id = get_next_job_id();

    let job = Job {
        id: job_id,
        thread_id,
        command,
    };

    // リストに追加
    JOBS.lock().unwrap().push(job);

    // IDを返す
    job_id
}
pub fn get_next_job_id() -> u32 {
    let mut counter = JOB_COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_create_job() {
        // ジョブIDが順番に増えることを確認
        let thread_id = thread::current().id();

        let job_id1 = create_job("echo test1".to_string(), thread_id);
        let job_id2 = create_job("echo test2".to_string(), thread_id);

        assert_eq!(job_id2, job_id1 + 1);
    }

    #[test]
    fn test_job_list() {
        // ジョブがリストに追加されることを確認
        let initial_count = JOBS.lock().unwrap().len();
        let thread_id = thread::current().id();

        create_job("test command".to_string(), thread_id);

        let jobs = JOBS.lock().unwrap();
        assert_eq!(jobs.len(), initial_count + 1);

        // 最後のジョブを確認
        let last_job = jobs.last().unwrap();
        assert_eq!(last_job.command, "test command");
        assert_eq!(last_job.thread_id, thread_id);
    }
}
