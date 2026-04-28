#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, vec, Address, BytesN,
    Env, Symbol, Vec,
};

const DEFAULT_REP: u64 = 50;
const MAX_REP: u64 = 100;
const REP_REWARD: u64 = 1;
const REP_PENALTY: u64 = 10;

const COOLDOWN_SECONDS: u64 = 3600;
const MAX_ACTIVITY_POINTS: u64 = 10_000;
const LEADERBOARD_LIMIT: u32 = 10;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Activity(Symbol),
    UserPoints(Address),
    UserStamps(Address),
    UserRep(Address),
    UserLastTime(Address, Symbol),
    UsedProof(BytesN<32>),
    Leaderboard,
}

#[contracttype]
#[derive(Clone)]
pub struct Activity {
    pub points: u64,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct LeaderboardEntry {
    pub addr: Address,
    pub score: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAdmin = 3,
    BadActivity = 4,
    ActivityInactive = 5,
    ProofUsed = 6,
    Cooldown = 7,
    InvalidPoints = 8,
    Overflow = 9,
}

#[contract]
pub struct EcoImpactSystem;

#[contractimpl]
impl EcoImpactSystem {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotInitialized))
    }

    pub fn transfer_admin(env: Env, admin: Address, new_admin: Address) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    pub fn register_activity(env: Env, admin: Address, name: Symbol, points: u64) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        if points == 0 || points > MAX_ACTIVITY_POINTS {
            panic_with_error!(&env, Error::InvalidPoints);
        }

        let activity = Activity {
            points,
            active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Activity(name), &activity);
    }

    pub fn set_activity_status(env: Env, admin: Address, name: Symbol, active: bool) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let key = DataKey::Activity(name);

        let mut activity: Activity = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, Error::BadActivity));

        activity.active = active;

        env.storage().persistent().set(&key, &activity);
    }

    pub fn get_activity(env: Env, name: Symbol) -> Activity {
        env.storage()
            .persistent()
            .get(&DataKey::Activity(name))
            .unwrap_or_else(|| panic_with_error!(&env, Error::BadActivity))
    }

    pub fn submit_activity(env: Env, user: Address, act: Symbol, proof: BytesN<32>) {
        user.require_auth();

        let activity: Activity = env
            .storage()
            .persistent()
            .get(&DataKey::Activity(act.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::BadActivity));

        if !activity.active {
            panic_with_error!(&env, Error::ActivityInactive);
        }

        let proof_key = DataKey::UsedProof(proof);

        if env.storage().persistent().has(&proof_key) {
            Self::penalize_user(&env, &user);
            panic_with_error!(&env, Error::ProofUsed);
        }

        let now = env.ledger().timestamp();
        let last_time_key = DataKey::UserLastTime(user.clone(), act);

        if let Some(last_time) = env
            .storage()
            .persistent()
            .get::<DataKey, u64>(&last_time_key)
        {
            let next_allowed = last_time
                .checked_add(COOLDOWN_SECONDS)
                .unwrap_or_else(|| panic_with_error!(&env, Error::Overflow));

            if now < next_allowed {
                panic_with_error!(&env, Error::Cooldown);
            }
        }

        env.storage().persistent().set(&proof_key, &true);
        env.storage().persistent().set(&last_time_key, &now);

        let points_key = DataKey::UserPoints(user.clone());
        let old_points: u64 = env.storage().persistent().get(&points_key).unwrap_or(0);

        let new_points = old_points
            .checked_add(activity.points)
            .unwrap_or_else(|| panic_with_error!(&env, Error::Overflow));

        env.storage().persistent().set(&points_key, &new_points);

        let stamps_key = DataKey::UserStamps(user.clone());
        let old_stamps: u64 = env.storage().persistent().get(&stamps_key).unwrap_or(0);

        let new_stamps = old_stamps
            .checked_add(1)
            .unwrap_or_else(|| panic_with_error!(&env, Error::Overflow));

        env.storage().persistent().set(&stamps_key, &new_stamps);

        Self::reward_reputation(&env, &user);
        Self::update_leaderboard(&env, user, new_points);
    }

    pub fn get_points(env: Env, user: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::UserPoints(user))
            .unwrap_or(0)
    }

    pub fn get_stamps(env: Env, user: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::UserStamps(user))
            .unwrap_or(0)
    }

    pub fn get_rep(env: Env, user: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::UserRep(user))
            .unwrap_or(DEFAULT_REP)
    }

    pub fn get_board(env: Env) -> Vec<LeaderboardEntry> {
        env.storage()
            .persistent()
            .get(&DataKey::Leaderboard)
            .unwrap_or(vec![&env])
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized));

        if &admin != caller {
            panic_with_error!(env, Error::NotAdmin);
        }
    }

    fn penalize_user(env: &Env, user: &Address) {
        let key = DataKey::UserRep(user.clone());

        let old_rep: u64 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(DEFAULT_REP);

        let new_rep = old_rep.saturating_sub(REP_PENALTY);

        env.storage().persistent().set(&key, &new_rep);
    }

    fn reward_reputation(env: &Env, user: &Address) {
        let key = DataKey::UserRep(user.clone());

        let old_rep: u64 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(DEFAULT_REP);

        if old_rep >= MAX_REP {
            return;
        }

        let mut new_rep = old_rep
            .checked_add(REP_REWARD)
            .unwrap_or_else(|| panic_with_error!(env, Error::Overflow));

        if new_rep > MAX_REP {
            new_rep = MAX_REP;
        }

        env.storage().persistent().set(&key, &new_rep);
    }

    fn update_leaderboard(env: &Env, user: Address, score: u64) {
        let mut board: Vec<LeaderboardEntry> = env
            .storage()
            .persistent()
            .get(&DataKey::Leaderboard)
            .unwrap_or(vec![env]);

        let mut found = false;
        let mut i = 0;

        while i < board.len() {
            let entry = board.get(i).unwrap();

            if entry.addr == user {
                board.set(
                    i,
                    LeaderboardEntry {
                        addr: user.clone(),
                        score,
                    },
                );

                found = true;
                break;
            }

            i += 1;
        }

        if !found {
            board.push_back(LeaderboardEntry {
                addr: user,
                score,
            });
        }

        Self::sort_leaderboard(&mut board);

        while board.len() > LEADERBOARD_LIMIT {
            board.pop_back();
        }

        env.storage()
            .persistent()
            .set(&DataKey::Leaderboard, &board);
    }

    fn sort_leaderboard(board: &mut Vec<LeaderboardEntry>) {
        let len = board.len();

        if len <= 1 {
            return;
        }

        let mut i = 0;

        while i < len {
            let mut j = 0;

            while j + 1 < len {
                let left = board.get(j).unwrap();
                let right = board.get(j + 1).unwrap();

                if right.score > left.score {
                    board.set(j, right);
                    board.set(j + 1, left);
                }

                j += 1;
            }

            i += 1;
        }
    }
}
