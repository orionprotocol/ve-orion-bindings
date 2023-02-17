pub use i_convex_base_reward_pool::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_convex_base_reward_pool {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IConvexBaseRewardPool was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
    use ::ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ::ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ::ethers::providers::Middleware;
    use std::sync::Arc;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_reward\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addExtraReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"clearExtraRewards\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"donate\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"duration\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earned\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraRewards\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraRewardsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_claimExtras\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"historicalRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastTimeRewardApplicable\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastUpdateTime\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"newRewardRatio\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"operator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"periodFinish\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pid\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_rewards\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queueNewRewards\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"queuedRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardPerToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardPerTokenStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stake\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stakeAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_for\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stakeFor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stakingToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"userRewardPerTokenPaid\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAllAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAndUnwrap\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICONVEXBASEREWARDPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IConvexBaseRewardPool<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IConvexBaseRewardPool<M> {
        fn clone(&self) -> Self {
            IConvexBaseRewardPool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IConvexBaseRewardPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IConvexBaseRewardPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IConvexBaseRewardPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IConvexBaseRewardPool<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ICONVEXBASEREWARDPOOL_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addExtraReward` (0x5e43c47b) function
        pub fn add_extra_reward(
            &self,
            reward: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([94, 67, 196, 123], reward)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearExtraRewards` (0x0569d388) function
        pub fn clear_extra_rewards(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 105, 211, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentRewards` (0x901a7d53) function
        pub fn current_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 26, 125, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `donate` (0xf14faf6f) function
        pub fn donate(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([241, 79, 175, 111], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `duration` (0x0fb5a6b4) function
        pub fn duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 181, 166, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earned` (0x008cc262) function
        pub fn earned(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 140, 194, 98], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extraRewards` (0x40c35446) function
        pub fn extra_rewards(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([64, 195, 84, 70], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extraRewardsLength` (0xd55a23f4) function
        pub fn extra_rewards_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([213, 90, 35, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReward` (0x3d18b912) function
        pub fn get_reward(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 24, 185, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReward` (0x7050ccd9) function
        pub fn get_reward_with_account_and_claim_extras(
            &self,
            account: ::ethers::core::types::Address,
            claim_extras: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([112, 80, 204, 217], (account, claim_extras))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `historicalRewards` (0x262d3d6d) function
        pub fn historical_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 45, 61, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastTimeRewardApplicable` (0x80faa57d) function
        pub fn last_time_reward_applicable(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 250, 165, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdateTime` (0xc8f33c91) function
        pub fn last_update_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([200, 243, 60, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `newRewardRatio` (0x6c8bcee8) function
        pub fn new_reward_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([108, 139, 206, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operator` (0x570ca735) function
        pub fn operator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([87, 12, 167, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `periodFinish` (0xebe2b12b) function
        pub fn period_finish(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 226, 177, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pid` (0xf1068454) function
        pub fn pid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 6, 132, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueNewRewards` (0x590a41f5) function
        pub fn queue_new_rewards(
            &self,
            rewards: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([89, 10, 65, 245], rewards)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queuedRewards` (0x63d38c3b) function
        pub fn queued_rewards(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([99, 211, 140, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardManager` (0x0f4ef8a6) function
        pub fn reward_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([15, 78, 248, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerToken` (0xcd3daf9d) function
        pub fn reward_per_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 61, 175, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenStored` (0xdf136d65) function
        pub fn reward_per_token_stored(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([223, 19, 109, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardRate` (0x7b0a47ee) function
        pub fn reward_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([123, 10, 71, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardToken` (0xf7c618c1) function
        pub fn reward_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewards` (0x0700037d) function
        pub fn rewards(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 0, 3, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0xa694fc3a) function
        pub fn stake(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 148, 252, 58], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeAll` (0x8dcb4061) function
        pub fn stake_all(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([141, 203, 64, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeFor` (0x2ee40908) function
        pub fn stake_for(
            &self,
            for_: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([46, 228, 9, 8], (for_, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakingToken` (0x72f702f3) function
        pub fn staking_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([114, 247, 2, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userRewardPerTokenPaid` (0x8b876347) function
        pub fn user_reward_per_token_paid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 135, 99, 71], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x38d07436) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
            claim: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([56, 208, 116, 54], (amount, claim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawAll` (0x1c1c6fe5) function
        pub fn withdraw_all(
            &self,
            claim: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 28, 111, 229], claim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawAllAndUnwrap` (0x49f039a2) function
        pub fn withdraw_all_and_unwrap(
            &self,
            claim: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 240, 57, 162], claim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawAndUnwrap` (0xc32e7202) function
        pub fn withdraw_and_unwrap(
            &self,
            amount: ::ethers::core::types::U256,
            claim: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 46, 114, 2], (amount, claim))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IConvexBaseRewardPool<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addExtraReward` function with signature `addExtraReward(address)` and selector `0x5e43c47b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "addExtraReward", abi = "addExtraReward(address)")]
    pub struct AddExtraRewardCall {
        pub reward: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `clearExtraRewards` function with signature `clearExtraRewards()` and selector `0x0569d388`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "clearExtraRewards", abi = "clearExtraRewards()")]
    pub struct ClearExtraRewardsCall;
    ///Container type for all input parameters for the `currentRewards` function with signature `currentRewards()` and selector `0x901a7d53`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "currentRewards", abi = "currentRewards()")]
    pub struct CurrentRewardsCall;
    ///Container type for all input parameters for the `donate` function with signature `donate(uint256)` and selector `0xf14faf6f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "donate", abi = "donate(uint256)")]
    pub struct DonateCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `duration` function with signature `duration()` and selector `0x0fb5a6b4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "duration", abi = "duration()")]
    pub struct DurationCall;
    ///Container type for all input parameters for the `earned` function with signature `earned(address)` and selector `0x008cc262`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "earned", abi = "earned(address)")]
    pub struct EarnedCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `extraRewards` function with signature `extraRewards(uint256)` and selector `0x40c35446`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "extraRewards", abi = "extraRewards(uint256)")]
    pub struct ExtraRewardsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `extraRewardsLength` function with signature `extraRewardsLength()` and selector `0xd55a23f4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "extraRewardsLength", abi = "extraRewardsLength()")]
    pub struct ExtraRewardsLengthCall;
    ///Container type for all input parameters for the `getReward` function with signature `getReward()` and selector `0x3d18b912`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReward", abi = "getReward()")]
    pub struct GetRewardCall;
    ///Container type for all input parameters for the `getReward` function with signature `getReward(address,bool)` and selector `0x7050ccd9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReward", abi = "getReward(address,bool)")]
    pub struct GetRewardWithAccountAndClaimExtrasCall {
        pub account: ::ethers::core::types::Address,
        pub claim_extras: bool,
    }
    ///Container type for all input parameters for the `historicalRewards` function with signature `historicalRewards()` and selector `0x262d3d6d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "historicalRewards", abi = "historicalRewards()")]
    pub struct HistoricalRewardsCall;
    ///Container type for all input parameters for the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable()` and selector `0x80faa57d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastTimeRewardApplicable", abi = "lastTimeRewardApplicable()")]
    pub struct LastTimeRewardApplicableCall;
    ///Container type for all input parameters for the `lastUpdateTime` function with signature `lastUpdateTime()` and selector `0xc8f33c91`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastUpdateTime", abi = "lastUpdateTime()")]
    pub struct LastUpdateTimeCall;
    ///Container type for all input parameters for the `newRewardRatio` function with signature `newRewardRatio()` and selector `0x6c8bcee8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "newRewardRatio", abi = "newRewardRatio()")]
    pub struct NewRewardRatioCall;
    ///Container type for all input parameters for the `operator` function with signature `operator()` and selector `0x570ca735`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "operator", abi = "operator()")]
    pub struct OperatorCall;
    ///Container type for all input parameters for the `periodFinish` function with signature `periodFinish()` and selector `0xebe2b12b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "periodFinish", abi = "periodFinish()")]
    pub struct PeriodFinishCall;
    ///Container type for all input parameters for the `pid` function with signature `pid()` and selector `0xf1068454`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "pid", abi = "pid()")]
    pub struct PidCall;
    ///Container type for all input parameters for the `queueNewRewards` function with signature `queueNewRewards(uint256)` and selector `0x590a41f5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "queueNewRewards", abi = "queueNewRewards(uint256)")]
    pub struct QueueNewRewardsCall {
        pub rewards: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `queuedRewards` function with signature `queuedRewards()` and selector `0x63d38c3b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "queuedRewards", abi = "queuedRewards()")]
    pub struct QueuedRewardsCall;
    ///Container type for all input parameters for the `rewardManager` function with signature `rewardManager()` and selector `0x0f4ef8a6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "rewardManager", abi = "rewardManager()")]
    pub struct RewardManagerCall;
    ///Container type for all input parameters for the `rewardPerToken` function with signature `rewardPerToken()` and selector `0xcd3daf9d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "rewardPerToken", abi = "rewardPerToken()")]
    pub struct RewardPerTokenCall;
    ///Container type for all input parameters for the `rewardPerTokenStored` function with signature `rewardPerTokenStored()` and selector `0xdf136d65`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "rewardPerTokenStored", abi = "rewardPerTokenStored()")]
    pub struct RewardPerTokenStoredCall;
    ///Container type for all input parameters for the `rewardRate` function with signature `rewardRate()` and selector `0x7b0a47ee`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "rewardRate", abi = "rewardRate()")]
    pub struct RewardRateCall;
    ///Container type for all input parameters for the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "rewardToken", abi = "rewardToken()")]
    pub struct RewardTokenCall;
    ///Container type for all input parameters for the `rewards` function with signature `rewards(address)` and selector `0x0700037d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "rewards", abi = "rewards(address)")]
    pub struct RewardsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `stake` function with signature `stake(uint256)` and selector `0xa694fc3a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "stake", abi = "stake(uint256)")]
    pub struct StakeCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeAll` function with signature `stakeAll()` and selector `0x8dcb4061`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakeAll", abi = "stakeAll()")]
    pub struct StakeAllCall;
    ///Container type for all input parameters for the `stakeFor` function with signature `stakeFor(address,uint256)` and selector `0x2ee40908`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakeFor", abi = "stakeFor(address,uint256)")]
    pub struct StakeForCall {
        pub for_: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakingToken` function with signature `stakingToken()` and selector `0x72f702f3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakingToken", abi = "stakingToken()")]
    pub struct StakingTokenCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `userRewardPerTokenPaid` function with signature `userRewardPerTokenPaid(address)` and selector `0x8b876347`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "userRewardPerTokenPaid",
        abi = "userRewardPerTokenPaid(address)"
    )]
    pub struct UserRewardPerTokenPaidCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,bool)` and selector `0x38d07436`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,bool)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
        pub claim: bool,
    }
    ///Container type for all input parameters for the `withdrawAll` function with signature `withdrawAll(bool)` and selector `0x1c1c6fe5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdrawAll", abi = "withdrawAll(bool)")]
    pub struct WithdrawAllCall {
        pub claim: bool,
    }
    ///Container type for all input parameters for the `withdrawAllAndUnwrap` function with signature `withdrawAllAndUnwrap(bool)` and selector `0x49f039a2`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdrawAllAndUnwrap", abi = "withdrawAllAndUnwrap(bool)")]
    pub struct WithdrawAllAndUnwrapCall {
        pub claim: bool,
    }
    ///Container type for all input parameters for the `withdrawAndUnwrap` function with signature `withdrawAndUnwrap(uint256,bool)` and selector `0xc32e7202`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdrawAndUnwrap", abi = "withdrawAndUnwrap(uint256,bool)")]
    pub struct WithdrawAndUnwrapCall {
        pub amount: ::ethers::core::types::U256,
        pub claim: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IConvexBaseRewardPoolCalls {
        AddExtraReward(AddExtraRewardCall),
        BalanceOf(BalanceOfCall),
        ClearExtraRewards(ClearExtraRewardsCall),
        CurrentRewards(CurrentRewardsCall),
        Donate(DonateCall),
        Duration(DurationCall),
        Earned(EarnedCall),
        ExtraRewards(ExtraRewardsCall),
        ExtraRewardsLength(ExtraRewardsLengthCall),
        GetReward(GetRewardCall),
        GetRewardWithAccountAndClaimExtras(GetRewardWithAccountAndClaimExtrasCall),
        HistoricalRewards(HistoricalRewardsCall),
        LastTimeRewardApplicable(LastTimeRewardApplicableCall),
        LastUpdateTime(LastUpdateTimeCall),
        NewRewardRatio(NewRewardRatioCall),
        Operator(OperatorCall),
        PeriodFinish(PeriodFinishCall),
        Pid(PidCall),
        QueueNewRewards(QueueNewRewardsCall),
        QueuedRewards(QueuedRewardsCall),
        RewardManager(RewardManagerCall),
        RewardPerToken(RewardPerTokenCall),
        RewardPerTokenStored(RewardPerTokenStoredCall),
        RewardRate(RewardRateCall),
        RewardToken(RewardTokenCall),
        Rewards(RewardsCall),
        Stake(StakeCall),
        StakeAll(StakeAllCall),
        StakeFor(StakeForCall),
        StakingToken(StakingTokenCall),
        TotalSupply(TotalSupplyCall),
        UserRewardPerTokenPaid(UserRewardPerTokenPaidCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
        WithdrawAllAndUnwrap(WithdrawAllAndUnwrapCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
    }
    impl ::ethers::core::abi::AbiDecode for IConvexBaseRewardPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddExtraRewardCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::AddExtraReward(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ClearExtraRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::ClearExtraRewards(decoded));
            }
            if let Ok(decoded) =
                <CurrentRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::CurrentRewards(decoded));
            }
            if let Ok(decoded) =
                <DonateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Donate(decoded));
            }
            if let Ok(decoded) =
                <DurationCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Duration(decoded));
            }
            if let Ok(decoded) =
                <EarnedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Earned(decoded));
            }
            if let Ok(decoded) =
                <ExtraRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::ExtraRewards(decoded));
            }
            if let Ok(decoded) =
                <ExtraRewardsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::ExtraRewardsLength(decoded));
            }
            if let Ok(decoded) =
                <GetRewardCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::GetReward(decoded));
            }
            if let Ok(decoded) =
                <GetRewardWithAccountAndClaimExtrasCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IConvexBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(decoded));
            }
            if let Ok(decoded) =
                <HistoricalRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::HistoricalRewards(decoded));
            }
            if let Ok(decoded) =
                <LastTimeRewardApplicableCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IConvexBaseRewardPoolCalls::LastTimeRewardApplicable(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastUpdateTimeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::LastUpdateTime(decoded));
            }
            if let Ok(decoded) =
                <NewRewardRatioCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::NewRewardRatio(decoded));
            }
            if let Ok(decoded) =
                <OperatorCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Operator(decoded));
            }
            if let Ok(decoded) =
                <PeriodFinishCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::PeriodFinish(decoded));
            }
            if let Ok(decoded) = <PidCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Pid(decoded));
            }
            if let Ok(decoded) =
                <QueueNewRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::QueueNewRewards(decoded));
            }
            if let Ok(decoded) =
                <QueuedRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::QueuedRewards(decoded));
            }
            if let Ok(decoded) =
                <RewardManagerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::RewardManager(decoded));
            }
            if let Ok(decoded) =
                <RewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::RewardPerToken(decoded));
            }
            if let Ok(decoded) =
                <RewardPerTokenStoredCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::RewardPerTokenStored(decoded));
            }
            if let Ok(decoded) =
                <RewardRateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::RewardRate(decoded));
            }
            if let Ok(decoded) =
                <RewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::RewardToken(decoded));
            }
            if let Ok(decoded) =
                <RewardsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Rewards(decoded));
            }
            if let Ok(decoded) =
                <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Stake(decoded));
            }
            if let Ok(decoded) =
                <StakeAllCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::StakeAll(decoded));
            }
            if let Ok(decoded) =
                <StakeForCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::StakeFor(decoded));
            }
            if let Ok(decoded) =
                <StakingTokenCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::StakingToken(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <UserRewardPerTokenPaidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IConvexBaseRewardPoolCalls::UserRewardPerTokenPaid(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::WithdrawAll(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllAndUnwrapCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::WithdrawAllAndUnwrap(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexBaseRewardPoolCalls::WithdrawAndUnwrap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IConvexBaseRewardPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IConvexBaseRewardPoolCalls::AddExtraReward(element) => element.encode(),
                IConvexBaseRewardPoolCalls::BalanceOf(element) => element.encode(),
                IConvexBaseRewardPoolCalls::ClearExtraRewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::CurrentRewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Donate(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Duration(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Earned(element) => element.encode(),
                IConvexBaseRewardPoolCalls::ExtraRewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::ExtraRewardsLength(element) => element.encode(),
                IConvexBaseRewardPoolCalls::GetReward(element) => element.encode(),
                IConvexBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(element) => {
                    element.encode()
                }
                IConvexBaseRewardPoolCalls::HistoricalRewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::LastTimeRewardApplicable(element) => element.encode(),
                IConvexBaseRewardPoolCalls::LastUpdateTime(element) => element.encode(),
                IConvexBaseRewardPoolCalls::NewRewardRatio(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Operator(element) => element.encode(),
                IConvexBaseRewardPoolCalls::PeriodFinish(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Pid(element) => element.encode(),
                IConvexBaseRewardPoolCalls::QueueNewRewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::QueuedRewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::RewardManager(element) => element.encode(),
                IConvexBaseRewardPoolCalls::RewardPerToken(element) => element.encode(),
                IConvexBaseRewardPoolCalls::RewardPerTokenStored(element) => element.encode(),
                IConvexBaseRewardPoolCalls::RewardRate(element) => element.encode(),
                IConvexBaseRewardPoolCalls::RewardToken(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Rewards(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Stake(element) => element.encode(),
                IConvexBaseRewardPoolCalls::StakeAll(element) => element.encode(),
                IConvexBaseRewardPoolCalls::StakeFor(element) => element.encode(),
                IConvexBaseRewardPoolCalls::StakingToken(element) => element.encode(),
                IConvexBaseRewardPoolCalls::TotalSupply(element) => element.encode(),
                IConvexBaseRewardPoolCalls::UserRewardPerTokenPaid(element) => element.encode(),
                IConvexBaseRewardPoolCalls::Withdraw(element) => element.encode(),
                IConvexBaseRewardPoolCalls::WithdrawAll(element) => element.encode(),
                IConvexBaseRewardPoolCalls::WithdrawAllAndUnwrap(element) => element.encode(),
                IConvexBaseRewardPoolCalls::WithdrawAndUnwrap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IConvexBaseRewardPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IConvexBaseRewardPoolCalls::AddExtraReward(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::BalanceOf(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::ClearExtraRewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::CurrentRewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Donate(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Duration(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Earned(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::ExtraRewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::ExtraRewardsLength(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::GetReward(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(element) => {
                    element.fmt(f)
                }
                IConvexBaseRewardPoolCalls::HistoricalRewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::LastTimeRewardApplicable(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::LastUpdateTime(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::NewRewardRatio(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Operator(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::PeriodFinish(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Pid(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::QueueNewRewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::QueuedRewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::RewardManager(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::RewardPerToken(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::RewardPerTokenStored(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::RewardRate(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::RewardToken(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Rewards(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Stake(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::StakeAll(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::StakeFor(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::StakingToken(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::TotalSupply(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::UserRewardPerTokenPaid(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::Withdraw(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::WithdrawAll(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::WithdrawAllAndUnwrap(element) => element.fmt(f),
                IConvexBaseRewardPoolCalls::WithdrawAndUnwrap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddExtraRewardCall> for IConvexBaseRewardPoolCalls {
        fn from(var: AddExtraRewardCall) -> Self {
            IConvexBaseRewardPoolCalls::AddExtraReward(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IConvexBaseRewardPoolCalls {
        fn from(var: BalanceOfCall) -> Self {
            IConvexBaseRewardPoolCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ClearExtraRewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: ClearExtraRewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::ClearExtraRewards(var)
        }
    }
    impl ::std::convert::From<CurrentRewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: CurrentRewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::CurrentRewards(var)
        }
    }
    impl ::std::convert::From<DonateCall> for IConvexBaseRewardPoolCalls {
        fn from(var: DonateCall) -> Self {
            IConvexBaseRewardPoolCalls::Donate(var)
        }
    }
    impl ::std::convert::From<DurationCall> for IConvexBaseRewardPoolCalls {
        fn from(var: DurationCall) -> Self {
            IConvexBaseRewardPoolCalls::Duration(var)
        }
    }
    impl ::std::convert::From<EarnedCall> for IConvexBaseRewardPoolCalls {
        fn from(var: EarnedCall) -> Self {
            IConvexBaseRewardPoolCalls::Earned(var)
        }
    }
    impl ::std::convert::From<ExtraRewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: ExtraRewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::ExtraRewards(var)
        }
    }
    impl ::std::convert::From<ExtraRewardsLengthCall> for IConvexBaseRewardPoolCalls {
        fn from(var: ExtraRewardsLengthCall) -> Self {
            IConvexBaseRewardPoolCalls::ExtraRewardsLength(var)
        }
    }
    impl ::std::convert::From<GetRewardCall> for IConvexBaseRewardPoolCalls {
        fn from(var: GetRewardCall) -> Self {
            IConvexBaseRewardPoolCalls::GetReward(var)
        }
    }
    impl ::std::convert::From<GetRewardWithAccountAndClaimExtrasCall> for IConvexBaseRewardPoolCalls {
        fn from(var: GetRewardWithAccountAndClaimExtrasCall) -> Self {
            IConvexBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(var)
        }
    }
    impl ::std::convert::From<HistoricalRewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: HistoricalRewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::HistoricalRewards(var)
        }
    }
    impl ::std::convert::From<LastTimeRewardApplicableCall> for IConvexBaseRewardPoolCalls {
        fn from(var: LastTimeRewardApplicableCall) -> Self {
            IConvexBaseRewardPoolCalls::LastTimeRewardApplicable(var)
        }
    }
    impl ::std::convert::From<LastUpdateTimeCall> for IConvexBaseRewardPoolCalls {
        fn from(var: LastUpdateTimeCall) -> Self {
            IConvexBaseRewardPoolCalls::LastUpdateTime(var)
        }
    }
    impl ::std::convert::From<NewRewardRatioCall> for IConvexBaseRewardPoolCalls {
        fn from(var: NewRewardRatioCall) -> Self {
            IConvexBaseRewardPoolCalls::NewRewardRatio(var)
        }
    }
    impl ::std::convert::From<OperatorCall> for IConvexBaseRewardPoolCalls {
        fn from(var: OperatorCall) -> Self {
            IConvexBaseRewardPoolCalls::Operator(var)
        }
    }
    impl ::std::convert::From<PeriodFinishCall> for IConvexBaseRewardPoolCalls {
        fn from(var: PeriodFinishCall) -> Self {
            IConvexBaseRewardPoolCalls::PeriodFinish(var)
        }
    }
    impl ::std::convert::From<PidCall> for IConvexBaseRewardPoolCalls {
        fn from(var: PidCall) -> Self {
            IConvexBaseRewardPoolCalls::Pid(var)
        }
    }
    impl ::std::convert::From<QueueNewRewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: QueueNewRewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::QueueNewRewards(var)
        }
    }
    impl ::std::convert::From<QueuedRewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: QueuedRewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::QueuedRewards(var)
        }
    }
    impl ::std::convert::From<RewardManagerCall> for IConvexBaseRewardPoolCalls {
        fn from(var: RewardManagerCall) -> Self {
            IConvexBaseRewardPoolCalls::RewardManager(var)
        }
    }
    impl ::std::convert::From<RewardPerTokenCall> for IConvexBaseRewardPoolCalls {
        fn from(var: RewardPerTokenCall) -> Self {
            IConvexBaseRewardPoolCalls::RewardPerToken(var)
        }
    }
    impl ::std::convert::From<RewardPerTokenStoredCall> for IConvexBaseRewardPoolCalls {
        fn from(var: RewardPerTokenStoredCall) -> Self {
            IConvexBaseRewardPoolCalls::RewardPerTokenStored(var)
        }
    }
    impl ::std::convert::From<RewardRateCall> for IConvexBaseRewardPoolCalls {
        fn from(var: RewardRateCall) -> Self {
            IConvexBaseRewardPoolCalls::RewardRate(var)
        }
    }
    impl ::std::convert::From<RewardTokenCall> for IConvexBaseRewardPoolCalls {
        fn from(var: RewardTokenCall) -> Self {
            IConvexBaseRewardPoolCalls::RewardToken(var)
        }
    }
    impl ::std::convert::From<RewardsCall> for IConvexBaseRewardPoolCalls {
        fn from(var: RewardsCall) -> Self {
            IConvexBaseRewardPoolCalls::Rewards(var)
        }
    }
    impl ::std::convert::From<StakeCall> for IConvexBaseRewardPoolCalls {
        fn from(var: StakeCall) -> Self {
            IConvexBaseRewardPoolCalls::Stake(var)
        }
    }
    impl ::std::convert::From<StakeAllCall> for IConvexBaseRewardPoolCalls {
        fn from(var: StakeAllCall) -> Self {
            IConvexBaseRewardPoolCalls::StakeAll(var)
        }
    }
    impl ::std::convert::From<StakeForCall> for IConvexBaseRewardPoolCalls {
        fn from(var: StakeForCall) -> Self {
            IConvexBaseRewardPoolCalls::StakeFor(var)
        }
    }
    impl ::std::convert::From<StakingTokenCall> for IConvexBaseRewardPoolCalls {
        fn from(var: StakingTokenCall) -> Self {
            IConvexBaseRewardPoolCalls::StakingToken(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IConvexBaseRewardPoolCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IConvexBaseRewardPoolCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<UserRewardPerTokenPaidCall> for IConvexBaseRewardPoolCalls {
        fn from(var: UserRewardPerTokenPaidCall) -> Self {
            IConvexBaseRewardPoolCalls::UserRewardPerTokenPaid(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IConvexBaseRewardPoolCalls {
        fn from(var: WithdrawCall) -> Self {
            IConvexBaseRewardPoolCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAllCall> for IConvexBaseRewardPoolCalls {
        fn from(var: WithdrawAllCall) -> Self {
            IConvexBaseRewardPoolCalls::WithdrawAll(var)
        }
    }
    impl ::std::convert::From<WithdrawAllAndUnwrapCall> for IConvexBaseRewardPoolCalls {
        fn from(var: WithdrawAllAndUnwrapCall) -> Self {
            IConvexBaseRewardPoolCalls::WithdrawAllAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for IConvexBaseRewardPoolCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            IConvexBaseRewardPoolCalls::WithdrawAndUnwrap(var)
        }
    }
    ///Container type for all return fields from the `addExtraReward` function with signature `addExtraReward(address)` and selector `0x5e43c47b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AddExtraRewardReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `currentRewards` function with signature `currentRewards()` and selector `0x901a7d53`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct CurrentRewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `donate` function with signature `donate(uint256)` and selector `0xf14faf6f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DonateReturn(pub bool);
    ///Container type for all return fields from the `duration` function with signature `duration()` and selector `0x0fb5a6b4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `earned` function with signature `earned(address)` and selector `0x008cc262`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct EarnedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `extraRewards` function with signature `extraRewards(uint256)` and selector `0x40c35446`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct ExtraRewardsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `extraRewardsLength` function with signature `extraRewardsLength()` and selector `0xd55a23f4`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct ExtraRewardsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReward` function with signature `getReward()` and selector `0x3d18b912`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetRewardReturn(pub bool);
    ///Container type for all return fields from the `getReward` function with signature `getReward(address,bool)` and selector `0x7050ccd9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetRewardWithAccountAndClaimExtrasReturn(pub bool);
    ///Container type for all return fields from the `historicalRewards` function with signature `historicalRewards()` and selector `0x262d3d6d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct HistoricalRewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable()` and selector `0x80faa57d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct LastTimeRewardApplicableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastUpdateTime` function with signature `lastUpdateTime()` and selector `0xc8f33c91`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct LastUpdateTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `newRewardRatio` function with signature `newRewardRatio()` and selector `0x6c8bcee8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NewRewardRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operator` function with signature `operator()` and selector `0x570ca735`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct OperatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `periodFinish` function with signature `periodFinish()` and selector `0xebe2b12b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PeriodFinishReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pid` function with signature `pid()` and selector `0xf1068454`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PidReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queueNewRewards` function with signature `queueNewRewards(uint256)` and selector `0x590a41f5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct QueueNewRewardsReturn(pub bool);
    ///Container type for all return fields from the `queuedRewards` function with signature `queuedRewards()` and selector `0x63d38c3b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct QueuedRewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardManager` function with signature `rewardManager()` and selector `0x0f4ef8a6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RewardManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewardPerToken` function with signature `rewardPerToken()` and selector `0xcd3daf9d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RewardPerTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerTokenStored` function with signature `rewardPerTokenStored()` and selector `0xdf136d65`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RewardPerTokenStoredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardRate` function with signature `rewardRate()` and selector `0x7b0a47ee`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RewardRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RewardTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewards` function with signature `rewards(address)` and selector `0x0700037d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stake` function with signature `stake(uint256)` and selector `0xa694fc3a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct StakeReturn(pub bool);
    ///Container type for all return fields from the `stakeAll` function with signature `stakeAll()` and selector `0x8dcb4061`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct StakeAllReturn(pub bool);
    ///Container type for all return fields from the `stakeFor` function with signature `stakeFor(address,uint256)` and selector `0x2ee40908`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct StakeForReturn(pub bool);
    ///Container type for all return fields from the `stakingToken` function with signature `stakingToken()` and selector `0x72f702f3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct StakingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userRewardPerTokenPaid` function with signature `userRewardPerTokenPaid(address)` and selector `0x8b876347`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct UserRewardPerTokenPaidReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(uint256,bool)` and selector `0x38d07436`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct WithdrawReturn(pub bool);
    ///Container type for all return fields from the `withdrawAndUnwrap` function with signature `withdrawAndUnwrap(uint256,bool)` and selector `0xc32e7202`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct WithdrawAndUnwrapReturn(pub bool);
}
