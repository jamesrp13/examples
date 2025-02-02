pub mod id;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

pub use id::ID;

#[program]
pub mod event_stream {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn ping(ctx: Context<Ping>) -> Result<()> {
        ping::handler(ctx)
    }

    pub fn process_event(ctx: Context<ProcessEvent>) -> Result<()> {
        process_event::handler(ctx)
    }
}
