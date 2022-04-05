use crate::*;

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct AssetView {
    pub token_id: TokenId,
    #[serde(with = "u128_dec_format")]
    pub balance: Balance,
    /// The number of shares this account holds in the corresponding asset pool
    pub shares: Shares,
    /// The current APR for this asset (either supply or borrow APR).
    pub apy: BigDecimal,
}

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountDetailedView {
    pub account_id: AccountId,
    /// A list of assets that are supplied by the account (but not used a collateral).
    pub supplied: Vec<AssetView>,
    /// A list of assets that are used as a collateral.
    pub collateral: Vec<AssetView>,
    /// A list of assets that are borrowed.
    pub borrowed: Vec<AssetView>,
    /// Account farms
    pub farms: Vec<AccountFarmView>,
}

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountFarmView {
    pub farm_id: FarmId,
    pub rewards: Vec<AccountFarmRewardView>,
}

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountFarmRewardView {
    pub reward_token_id: TokenId,
    pub asset_farm_reward: AssetFarmReward,
    #[serde(with = "u128_dec_format")]
    pub boosted_shares: Balance,
    #[serde(with = "u128_dec_format")]
    pub unclaimed_amount: Balance,
}

impl Contract {
    pub fn account_into_detailed_view(&self, account: Account) -> AccountDetailedView {
        let farms = account
            .farms
            .keys()
            .map(|farm_id| {
                let mut asset_farm = self.internal_unwrap_asset_farm(&farm_id, true);
                let (account_farm, new_rewards, inactive_rewards) =
                    self.internal_account_farm_claim(&account, &farm_id, &asset_farm);
                AccountFarmView {
                    farm_id,
                    rewards: account_farm
                        .rewards
                        .into_iter()
                        .map(|(token_id, AccountFarmReward { boosted_shares, .. })| {
                            (token_id, boosted_shares)
                        })
                        .chain(inactive_rewards)
                        .map(|(reward_token_id, boosted_shares)| {
                            let asset_farm_reward = asset_farm
                                .rewards
                                .remove(&reward_token_id)
                                .or_else(|| {
                                    asset_farm
                                        .internal_get_inactive_asset_farm_reward(&reward_token_id)
                                })
                                .unwrap();
                            let unclaimed_amount = new_rewards
                                .iter()
                                .find(|(token_id, _)| token_id == &reward_token_id)
                                .map(|(_, amount)| *amount)
                                .unwrap_or(0);
                            AccountFarmRewardView {
                                reward_token_id,
                                asset_farm_reward,
                                boosted_shares,
                                unclaimed_amount,
                            }
                        })
                        .collect(),
                }
            })
            .collect();
        AccountDetailedView {
            account_id: account.account_id,
            supplied: unordered_map_pagination(&account.supplied, None, None)
                .into_iter()
                .map(|(token_id, AccountAsset { shares })| {
                    self.get_asset_view(token_id, shares, false)
                })
                .collect(),
            collateral: account
                .collateral
                .into_iter()
                .map(|CollateralAsset { token_id, shares }| {
                    self.get_asset_view(token_id, shares, false)
                })
                .collect(),
            borrowed: account
                .borrowed
                .into_iter()
                .map(|BorrowedAsset { token_id, shares }| {
                    self.get_asset_view(token_id, shares, true)
                })
                .collect(),
            farms,
        }
    }

    fn get_asset_view(&self, token_id: TokenId, shares: Shares, is_borrowing: bool) -> AssetView {
        let asset = self.internal_unwrap_asset(&token_id);
        let apy = if is_borrowing {
            log!("get_asset_view: ===> Get borrow apy");
            asset.get_borrow_apy()
        } else {
            log!("get_asset_view: ===> Get supply apy");
            asset.get_supply_apy()
        };
        let balance = if is_borrowing {
            asset.borrowed.shares_to_amount(shares, true)
        } else {
            asset.supplied.shares_to_amount(shares, false)
        };

        AssetView {
            token_id,
            balance,
            shares,
            apy,
        }
    }
}
