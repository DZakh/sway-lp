contract;

use std::{
    asset::{
        burn,
        mint,
        mint_to,
        transfer,
    },
    bytes::Bytes,
    call_frames::{
        msg_asset_id,
    },
    constants::ZERO_B256,
    context::msg_amount,
};

abi Pool {
    #[payable]
    fn deposit(recipient: Identity);
    #[payable]
    fn withdraw(recipient: Identity);
}

struct DepositEvent {
    recipient: Identity,
    amount: u64,
}

impl Pool for Contract {
    #[payable]
    fn deposit(recipient: Identity) {
        require(AssetId::base() == msg_asset_id(), "wrong-asset-id");

        let amount = msg_amount();
        assert(0 < amount);

        // Mint some LP token based upon the amount of the base token.
        mint_to(recipient, ZERO_B256, amount);

        log(DepositEvent {
            recipient,
            amount,
        })
    }

    #[payable]
    fn withdraw(recipient: Identity) {
        require(
            AssetId::new(ContractId::this(), ZERO_B256) == msg_asset_id(),
            "wrong-asset-id",
        );

        let amount = msg_amount();
        assert(0 < amount);

        transfer(recipient, AssetId::base(), amount);
        burn(ZERO_B256, amount);
    }
}
