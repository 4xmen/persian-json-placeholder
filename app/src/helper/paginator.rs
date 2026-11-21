use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
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

/// Builds a Laravel-style pagination response from total count, page size and current page data.
pub struct PaginateBuilder {
    max: usize,
    count: usize,
    page: usize,
    current_url: String,
}

#[allow(dead_code)]
impl PaginateBuilder {
    /// Creates a new builder.
    /// - `max` is clamped to at least 1
    /// - `page` is clamped to at least 1
    pub fn new(max: usize, count: usize, page: usize, current_url: String) -> Self {
        Self {
            max: max.max(1),
            count,
            page: page.max(1),
            current_url,
        }
    }

    /// Returns the zero-based offset for the current page.
    pub fn offset(&self) -> usize {
        (self.page - 1).saturating_mul(self.max)
    }

    /// Builds the final pagination object.
    /// Caller is responsible for slicing the correct page of data.
    pub fn build<T: Serialize>(&self, data: Vec<T>) -> Pagination<T> {
        // How many pages are needed in total
        let last_page = self.count.div_ceil(self.max);

        // Keep only the path part for the public `path` field
        let path = self
            .current_url
            .split('?')
            .next()
            .unwrap_or(&self.current_url)
            .to_string();

        let from = if self.offset() < self.count {
            Some(self.offset() + 1)
        } else {
            None
        };

        // `to` is based on actual data length so last page can be shorter
        let to = from.map(|_| (self.offset() + data.len()).min(self.count));

        Pagination {
            current_page: self.page,
            data,
            first_page_url: self.make_url(1),
            from,
            last_page,
            last_page_url: self.make_url(last_page.max(1)),
            next_page_url: if self.page < last_page {
                Some(self.make_url(self.page + 1))
            } else {
                None
            },
            path,
            per_page: self.max,
            prev_page_url: if self.page > 1 && self.page <= last_page {
                Some(self.make_url(self.page - 1))
            } else {
                None
            },
            to,
            total: self.count,
        }
    }

    /// Builds a URL for the given page while preserving any existing query parameters
    /// (except the old `page` value which is replaced).
    fn make_url(&self, page: usize) -> String {
        let (base, query) = match self.current_url.split_once('?') {
            Some((b, q)) => (b, Some(q)),
            None => (self.current_url.as_str(), None),
        };

        // Drop any existing page=... and keep the rest
        let mut parts: Vec<&str> = query
            .map(|q| {
                q.split('&')
                    .filter(|pair| !pair.starts_with("page=") && !pair.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        // Always add the new page
        let page_param = format!("page={page}");
        parts.push(&page_param);

        format!("{base}?{}", parts.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Debug, Serialize, PartialEq, Clone)]
    struct Item {
        id: usize,
    }

    fn items(from: usize, to: usize) -> Vec<Item> {
        (from..=to).map(|id| Item { id }).collect()
    }

    #[test]
    fn first_page_full() {
        let b = PaginateBuilder::new(10, 25, 1, "https://api.example.com/users".into());
        let data = items(1, 10);
        let p = b.build(data.clone());

        assert_eq!(p.current_page, 1);
        assert_eq!(p.per_page, 10);
        assert_eq!(p.total, 25);
        assert_eq!(p.last_page, 3);
        assert_eq!(p.from, Some(1));
        assert_eq!(p.to, Some(10));
        assert_eq!(p.data, data);
        assert_eq!(p.first_page_url, "https://api.example.com/users?page=1");
        assert_eq!(p.last_page_url, "https://api.example.com/users?page=3");
        assert_eq!(
            p.next_page_url,
            Some("https://api.example.com/users?page=2".into())
        );
        assert_eq!(p.prev_page_url, None);
        assert_eq!(p.path, "https://api.example.com/users");
    }

    #[test]
    fn middle_page() {
        let b = PaginateBuilder::new(10, 25, 2, "https://api.example.com/users".into());
        let p = b.build(items(11, 20));

        assert_eq!(p.current_page, 2);
        assert_eq!(p.from, Some(11));
        assert_eq!(p.to, Some(20));
        assert_eq!(
            p.next_page_url,
            Some("https://api.example.com/users?page=3".into())
        );
        assert_eq!(
            p.prev_page_url,
            Some("https://api.example.com/users?page=1".into())
        );
    }

    #[test]
    fn last_page_partial() {
        let b = PaginateBuilder::new(10, 25, 3, "https://api.example.com/users".into());
        let p = b.build(items(21, 25));

        assert_eq!(p.current_page, 3);
        assert_eq!(p.from, Some(21));
        assert_eq!(p.to, Some(25));
        assert_eq!(p.next_page_url, None);
        assert_eq!(
            p.prev_page_url,
            Some("https://api.example.com/users?page=2".into())
        );
    }

    #[test]
    fn empty_collection() {
        let b = PaginateBuilder::new(10, 0, 1, "https://api.example.com/users".into());
        let p = b.build(Vec::<Item>::new());

        assert_eq!(p.total, 0);
        assert_eq!(p.last_page, 0);
        assert_eq!(p.from, None);
        assert_eq!(p.to, None);
        assert_eq!(p.next_page_url, None);
        assert_eq!(p.prev_page_url, None);
        assert_eq!(p.last_page_url, "https://api.example.com/users?page=1");
    }

    #[test]
    fn page_past_the_end() {
        let b = PaginateBuilder::new(10, 25, 9, "https://api.example.com/users".into());
        let p = b.build(Vec::<Item>::new());

        assert_eq!(p.current_page, 9);
        assert_eq!(p.from, None);
        assert_eq!(p.to, None);
        assert_eq!(p.next_page_url, None);
        assert_eq!(p.prev_page_url, None); // page > last_page
    }

    #[test]
    fn preserves_other_query_params() {
        let b = PaginateBuilder::new(
            10,
            30,
            2,
            "https://api.example.com/users?filter=active&sort=name&page=99".into(),
        );
        let p = b.build(items(11, 20));

        // old page= is removed, other params stay, new page is added
        assert!(p.next_page_url.as_ref().unwrap().contains("filter=active"));
        assert!(p.next_page_url.as_ref().unwrap().contains("sort=name"));
        assert!(p.next_page_url.as_ref().unwrap().contains("page=3"));
        assert!(!p.next_page_url.as_ref().unwrap().contains("page=99"));
        assert!(!p.next_page_url.as_ref().unwrap().contains("page=2")); // only the new one

        assert_eq!(p.path, "https://api.example.com/users");
        assert!(p.prev_page_url.as_ref().unwrap().contains("page=1"));
    }

    #[test]
    fn single_page() {
        let b = PaginateBuilder::new(20, 7, 1, "https://api.example.com/items".into());
        let p = b.build(items(1, 7));

        assert_eq!(p.last_page, 1);
        assert_eq!(p.from, Some(1));
        assert_eq!(p.to, Some(7));
        assert_eq!(p.next_page_url, None);
        assert_eq!(p.prev_page_url, None);
        assert_eq!(p.first_page_url, p.last_page_url);
    }

    #[test]
    fn clamps_zero_values() {
        let b = PaginateBuilder::new(0, 50, 0, "https://x.com".into());
        assert_eq!(b.max, 1);
        assert_eq!(b.page, 1);
        assert_eq!(b.offset(), 0);
    }

    #[test]
    fn offset_is_correct() {
        assert_eq!(
            PaginateBuilder::new(15, 100, 4, "https://x.com".into()).offset(),
            45
        );
    }
}
