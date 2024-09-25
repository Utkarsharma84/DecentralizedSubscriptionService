#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, symbol_short};

// Structure for storing subscription details
#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub subscriber_id: u64,
    pub creator_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
}

// For mapping unique subscription to subscription details
#[contracttype]
pub enum SubscriptionBook {
    Subscription(u64)
}

const COUNT_SUBSCRIPTIONS: Symbol = symbol_short!("C_SUBS");

#[contract]
pub struct SubscriptionServiceContract;

#[contractimpl]
impl SubscriptionServiceContract {

    // Function to create a new subscription
    pub fn create_subscription(env: Env, subscriber_id: u64, creator_id: u64, duration: u64) -> u64 {
        let mut count_subs: u64 = env.storage().instance().get(&COUNT_SUBSCRIPTIONS).unwrap_or(0);
        count_subs += 1;

        let time_now = env.ledger().timestamp();
        let end_time = time_now + duration;

        let subscription = Subscription {
            subscriber_id,
            creator_id,
            start_time: time_now,
            end_time,
            is_active: true,
        };

        env.storage().instance().set(&SubscriptionBook::Subscription(count_subs), &subscription);
        env.storage().instance().set(&COUNT_SUBSCRIPTIONS, &count_subs);

        env.storage().instance().extend_ttl(5000, 5000);

        count_subs // Return the subscription ID
    }

    // Function to check the status of a subscription by ID
    pub fn check_subscription_status(env: Env, subscription_id: u64) -> Subscription {
        let key = SubscriptionBook::Subscription(subscription_id);
        env.storage().instance().get(&key).unwrap_or(Subscription {
            subscriber_id: 0,
            creator_id: 0,
            start_time: 0,
            end_time: 0,
            is_active: false,
        })
    }
}
