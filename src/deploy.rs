use web3::types::Address;
use web3::Web3;

/// Development account address if parity is using with the `dev` chain
pub const DEV_ACCOUNT_ADDR: &str = "00a329c0648769a73afac7f9381e08fb43dbea72";

/// Password for the dev account
pub const DEV_ACCOUNT_PASSWORD: &str = "";

pub const NODE_URL: &str = "http://localhost:8545";

/// File to write the address of the deployed contract to
pub const CONTRACT_ADDRESS_FILE: &str = "./.oscoin_ledger_address";

/// Development account address if parity is using with the `dev` chain
pub fn dev_account_addr() -> Address {
    DEV_ACCOUNT_ADDR
        .parse()
        .expect("address is hardcoded and valid")
}

pub fn unlock_dev_account<T: web3::Transport>(
    web3: &Web3<T>,
) -> web3::helpers::CallFuture<bool, T::Out> {
    web3.personal()
        .unlock_account(dev_account_addr(), DEV_ACCOUNT_PASSWORD, None)
}
