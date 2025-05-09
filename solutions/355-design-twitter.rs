use std::collections::{HashMap, HashSet};
use std::iter::once;

struct Twitter {
    follows: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    tweet_id_counter: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Self {
            follows: HashMap::new(),
            tweets: HashMap::new(),
            tweet_id_counter: 0,
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.entry(user_id)
            .and_modify(|e| e.push((self.tweet_id_counter, tweet_id)))
            .or_insert(vec![(self.tweet_id_counter, tweet_id)]);
        
        self.tweet_id_counter += 1;
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id)
            .and_modify(|e| { e.insert(followee_id); })
            .or_insert(HashSet::from([followee_id]));
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(mut following) = self.follows.get_mut(&follower_id){
            following.remove(&followee_id);
        }
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        // println!("follows: {:?}", self.follows.get(&user_id));
        // println!("follow tweets: {:?}", self.follows.get(&user_id).map(|f| {
        //     f.iter().map(|f| self.tweets.get(&f))
        // }));
        
        let mut most_recent_tweets_from_all_followees =
            self.follows.get(&user_id)
                .unwrap_or(&HashSet::with_capacity(0))
                .iter()
                .chain(once(&user_id))
                .filter_map(|f| {
                    self.tweets.get(&f)
                        .map(|t| &t[t.len().checked_sub(10).unwrap_or(0)..])
                })
                .flatten()
                .collect::<Vec<_>>();
        most_recent_tweets_from_all_followees.sort_unstable_by(|a, b| b.cmp(&a));
        most_recent_tweets_from_all_followees.iter().take(10).map(|(_, id)| *id).collect()
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */