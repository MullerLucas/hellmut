pub struct StyleSheet {
    content: &'static str,
}

impl StyleSheet {
    pub const fn new(content: &'static str) -> Self {
        Self {
            content
        }
    }

    pub const fn content(&self) -> &str {
        self.content
    }
}


