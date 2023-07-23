use database::{DBConnection, Post, User, Vote};
use tokio::join;

const POST_WEIGHT: u32 = 20;
const VOTE_UPVOTE_WEIGHT: u32 = 2;
const VOTE_DOWNVOTE_WEIGHT: u32 = 1;
const FRIEND_ADD_WEIGHT: u32 = 5;
const COMMENT_WEIGHT: u32 = 3;
const REPLY_WEIGHT: u32 = 2;
const STORY_WEIGHT: u32 = 10;
const PHOTO_PROFILE_WEIGHT: u32 = 30;
const PHOTO_BANNER_WEIGHT: u32 = 20;

pub enum ComputeError {
    UuidNotFound,
    NoArgs,
}

impl From<ComputeError> for i32 {
    fn from(value: ComputeError) -> Self {
        match value {
            ComputeError::UuidNotFound => 1,
            ComputeError::NoArgs => 2,
        }
    }
}

pub(crate) async fn compute(user_id: &str, day: &str) -> Result<u32, ComputeError> {
    let db = DBConnection::new().await;
    db.select_by_id::<User>(&user_id.to_owned()).await.ok_or(ComputeError::UuidNotFound)?;
    let computer = Computer {
        db,
        user_id: user_id.to_owned(),
        day: day.to_owned(),
    };
    let res = join!(computer.posts_score(), computer.vote_score());
    Ok(res.0 + res.1)
}

struct Computer {
    db: DBConnection,
    user_id: String,
    day: String,
}

impl Computer {
    async fn posts_score(&self) -> u32 {
        let posts = self
            .db
            .select_where::<Post>(vec![
                ("owner_id", &self.user_id),
                ("CAST(create_time AS DATE)", &self.day),
            ])
            .await;
        posts.len() as u32 * POST_WEIGHT
    }

    async fn vote_score(&self) -> u32 {
        let votes = self
            .db
            .select_where::<Vote>(vec![("author_id", &self.user_id)])
            .await;
        let mut upvote_score = 0;
        let mut downvote_score = 0;
        votes.iter().for_each(|v| {
            if v.value.is_upvote() {
                upvote_score += 1
            } else if v.value.is_downvote() {
                downvote_score += 1
            }
        });
        upvote_score * VOTE_UPVOTE_WEIGHT + downvote_score * VOTE_DOWNVOTE_WEIGHT
    }
}
