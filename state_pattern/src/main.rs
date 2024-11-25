use state_pattern::state::*;

fn main() {
    let mut post = Post::new();

    post.add_content(String::from("Sloths are the cutest!"));

    post.approve();

    post.publish();
}
