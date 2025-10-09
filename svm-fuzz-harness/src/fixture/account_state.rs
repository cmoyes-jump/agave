use {
    super::error::FixtureError,
    protosol::protos::AcctState,
    solana_account::Account,
    solana_pubkey::Pubkey,
};

// Wrapper type to work around orphan rules
pub struct ProtoAccount(pub AcctState);

// Default `rent_epoch` field value for all accounts.
const RENT_EXEMPT_RENT_EPOCH: u64 = u64::MAX;

impl TryFrom<ProtoAccount> for (Pubkey, Account) {
    type Error = FixtureError;

    fn try_from(value: ProtoAccount) -> Result<Self, Self::Error> {
        let AcctState {
            address,
            owner,
            lamports,
            data,
            executable,
            ..
        } = value.0;

        let pubkey = Pubkey::try_from(address).map_err(FixtureError::InvalidPubkeyBytes)?;
        let owner = Pubkey::try_from(owner).map_err(FixtureError::InvalidPubkeyBytes)?;

        Ok((
            pubkey,
            Account {
                data,
                executable,
                lamports,
                owner,
                rent_epoch: RENT_EXEMPT_RENT_EPOCH,
            },
        ))
    }
}

impl From<(Pubkey, Account)> for ProtoAccount {
    fn from(value: (Pubkey, Account)) -> Self {
        let Account {
            lamports,
            data,
            owner,
            executable,
            ..
        } = value.1;

        ProtoAccount(AcctState {
            address: value.0.to_bytes().to_vec(),
            owner: owner.to_bytes().to_vec(),
            lamports,
            data,
            executable,
            seed_addr: None,
        })
    }
}
