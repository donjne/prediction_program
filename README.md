# Prediction Market for Memecoins

This program allows users to create a prediction market, place predictions, and resolve predictions. It also includes a reward system for the correct predictions.

## Installation Process

### Prerequisites

Ensure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor](https://www.anchor-lang.com/docs/installation)

### Clone the Repository

Clone the repository containing the `hello_anchor` program:

```shell
git clone https://github.com/donjne/prediction_program.git
cd hello_anchor
```

### Build the Project

Build the project using anchor:

```shell
anchor build
```

### Deploy the project

Build the project using anchor:

```shell
anchor deploy
```

## Usage

### Initializing the Prediction Market

Call this function to set up the initial state of the prediction market.

```rust
pub fn initialize_prediction_market(ctx: Context<InitializePredictionMarket>) -> ProgramResult {
    // Initializes the state data for the prediction market
}
```

### Placing a Bet

This function allows a user to place a bet on the prediction market.

```rust
pub fn place_bet(
    ctx: Context<BetParams>, 
    prediction_name: String, 
    prediction: PredictionType, 
    amount: u64, 
    start_time: i64, 
    expiration_time: i64
) -> ProgramResult {
    // Places a new bet, transferring tokens to the vault and recording the bet details
}
```

### Resolving a Bet

Resolve an existing bet after the expiration time has passed.

```rust
pub fn resolve_bet(ctx: Context<BetParams>, bet_index: u64) -> ProgramResult {
    // Resolves the bet, checking if the expiration time has passed, 
    // and transfers the original amount plus a reward back to the user
}
```

### Helper Function: Calculate Reward

Calculates a 10% reward based on the bet amount.

```rust
pub fn calculate_reward(amount: u64) -> u64 {
    amount / 10 // 10% reward
}
```

## Structs and Enums

### PredictionMarket

Stores all bets in the prediction market.

```rust
#[account]
pub struct PredictionMarket {
    pub bets: Vec<Bet>,
}
```

### Bet

Represents a single bet in the prediction market.

```rust

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Bet {
    pub user: Pubkey,
    pub prediction_name: String,
    pub prediction: PredictionType,
    pub amount: u64,
    pub start_time: i64,
    pub expiration_time: i64,
    pub is_open: bool,
}
```

### PredictionType

Defines the types of predictions that can be made.

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PredictionType {
    Higher,
    Lower,
}
```

## Accounts

### InitializePredictionMarket

Sets up the initial state for the prediction market.

```rust
#[derive(Accounts)]
pub struct InitializePredictionMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 8, seeds = [b"predictionmarket"], bump)]
    pub state_data: Account<'info, PredictionMarket>,
    pub system_program: Program<'info, System>,
}
```

### BetParams

Defines the parameters for placing and resolving bets.

```rust
#[derive(Accounts)]
pub struct BetParams<'info> {
    #[account(mut, seeds = [b"predictionmarket"], bump)]
    pub state_data: Account<'info, PredictionMarket>,
    #[account(
        mut,
        seeds = [b"vault".as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut,  seeds = [b"tokenaccount"], bump)]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
```
