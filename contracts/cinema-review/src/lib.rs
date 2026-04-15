#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Review {
    pub id: u64,
    pub movie_id: String,
    pub username: String,
    pub content: String,
    pub upvotes: u32,
}

const REVIEW_DATA: Symbol = symbol_short!("REVIEWS");

#[contract]
pub struct MovieReviewContract;

#[contractimpl]
impl MovieReviewContract {
    
    pub fn get_movie_reviews(env: Env, movie_id: String) -> Vec<Review> {
        let all_reviews: Vec<Review> = env.storage().instance().get(&REVIEW_DATA).unwrap_or(Vec::new(&env));
        let mut filtered_reviews = Vec::new(&env);

        for i in 0..all_reviews.len() {
            let review = all_reviews.get(i).unwrap();
            if review.movie_id == movie_id {
                filtered_reviews.push_back(review);
            }
        }
        
        return filtered_reviews;
    }

    pub fn add_review(env: Env, movie_id: String, username: String, content: String) -> String {
        let mut reviews: Vec<Review> = env.storage().instance().get(&REVIEW_DATA).unwrap_or(Vec::new(&env));
        
        let new_review = Review {
            id: env.prng().gen::<u64>(),
            movie_id: movie_id,
            username: username,
            content: content,
            upvotes: 0,
        };
        
        reviews.push_back(new_review);
        env.storage().instance().set(&REVIEW_DATA, &reviews);
        
        return String::from_str(&env, "Review Success Added!");
    }

    pub fn upvote_review(env: Env, id: u64) -> String {
        let mut reviews: Vec<Review> = env.storage().instance().get(&REVIEW_DATA).unwrap_or(Vec::new(&env));

        for i in 0..reviews.len() {
            let mut review = reviews.get(i).unwrap();
            if review.id == id {
                review.upvotes += 1;
                
                reviews.set(i, review); 
                
                env.storage().instance().set(&REVIEW_DATA, &reviews);
                return String::from_str(&env, "Upvote Success Added!");
            }
        }

        return String::from_str(&env, "Review not found");
    }

    pub fn delete_review(env: Env, id: u64) -> String {
        let mut reviews: Vec<Review> = env.storage().instance().get(&REVIEW_DATA).unwrap_or(Vec::new(&env));

        for i in 0..reviews.len() {
            if reviews.get(i).unwrap().id == id {
                reviews.remove(i);
                env.storage().instance().set(&REVIEW_DATA, &reviews);
                return String::from_str(&env, "Review success Deleted   ");
            }
        }

        return String::from_str(&env, "Review Notfound");
    }
}