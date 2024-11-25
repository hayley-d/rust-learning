pub mod state_pattern {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            return Post {
                state: None,
                content: String::new(),
            };
        }

        pub fn add_content(&mut self, content: String) {
            self.content = content;
        }

        pub fn get_content(&self) -> &str {
            return &self.content;
        }

        pub fn set_state(&mut self, state: Box<dyn State>) {
            self.state = Some(state);
        }

        pub fn request_review(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.request_review());
            }
        }
    }

    pub trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn get_content<'a>(&self, post: &'a Post) -> &'a str;
    }

    pub struct Draft {}

    pub struct DraftPost {
        content: String,
    }

    impl DraftPost {
        pub fn new() -> DraftPost {
            return DraftPost {
                content: String::new(),
            };
        }

        pub fn add_content(&mut self, content: &str) {
            self.content.push_str(content);
        }

        pub fn request_review(self) -> PendingReviewPost {
            return PendingReviewPost {
                content: self.content,
            };
        }
    }

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            return Box::new(PendingReview {});
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            return self;
        }
        fn get_content<'a>(&self, post: &'a Post) -> &'a str {
            return "";
        }
    }

    pub struct PendingReview {}

    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            return Post {
                state: Some(Box::new(Published {})),
                content: self.content,
            };
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            return self;
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            return Box::new(Published {});
        }

        fn get_content<'a>(&self, post: &'a Post) -> &'a str {
            return "";
        }
    }

    pub struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            return self;
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            return self;
        }

        fn get_content<'a>(&self, post: &'a Post) -> &'a str {
            return post.get_content();
        }
    }
}
