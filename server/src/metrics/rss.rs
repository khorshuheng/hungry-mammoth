use metrics::{counter, describe_counter};

pub static RSS_FETCH_FAILURE_COUNT: &str = "rss_fetch_failure_count";

pub(crate) fn init_metrics() {
  describe_counter!(
    RSS_FETCH_FAILURE_COUNT,
    "Number of requests to fetch RSS feeds that failed."
  );
}

#[allow(dead_code)]
pub fn increase_rss_fetch_failure_count() {
  counter!(RSS_FETCH_FAILURE_COUNT).increment(1);
}
