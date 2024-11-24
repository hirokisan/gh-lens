use super::pull_request::PullRequest;

pub struct PullRequests {
    pub inner: Vec<PullRequest>,
}

impl PullRequests {
    pub(super) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(super) fn add(&mut self, item: PullRequest) {
        self.inner.push(item);
    }

    pub fn count(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn count_by(&self, by: &str) -> i64 {
        self.inner.iter().filter(|pr| pr.author() == by).count() as i64
    }

    pub fn comments_count(&self) -> i64 {
        self.inner.iter().map(|pr| pr.comments_count()).sum()
    }

    pub fn comments_count_by(&self, by: &str) -> i64 {
        self.inner.iter().map(|pr| pr.comments_count_by(by)).sum()
    }

    pub fn comments_count_average(&self) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.comments_count() as f64 / self.count() as f64
        }
    }

    pub fn comments_count_average_by(&self, by: &str) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.comments_count_by(by) as f64 / self.count() as f64
        }
    }

    pub fn commits_count(&self) -> i64 {
        self.inner.iter().map(|pr| pr.commits_count()).sum()
    }

    pub fn commits_count_by(&self, by: &str) -> i64 {
        self.inner.iter().map(|pr| pr.commits_count_by(by)).sum()
    }

    pub fn commits_count_average(&self) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.commits_count() as f64 / self.count() as f64
        }
    }

    pub fn commits_count_average_by(&self, by: &str) -> f64 {
        if self.count_by(by) == 0 {
            0.0
        } else {
            self.commits_count_by(by) as f64 / self.count_by(by) as f64
        }
    }

    pub fn changed_files_count(&self) -> i64 {
        self.inner.iter().map(|pr| pr.changed_files_count()).sum()
    }

    pub fn changed_files_count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .map(|pr| pr.changed_files_count_by(by))
            .sum()
    }

    pub fn changed_files_count_average(&self) -> f64 {
        if self.count() == 0 {
            0.0
        } else {
            self.changed_files_count() as f64 / self.count() as f64
        }
    }

    pub fn changed_files_count_average_by(&self, by: &str) -> f64 {
        if self.count_by(by) == 0 {
            0.0
        } else {
            self.changed_files_count_by(by) as f64 / self.count_by(by) as f64
        }
    }

    pub fn time_to_first_contacted_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.first_contacted_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr
                .first_contacted_at()
                .unwrap()
                .diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_first_contacted_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.first_contacted_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr
                .first_contacted_at_by(by)
                .unwrap()
                .diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_approved_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.approved_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr.approved_at().unwrap().diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_approved_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.approved_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr
                .approved_at_by(by)
                .unwrap()
                .diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_merged_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.merged_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr.merged_at().unwrap().diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_merged_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for pr in self.inner.iter() {
            if pr.merged_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += pr.merged_at_by(by).unwrap().diff_seconds(&pr.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }
}
