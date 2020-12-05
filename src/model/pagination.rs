pub struct Paginator<O: Clone> {
    pub page_count: usize,
    pub current_page: usize,
    offsets: Vec<Option<O>>,
}

impl<O: Clone> Paginator<O> {
    pub fn new(page_size: i64, item_count: i64) -> Self {
        let page_count = (item_count + page_size - 1) / page_size;

        Self {
            page_count: page_count as usize,
            current_page: 1,
            offsets: vec![None; page_count as usize],
        }
    }

    pub fn next(&mut self, offset: Option<O>) -> bool {
        if self.page_count == self.current_page {
            return false;
        }

        // Use the last tag in current page as offset
        self.offsets[self.current_page] = offset;

        self.current_page += 1;
        true
    }

    // Returns if allowed to switch to previous page, and if true, also returns
    // the previous page's offset
    pub fn prev_offset(&mut self) -> Option<&O> {
        let offset_i = self.current_page - 2;

        // New current page
        self.current_page -= 1;

        self.offsets[offset_i].as_ref()
    }
}
