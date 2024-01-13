use std::{collections::VecDeque, sync::RwLock};

use anyhow::anyhow;

pub struct RateLimiter {
    ts_deque: RwLock<VecDeque<std::time::Instant>>,
    time_period: std::time::Duration,
    max_requests: usize,
}

impl RateLimiter {
    pub fn new(time_period: std::time::Duration, max_requests: usize) -> Self {
        Self {
            ts_deque: RwLock::new(VecDeque::new()),
            time_period,
            max_requests,
        }
    }

    pub async fn permit(&self) -> Result<(), anyhow::Error> {
        loop {
            let (is_limited, wait_time) = {
                let mut ts_deque = self
                    .ts_deque
                    .write()
                    .map_err(|e| anyhow!("Unable to obtain lock on time deque. {:?}", e))?;

                while let Some(time) = ts_deque.front() {
                    if time.elapsed() > self.time_period {
                        ts_deque.pop_front();
                    } else {
                        break;
                    }
                }

                let is_limited = ts_deque.len() >= self.max_requests;
                let wait_time = if let Some(time) = ts_deque.front() {
                    self.time_period - time.elapsed()
                } else {
                    std::time::Duration::from_secs(0)
                };

                (is_limited, wait_time)
            };

            if !is_limited {
                break;
            }

            tokio::time::sleep(wait_time).await;
        }

        {
            let mut ts_deque = self
                .ts_deque
                .write()
                .map_err(|e| anyhow!("Unable to obtain lock on time deque. {:?}", e))?;
            ts_deque.push_back(std::time::Instant::now());
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = RateLimiter::new(std::time::Duration::from_secs(1), 3);
        let start = std::time::Instant::now();
        for _ in 0..4 {
            rate_limiter.permit().await.unwrap();
        }
        assert!(start.elapsed() >= std::time::Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_rate_limiter_multi() {
        let rate_limiter = RateLimiter::new(std::time::Duration::from_millis(100), 3);
        let start = std::time::Instant::now();
        for _ in 0..10 {
            rate_limiter.permit().await.unwrap();
        }
        assert!(start.elapsed() >= std::time::Duration::from_millis(300));
    }
}
