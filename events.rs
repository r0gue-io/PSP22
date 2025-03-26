use ink::H160;

/// Event emitted when allowance by `owner` to `spender` changes.
#[ink::event]
pub struct Approval {
    /// Account providing allowance.
    #[ink(topic)]
    pub owner: H160,
    /// Allowance beneficiary.
    #[ink(topic)]
    pub spender: H160,
    /// New allowance amount.
    pub amount: u128,
}

/// Event emitted when transfer of tokens occurs.
#[ink::event]
pub struct Transfer {
    /// Transfer sender. `None` in case of minting new tokens.
    #[ink(topic)]
    pub from: Option<H160>,
    /// Transfer recipient. `None` in case of burning tokens.
    #[ink(topic)]
    pub to: Option<H160>,
    /// Amount of tokens transferred (or minted/burned).
    pub value: u128,
}
