use super::issue::Issue;

pub struct Issues {
    pub inner: Vec<Issue>,
}

impl Issues {
    pub(super) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(super) fn add(&mut self, item: Issue) {
        self.inner.push(item);
    }

    pub fn count(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .filter(|issue| issue.author() == by)
            .count() as i64
    }

    pub fn assigns_count(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn assigns_count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .filter(|issue| {
                issue
                    .assignees()
                    .map_or(false, |assignees| assignees.contains(&by.to_string()))
            })
            .count() as i64
    }

    pub fn comments_count(&self) -> i64 {
        self.inner.iter().map(|issue| issue.comments_count()).sum()
    }

    pub fn comments_count_by(&self, by: &str) -> i64 {
        self.inner
            .iter()
            .map(|issue| issue.comments_count_by(by))
            .sum()
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

    pub fn time_to_closed_average(&self) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for issue in self.inner.iter() {
            if issue.closed_at().is_none() {
                continue;
            };
            count += 1;
            total_seconds += issue.closed_at().unwrap().diff_seconds(&issue.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }

    pub fn time_to_closed_average_by(&self, by: &str) -> f64 {
        let mut count = 0;
        let mut total_seconds = 0;
        for issue in self.inner.iter() {
            if issue.closed_at_by(by).is_none() {
                continue;
            };
            count += 1;
            total_seconds += issue
                .closed_at_by(by)
                .unwrap()
                .diff_seconds(&issue.created_at());
        }
        if count == 0 {
            0.0
        } else {
            total_seconds as f64 / count as f64
        }
    }
}
