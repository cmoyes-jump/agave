//! Instruction context (input).

use {
    super::error::FixtureError,
    protosol::protos::InstrContext as ProtoInstrContextInner,
    agave_feature_set::FeatureSet,
    solana_account::Account,
    solana_instruction::AccountMeta,
    solana_pubkey::Pubkey,
    solana_stable_layout::stable_instruction::StableInstruction,
};

// Wrapper type to work around orphan rules
pub struct ProtoInstrContext(pub ProtoInstrContextInner);

/// Instruction context fixture.
pub struct InstrContext {
    pub feature_set: FeatureSet,
    pub accounts: Vec<(Pubkey, Account)>,
    pub instruction: StableInstruction,
    pub cu_avail: u64,
}

impl TryFrom<ProtoInstrContext> for InstrContext {
    type Error = FixtureError;

    fn try_from(value: ProtoInstrContext) -> Result<Self, Self::Error> {
        let program_id = Pubkey::new_from_array(
            value.0
                .program_id
                .try_into()
                .map_err(FixtureError::InvalidPubkeyBytes)?,
        );

        let feature_set: FeatureSet = value.0
            .epoch_context
            .as_ref()
            .and_then(|epoch_ctx| epoch_ctx.features.as_ref())
            .map(|fs| (&super::feature_set::ProtoFeatureSet(fs.clone())).into())
            .unwrap_or_default();

        let accounts: Vec<(Pubkey, Account)> = value.0
            .accounts
            .into_iter()
            .map(|acct_state| super::account_state::ProtoAccount(acct_state).try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let instruction_accounts = value.0
            .instr_accounts
            .into_iter()
            .map(|acct| {
                if acct.index as usize >= accounts.len() {
                    return Err(FixtureError::AccountMissingForInstrAccount(
                        acct.index as usize,
                    ));
                }
                Ok(AccountMeta {
                    pubkey: accounts[acct.index as usize].0,
                    is_signer: acct.is_signer,
                    is_writable: acct.is_writable,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        if instruction_accounts.len() > 128 {
            return Err(FixtureError::InvalidFixtureInput);
        }

        let instruction = StableInstruction {
            accounts: instruction_accounts.into(),
            data: value.0.data.into(),
            program_id,
        };

        Ok(Self {
            feature_set,
            accounts,
            instruction,
            cu_avail: value.0.cu_avail,
        })
    }
}
