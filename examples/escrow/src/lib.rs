//! Soroban Escrow Contract
//!
//! A simple, trustless escrow between a depositor and a recipient, arbitrated
//! by a designated arbiter.
//!
//! # Roles
//! - **depositor** — deposits tokens into escrow and can cancel before approval
//! - **recipient** — receives tokens once the arbiter approves
//! - **arbiter**   — approves or refunds the escrow
//!
//! # Lifecycle
//! ```text
//!  deposit() → Pending
//!     │
//!     ├─ arbiter calls approve() → recipient receives tokens → Approved
//!     └─ arbiter calls refund()  → depositor receives tokens back → Refunded
//!     └─ depositor calls cancel() (before approval) → Cancelled
//! ```

// Events::publish is deprecated in soroban-sdk v23+ in favour of the
// #[contractevent] macro, which would change the contract spec (event types
// become part of the ABI). Migrating is the right long-term move, but it
// constitutes an interface change — left for a dedicated follow-up PR so that
// the change is clearly attributed and reviewed separately.
#![allow(deprecated)]
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol,
};

// ─── Storage keys ─────────────────────────────────────────────────────────────

const DEPOSITOR: Symbol = symbol_short!("DEPOSITOR");
const RECIPIENT: Symbol = symbol_short!("RECIPIENT");
const ARBITER: Symbol = symbol_short!("ARBITER");
const TOKEN: Symbol = symbol_short!("TOKEN");
const AMOUNT: Symbol = symbol_short!("AMOUNT");
const STATUS: Symbol = symbol_short!("STATUS");

// ─── Events ───────────────────────────────────────────────────────────────────

const EVT_DEPOSITED: Symbol = symbol_short!("deposited");
const EVT_APPROVED: Symbol = symbol_short!("approved");
const EVT_REFUNDED: Symbol = symbol_short!("refunded");
const EVT_CANCELLED: Symbol = symbol_short!("canceld"); // 9 char max

