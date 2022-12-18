use crate::{CommentBlock, Driver, Walker};

mod drive_function;
mod drive_types;
mod driver_basics;

#[test]
fn drive_comment_block() {
    let mut walker = Walker::from("/// This is a comment");
    assert_eq!(
        CommentBlock("This is a comment".to_string()),
        CommentBlock::drive(&mut walker).unwrap()
    );

    let mut walker = Walker::from("/// This is a comment\n");
    assert_eq!(
        CommentBlock("This is a comment".to_string()),
        CommentBlock::drive(&mut walker).unwrap()
    );
}
