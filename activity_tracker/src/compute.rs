use database::{DBConnection, TableType, User};

pub(crate) const POST_WEIGHT: u32 = 20;
pub(crate) const VOTE_WEIGHT: u32 = 1;
pub(crate) const FRIEND_ADD_WEIGHT: u32 = 5;
pub(crate) const COMMENT_WEIGHT: u32 = 3;
pub(crate) const REPLY_WEIGHT: u32 = 2;
pub(crate) const STORY_WEIGHT: u32 = 10;
pub(crate) const PHOTO_PROFILE_WEIGHT: u32 = 30;
pub(crate) const PHOTO_BANNER_WEIGHT: u32 = 20;

pub(crate) async fn compute(user_id: &String) -> Option<u32> {
    let mut db = DBConnection::new().await;
    let user = db.select_id::<User>(TableType::Users, user_id).await?;
    Some(
        POST_WEIGHT
            + VOTE_WEIGHT
            + FRIEND_ADD_WEIGHT
            + COMMENT_WEIGHT
            + REPLY_WEIGHT
            + STORY_WEIGHT
            + PHOTO_PROFILE_WEIGHT
            + PHOTO_BANNER_WEIGHT,
    )
}
