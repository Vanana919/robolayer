use anchor_lang::prelude::*;
declare_id!("RBLYr7mRXT4oFqJzKw8PqWnLkGpMR5C3aY6hN9qFau2");

#[program]
pub mod robolayer {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
