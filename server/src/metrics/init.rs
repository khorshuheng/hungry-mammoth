use super::{request, rss};

pub fn init_metrics() {
  request::init_metrics();
  rss::init_metrics();
}
