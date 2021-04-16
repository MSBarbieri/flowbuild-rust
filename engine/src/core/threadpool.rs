use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;
#[derive(Clone, Debug)]
pub struct EnginePoolnfo {
    engine_workers_count: usize,
    engine_jobs_count: usize,
    timer_workers_count: usize,
    timer_jobs_count: usize,
    notification_workers_count: usize,
    notification_jobs_count: usize,
}

impl EnginePoolnfo {
    pub fn new(
        _engine_workers_count: usize,
        _engine_jobs_count: usize,
        _timer_workers_count: usize,
        _timer_jobs_count: usize,
        _notification_workers_count: usize,
        _notification_jobs_count: usize,
    ) -> Self {
        EnginePoolnfo {
            engine_workers_count: _engine_workers_count,
            timer_workers_count: _timer_workers_count,
            notification_workers_count: _notification_workers_count,
            engine_jobs_count: _engine_jobs_count,
            timer_jobs_count: _timer_jobs_count,
            notification_jobs_count: _notification_jobs_count,
        }
    }

    pub fn default() -> Self {
        EnginePoolnfo {
            engine_workers_count: 3,
            engine_jobs_count: 10,
            timer_workers_count: 2,
            timer_jobs_count: 3,
            notification_workers_count: 1,
            notification_jobs_count: 3,
        }
    }
}

#[derive(Debug)]
pub struct InternalPool {
    pool: ThreadPool,
    pub jobs: usize,
    pub sender: Sender<Box<dyn FnOnce() + Send>>,
    pub receiver: Receiver<Box<dyn FnOnce() + Send>>,
}

impl InternalPool {
    pub fn new(num_threads: usize, num_jobs: usize) -> Self {
        let pool = ThreadPool::new(num_threads);
        let (tx, rx) = channel();
        InternalPool {
            pool: pool,
            jobs: num_jobs,
            sender: tx as Sender<Box<dyn FnOnce() + Send>>,
            receiver: rx as Receiver<Box<dyn FnOnce() + Send>>,
        }
    }

    pub fn execute(&self, command: fn()) {
        {
            self.pool.execute(command);
        }
    }
}

#[derive(Debug)]
pub struct EngineThreadPool {
    pub info: EnginePoolnfo,
    pub process_pool: InternalPool,
    pub notification_pool: InternalPool,
    pub timer_pool: InternalPool,
}

impl EngineThreadPool {
    pub fn new(info: Option<EnginePoolnfo>) -> Self {
        let _info = match info {
            Some(i) => i,
            None => EnginePoolnfo::default(),
        };

        let engine = EngineThreadPool {
            process_pool: InternalPool::new(_info.engine_workers_count, _info.engine_jobs_count),
            notification_pool: InternalPool::new(
                _info.notification_workers_count,
                _info.notification_jobs_count,
            ),
            timer_pool: InternalPool::new(_info.timer_workers_count, _info.timer_jobs_count),
            info: _info.clone(),
        };
        engine
    }

    pub fn start(self) {}
}
