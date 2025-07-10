use log::debug;
// src/job.rs
use once_cell::sync::Lazy;
use std::{sync::Mutex, thread};

#[derive(Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub thread_id: thread::ThreadId,
    pub command: String,
    pub status: JobStatus,
}

/// ジョブのステータス
#[derive(Debug, Clone)]
pub enum JobStatus {
    Running,
    Completed,
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
        status: JobStatus::Running,
    };

    // リストに追加
    JOBS.lock().unwrap().push(job);

    // IDを返す
    job_id
}

// 指定されたjob_idでjobを作成
pub fn create_job_with_id(job_id: u32, command: String, thread_id: thread::ThreadId) {
    // 指定されたIDでジョブを作成
    let job = Job {
        id: job_id,
        thread_id,
        command,
        status: JobStatus::Running,
    };

    // リストに追加
    JOBS.lock().unwrap().push(job);
}
pub fn get_next_job_id() -> u32 {
    let mut counter = JOB_COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}

// ジョブ一覧を返す
pub fn list_jobs() -> Vec<Job> {
    cleanup_completed_jobs();
    JOBS.lock().unwrap().clone()
}

// 特定のジョブを取得
pub fn get_job(id: u32) -> Option<Job> {
    // JOBSをロック
    let jobs = JOBS.lock().unwrap();

    jobs.iter().find(|job| job.id == id).cloned()
}

// ジョブを完了状態にする
pub fn mark_completed(job_id: u32) {
    debug!("Marking job {job_id} as completed");
    let mut jobs = JOBS.lock().unwrap();
    if let Some(job) = jobs.iter_mut().find(|job| job.id == job_id) {
        job.status = JobStatus::Completed;
    }

    // 完了状態のジョブを削除
    // cleanup_completed_jobs();
}

// 完了したジョブを削除
fn cleanup_completed_jobs() {
    let mut jobs = JOBS.lock().unwrap();
    let initial_count = jobs.len();

    // 完了したジョブを削除
    jobs.retain(|job| matches!(job.status, JobStatus::Running));

    let removed_count = initial_count - jobs.len();
    if removed_count > 0 {
        debug!("Cleaned up {removed_count} completed jobs");
    }
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
