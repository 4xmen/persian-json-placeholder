use rustavel_core::config::CONFIG;
use serde::Serialize;

pub fn asset_url(path: Option<&str>) -> Option<String> {
    path.map(|path| {
        format!(
            "{}/{}",
            CONFIG.app.url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    })
}


#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    pub current_page: usize,
    pub data: Vec<T>,
    pub first_page_url: String,
    pub from: Option<usize>,
    pub last_page: usize,
    pub last_page_url: String,
    pub next_page_url: Option<String>,
    pub path: String,
    pub per_page: usize,
    pub prev_page_url: Option<String>,
    pub to: Option<usize>,
    pub total: usize,
}

pub struct PaginateBuilder {
    max: usize,
    count: usize,
    page: usize,
    current_url: String,
}

impl PaginateBuilder {
    pub fn new(max: usize, count: usize, page: usize, current_url: String) -> Self {
        Self {
            max: max.max(1),
            count,
            page: page.max(1),
            current_url,
        }
    }

    pub fn offset(&self) -> usize {
        (self.page - 1).saturating_mul(self.max)
    }

    pub fn build<T: Serialize>(&self, data: Vec<T>) -> Pagination<T> {
        let last_page = self.count.div_ceil(self.max);

        let path = self
            .current_url
            .split('?')
            .next()
            .unwrap_or(&self.current_url)
            .to_string();

        let make_url = |page: usize| format!("{}?page={}", path, page);

        let from = if self.offset() < self.count {
            Some(self.offset() + 1)
        } else {
            None
        };

        let to = from.map(|_| (self.offset() + data.len()).min(self.count));

        Pagination {
            current_page: self.page,
            data,
            first_page_url: make_url(1),
            from,
            last_page,
            last_page_url: make_url(last_page.max(1)),
            next_page_url: if self.page < last_page {
                Some(make_url(self.page + 1))
            } else {
                None
            },
            path: path.clone(),
            per_page: self.max,
            prev_page_url: if self.page > 1 && self.page <= last_page {
                Some(make_url(self.page - 1))
            } else {
                None
            },
            to,
            total: self.count,
        }
    }
    pub fn build_resource<T>(&self, data: Vec<T>) -> Pagination<T> {
        let last_page = self.count.div_ceil(self.max);

        let path = self
            .current_url
            .split('?')
            .next()
            .unwrap_or(&self.current_url)
            .to_string();

        let make_url = |page: usize| format!("{}?page={}", path, page);

        let from = if self.offset() < self.count {
            Some(self.offset() + 1)
        } else {
            None
        };

        let to = from.map(|_| (self.offset() + data.len()).min(self.count));

        Pagination {
            current_page: self.page,
            data,
            first_page_url: make_url(1),
            from,
            last_page,
            last_page_url: make_url(last_page.max(1)),
            next_page_url: if self.page < last_page {
                Some(make_url(self.page + 1))
            } else {
                None
            },
            path: path.clone(),
            per_page: self.max,
            prev_page_url: if self.page > 1 && self.page <= last_page {
                Some(make_url(self.page - 1))
            } else {
                None
            },
            to,
            total: self.count,
        }
    }
}