// ─── Types ────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Approved,
    Refunded,
    Cancelled,
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Deposit tokens into escrow. Must be called once; sets all parties.
    ///
    /// # Arguments
    /// * `depositor`  — address funding the escrow and authorising the transfer
    /// * `recipient`  — address that will receive tokens on approval
    /// * `arbiter`    — address that can approve or refund
    /// * `token_address` — SEP-0041 token to hold
    /// * `amount`     — number of tokens to lock
    pub fn deposit(
        env: Env,
        depositor: Address,
        recipient: Address,
        arbiter: Address,
        token_address: Address,
        amount: i128,
    ) {
        // Only callable once
        if env.storage().instance().has(&STATUS) {
            panic!("escrow already initialised");
        }
        if amount <= 0 {
            panic!("amount must be positive");
        }

        depositor.require_auth();

        // Pull tokens from depositor into contract
        let tk = token::Client::new(&env, &token_address);
        tk.transfer(&depositor, env.current_contract_address(), &amount);

        // Persist state
        env.storage().instance().set(&DEPOSITOR, &depositor);
        env.storage().instance().set(&RECIPIENT, &recipient);
        env.storage().instance().set(&ARBITER, &arbiter);
        env.storage().instance().set(&TOKEN, &token_address);
        env.storage().instance().set(&AMOUNT, &amount);
        env.storage()
            .instance()
            .set(&STATUS, &EscrowStatus::Pending);

        env.events().publish((EVT_DEPOSITED,), (depositor, amount));
    }

    /// Arbiter approves: tokens go to recipient.
    pub fn approve(env: Env) {
        let arbiter: Address = env
            .storage()
            .instance()
            .get(&ARBITER)
            .expect("not initialised");
        arbiter.require_auth();
        Self::require_pending(&env);

        let recipient: Address = env.storage().instance().get(&RECIPIENT).unwrap();
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let amount: i128 = env.storage().instance().get(&AMOUNT).unwrap();

        env.storage()
            .instance()
            .set(&STATUS, &EscrowStatus::Approved);

        let tk = token::Client::new(&env, &token_address);
        tk.transfer(&env.current_contract_address(), &recipient, &amount);

        env.events().publish((EVT_APPROVED,), (recipient, amount));
    }

    /// Arbiter refunds: tokens return to depositor.
    pub fn refund(env: Env) {
        let arbiter: Address = env
            .storage()
            .instance()
            .get(&ARBITER)
            .expect("not initialised");
        arbiter.require_auth();
        Self::require_pending(&env);

        let depositor: Address = env.storage().instance().get(&DEPOSITOR).unwrap();
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let amount: i128 = env.storage().instance().get(&AMOUNT).unwrap();

        env.storage()
            .instance()
            .set(&STATUS, &EscrowStatus::Refunded);

        let tk = token::Client::new(&env, &token_address);
        tk.transfer(&env.current_contract_address(), &depositor, &amount);

        env.events().publish((EVT_REFUNDED,), (depositor, amount));
    }

    /// Depositor cancels before arbitration.
    pub fn cancel(env: Env) {
        let depositor: Address = env
            .storage()
            .instance()
            .get(&DEPOSITOR)
            .expect("not initialised");
        depositor.require_auth();
        Self::require_pending(&env);

        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let amount: i128 = env.storage().instance().get(&AMOUNT).unwrap();

        env.storage()
            .instance()
            .set(&STATUS, &EscrowStatus::Cancelled);

        let tk = token::Client::new(&env, &token_address);
        tk.transfer(&env.current_contract_address(), &depositor, &amount);

        env.events().publish((EVT_CANCELLED,), depositor);
    }

    // ── Read helpers ─────────────────────────────────────────────────────────

    pub fn status(env: Env) -> EscrowStatus {
        env.storage()
            .instance()
            .get(&STATUS)
            .unwrap_or(EscrowStatus::Pending)
    }

    pub fn amount(env: Env) -> i128 {
        env.storage().instance().get(&AMOUNT).unwrap_or(0)
    }

    pub fn depositor(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DEPOSITOR)
            .expect("not initialised")
    }

    pub fn recipient(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&RECIPIENT)
            .expect("not initialised")
    }

    pub fn arbiter(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ARBITER)
            .expect("not initialised")
    }

    // ── Internal ─────────────────────────────────────────────────────────────

    fn require_pending(env: &Env) {
        let status: EscrowStatus = env
            .storage()
            .instance()
            .get(&STATUS)
            .expect("not initialised");
        if status != EscrowStatus::Pending {
            panic!("escrow is not pending");
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{token::StellarAssetClient, Env};

    struct TestSetup {
        env: Env,
        contract_id: Address,
        depositor: Address,
        recipient: Address,
        arbiter: Address,
        token: Address,
    }

    fn setup(amount: i128) -> TestSetup {
        let env = Env::default();
        env.mock_all_auths_allowing_non_root_auth();

        let depositor = Address::generate(&env);
        let recipient = Address::generate(&env);
        let arbiter = Address::generate(&env);

        // Deploy and mint a test token
        let token_id = env.register_stellar_asset_contract_v2(depositor.clone());
        let token = token_id.address();
        StellarAssetClient::new(&env, &token).mint(&depositor, &amount);

        let contract_id = env.register(EscrowContract, ());

        TestSetup {
            env,
            contract_id,
            depositor,
            recipient,
            arbiter,
            token,
        }
    }

    fn deposit_helper(s: &TestSetup, amount: i128) {
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        client.deposit(&s.depositor, &s.recipient, &s.arbiter, &s.token, &amount);
    }

    // ── happy path: approve ───────────────────────────────────────────────────

    #[test]
    fn test_approve_sends_to_recipient() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 1_000);

        assert_eq!(client.status(), EscrowStatus::Pending);
        client.approve();
        assert_eq!(client.status(), EscrowStatus::Approved);

        // Recipient received tokens
        let tk = soroban_sdk::token::Client::new(&s.env, &s.token);
        assert_eq!(tk.balance(&s.recipient), 1_000);
        assert_eq!(tk.balance(&s.contract_id), 0);
    }

    // ── happy path: refund ────────────────────────────────────────────────────

    #[test]
    fn test_refund_returns_to_depositor() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 1_000);

        client.refund();
        assert_eq!(client.status(), EscrowStatus::Refunded);

        let tk = soroban_sdk::token::Client::new(&s.env, &s.token);
        assert_eq!(tk.balance(&s.depositor), 1_000);
        assert_eq!(tk.balance(&s.contract_id), 0);
    }

    // ── happy path: cancel by depositor ──────────────────────────────────────

    #[test]
    fn test_cancel_returns_to_depositor() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 1_000);

        client.cancel();
        assert_eq!(client.status(), EscrowStatus::Cancelled);

        let tk = soroban_sdk::token::Client::new(&s.env, &s.token);
        assert_eq!(tk.balance(&s.depositor), 1_000);
    }

    // ── double deposit panics ─────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "escrow already initialised")]
    fn test_double_deposit_panics() {
        let s = setup(2_000);
        deposit_helper(&s, 1_000);
        deposit_helper(&s, 1_000); // second call must panic
    }

    // ── approve after approve panics ──────────────────────────────────────────

    #[test]
    #[should_panic(expected = "escrow is not pending")]
    fn test_approve_twice_panics() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 1_000);
        client.approve();
        client.approve(); // must panic
    }

    // ── refund after approve panics ───────────────────────────────────────────

    #[test]
    #[should_panic(expected = "escrow is not pending")]
    fn test_refund_after_approve_panics() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 1_000);
        client.approve();
        client.refund(); // must panic
    }

    // ── cancel after approve panics ───────────────────────────────────────────

    #[test]
    #[should_panic(expected = "escrow is not pending")]
    fn test_cancel_after_approve_panics() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 1_000);
        client.approve();
        client.cancel(); // must panic
    }

    // ── read helpers return correct values ────────────────────────────────────

    #[test]
    fn test_read_helpers() {
        let s = setup(500);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        deposit_helper(&s, 500);

        assert_eq!(client.amount(), 500);
        assert_eq!(client.depositor(), s.depositor);
        assert_eq!(client.recipient(), s.recipient);
        assert_eq!(client.arbiter(), s.arbiter);
    }

    // ── zero amount panics ───────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn test_zero_amount_panics() {
        let s = setup(1_000);
        let client = EscrowContractClient::new(&s.env, &s.contract_id);
        client.deposit(&s.depositor, &s.recipient, &s.arbiter, &s.token, &0);
    }
}
