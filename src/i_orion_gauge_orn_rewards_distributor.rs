pub use i_orion_gauge_orn_rewards_distributor::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_orion_gauge_orn_rewards_distributor {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IOrionGaugeORNRewardsDistributor was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"curator_address\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"gauge_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentReward\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"reward_amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"gauge_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"distributeReward\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"weeks_elapsed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reward_tally\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"distributionsOn\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge_whitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"is_middleman\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"last_time_gauge_paid\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"nominateNewOwner\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nominatedOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"recoverERC20\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_new_curator_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCurator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_gauge_controller_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGaugeController\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_gauge_address\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_is_middleman\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_is_active\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGaugeState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_new_timelock\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTimelock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"timelock_address\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"toggleDistributions\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IORIONGAUGEORNREWARDSDISTRIBUTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct IOrionGaugeORNRewardsDistributor<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IOrionGaugeORNRewardsDistributor<M> {
        fn clone(&self) -> Self {
            IOrionGaugeORNRewardsDistributor(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOrionGaugeORNRewardsDistributor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IOrionGaugeORNRewardsDistributor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOrionGaugeORNRewardsDistributor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOrionGaugeORNRewardsDistributor<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IORIONGAUGEORNREWARDSDISTRIBUTOR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `curator_address` (0xc92073c1) function
        pub fn curator_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([201, 32, 115, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentReward` (0x9d18e4b0) function
        pub fn current_reward(
            &self,
            gauge_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 24, 228, 176], gauge_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributeReward` (0x092193ab) function
        pub fn distribute_reward(
            &self,
            gauge_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([9, 33, 147, 171], gauge_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributionsOn` (0x1f8a7edf) function
        pub fn distributions_on(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 138, 126, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_whitelist` (0x2fd37b08) function
        pub fn gauge_whitelist(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 211, 123, 8], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `is_middleman` (0x6ca81c1c) function
        pub fn is_middleman(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 168, 28, 28], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `last_time_gauge_paid` (0xe81e17c6) function
        pub fn last_time_gauge_paid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 30, 23, 198], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nominateNewOwner` (0x1627540c) function
        pub fn nominate_new_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 39, 84, 12], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nominatedOwner` (0x53a47bb7) function
        pub fn nominated_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([83, 164, 123, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recoverERC20` (0x8980f11f) function
        pub fn recover_erc20(
            &self,
            token_address: ::ethers::core::types::Address,
            token_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 128, 241, 31], (token_address, token_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCurator` (0xe90956cf) function
        pub fn set_curator(
            &self,
            new_curator_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 9, 86, 207], new_curator_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGaugeController` (0x0091d2b8) function
        pub fn set_gauge_controller(
            &self,
            gauge_controller_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 145, 210, 184], gauge_controller_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGaugeState` (0x570b1e99) function
        pub fn set_gauge_state(
            &self,
            gauge_address: ::ethers::core::types::Address,
            is_middleman: bool,
            is_active: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 11, 30, 153], (gauge_address, is_middleman, is_active))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTimelock` (0xbdacb303) function
        pub fn set_timelock(
            &self,
            new_timelock: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 172, 179, 3], new_timelock)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timelock_address` (0xdc6663c7) function
        pub fn timelock_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([220, 102, 99, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toggleDistributions` (0x305d6d5f) function
        pub fn toggle_distributions(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 93, 109, 95], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IOrionGaugeORNRewardsDistributor<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `curator_address` function with signature `curator_address()` and selector `0xc92073c1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "curator_address", abi = "curator_address()")]
    pub struct CuratorAddressCall;
    ///Container type for all input parameters for the `currentReward` function with signature `currentReward(address)` and selector `0x9d18e4b0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "currentReward", abi = "currentReward(address)")]
    pub struct CurrentRewardCall {
        pub gauge_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `distributeReward` function with signature `distributeReward(address)` and selector `0x092193ab`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "distributeReward", abi = "distributeReward(address)")]
    pub struct DistributeRewardCall {
        pub gauge_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `distributionsOn` function with signature `distributionsOn()` and selector `0x1f8a7edf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "distributionsOn", abi = "distributionsOn()")]
    pub struct DistributionsOnCall;
    ///Container type for all input parameters for the `gauge_whitelist` function with signature `gauge_whitelist(address)` and selector `0x2fd37b08`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "gauge_whitelist", abi = "gauge_whitelist(address)")]
    pub struct GaugeWhitelistCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `is_middleman` function with signature `is_middleman(address)` and selector `0x6ca81c1c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "is_middleman", abi = "is_middleman(address)")]
    pub struct IsMiddlemanCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `last_time_gauge_paid` function with signature `last_time_gauge_paid(address)` and selector `0xe81e17c6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "last_time_gauge_paid", abi = "last_time_gauge_paid(address)")]
    pub struct LastTimeGaugePaidCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `nominateNewOwner` function with signature `nominateNewOwner(address)` and selector `0x1627540c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "nominateNewOwner", abi = "nominateNewOwner(address)")]
    pub struct NominateNewOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nominatedOwner` function with signature `nominatedOwner()` and selector `0x53a47bb7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "nominatedOwner", abi = "nominatedOwner()")]
    pub struct NominatedOwnerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `recoverERC20` function with signature `recoverERC20(address,uint256)` and selector `0x8980f11f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "recoverERC20", abi = "recoverERC20(address,uint256)")]
    pub struct RecoverERC20Call {
        pub token_address: ::ethers::core::types::Address,
        pub token_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setCurator` function with signature `setCurator(address)` and selector `0xe90956cf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setCurator", abi = "setCurator(address)")]
    pub struct SetCuratorCall {
        pub new_curator_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGaugeController` function with signature `setGaugeController(address)` and selector `0x0091d2b8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setGaugeController", abi = "setGaugeController(address)")]
    pub struct SetGaugeControllerCall {
        pub gauge_controller_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGaugeState` function with signature `setGaugeState(address,bool,bool)` and selector `0x570b1e99`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setGaugeState", abi = "setGaugeState(address,bool,bool)")]
    pub struct SetGaugeStateCall {
        pub gauge_address: ::ethers::core::types::Address,
        pub is_middleman: bool,
        pub is_active: bool,
    }
    ///Container type for all input parameters for the `setTimelock` function with signature `setTimelock(address)` and selector `0xbdacb303`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTimelock", abi = "setTimelock(address)")]
    pub struct SetTimelockCall {
        pub new_timelock: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `timelock_address` function with signature `timelock_address()` and selector `0xdc6663c7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "timelock_address", abi = "timelock_address()")]
    pub struct TimelockAddressCall;
    ///Container type for all input parameters for the `toggleDistributions` function with signature `toggleDistributions()` and selector `0x305d6d5f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "toggleDistributions", abi = "toggleDistributions()")]
    pub struct ToggleDistributionsCall;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IOrionGaugeORNRewardsDistributorCalls {
        AcceptOwnership(AcceptOwnershipCall),
        CuratorAddress(CuratorAddressCall),
        CurrentReward(CurrentRewardCall),
        DistributeReward(DistributeRewardCall),
        DistributionsOn(DistributionsOnCall),
        GaugeWhitelist(GaugeWhitelistCall),
        IsMiddleman(IsMiddlemanCall),
        LastTimeGaugePaid(LastTimeGaugePaidCall),
        NominateNewOwner(NominateNewOwnerCall),
        NominatedOwner(NominatedOwnerCall),
        Owner(OwnerCall),
        RecoverERC20(RecoverERC20Call),
        SetCurator(SetCuratorCall),
        SetGaugeController(SetGaugeControllerCall),
        SetGaugeState(SetGaugeStateCall),
        SetTimelock(SetTimelockCall),
        TimelockAddress(TimelockAddressCall),
        ToggleDistributions(ToggleDistributionsCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOrionGaugeORNRewardsDistributorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::AcceptOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CuratorAddressCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::CuratorAddress(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CurrentRewardCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::CurrentReward(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DistributeRewardCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::DistributeReward(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DistributionsOnCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::DistributionsOn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GaugeWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::GaugeWhitelist(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IsMiddlemanCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::IsMiddleman(decoded));
            }
            if let Ok(decoded) =
                <LastTimeGaugePaidCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::LastTimeGaugePaid(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NominateNewOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::NominateNewOwner(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NominatedOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::NominatedOwner(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RecoverERC20Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::RecoverERC20(decoded));
            }
            if let Ok(decoded) =
                <SetCuratorCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::SetCurator(decoded));
            }
            if let Ok(decoded) =
                <SetGaugeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::SetGaugeController(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetGaugeStateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::SetGaugeState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetTimelockCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::SetTimelock(decoded));
            }
            if let Ok(decoded) =
                <TimelockAddressCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::TimelockAddress(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ToggleDistributionsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeORNRewardsDistributorCalls::ToggleDistributions(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOrionGaugeORNRewardsDistributorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOrionGaugeORNRewardsDistributorCalls::AcceptOwnership(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::CuratorAddress(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::CurrentReward(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::DistributeReward(element) => {
                    element.encode()
                }
                IOrionGaugeORNRewardsDistributorCalls::DistributionsOn(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::GaugeWhitelist(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::IsMiddleman(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::LastTimeGaugePaid(element) => {
                    element.encode()
                }
                IOrionGaugeORNRewardsDistributorCalls::NominateNewOwner(element) => {
                    element.encode()
                }
                IOrionGaugeORNRewardsDistributorCalls::NominatedOwner(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::Owner(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::RecoverERC20(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::SetCurator(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::SetGaugeController(element) => {
                    element.encode()
                }
                IOrionGaugeORNRewardsDistributorCalls::SetGaugeState(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::SetTimelock(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::TimelockAddress(element) => element.encode(),
                IOrionGaugeORNRewardsDistributorCalls::ToggleDistributions(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IOrionGaugeORNRewardsDistributorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOrionGaugeORNRewardsDistributorCalls::AcceptOwnership(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::CuratorAddress(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::CurrentReward(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::DistributeReward(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::DistributionsOn(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::GaugeWhitelist(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::IsMiddleman(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::LastTimeGaugePaid(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::NominateNewOwner(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::NominatedOwner(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::Owner(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::RecoverERC20(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::SetCurator(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::SetGaugeController(element) => {
                    element.fmt(f)
                }
                IOrionGaugeORNRewardsDistributorCalls::SetGaugeState(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::SetTimelock(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::TimelockAddress(element) => element.fmt(f),
                IOrionGaugeORNRewardsDistributorCalls::ToggleDistributions(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::AcceptOwnership(var)
        }
    }
    impl ::std::convert::From<CuratorAddressCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: CuratorAddressCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::CuratorAddress(var)
        }
    }
    impl ::std::convert::From<CurrentRewardCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: CurrentRewardCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::CurrentReward(var)
        }
    }
    impl ::std::convert::From<DistributeRewardCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: DistributeRewardCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::DistributeReward(var)
        }
    }
    impl ::std::convert::From<DistributionsOnCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: DistributionsOnCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::DistributionsOn(var)
        }
    }
    impl ::std::convert::From<GaugeWhitelistCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: GaugeWhitelistCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::GaugeWhitelist(var)
        }
    }
    impl ::std::convert::From<IsMiddlemanCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: IsMiddlemanCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::IsMiddleman(var)
        }
    }
    impl ::std::convert::From<LastTimeGaugePaidCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: LastTimeGaugePaidCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::LastTimeGaugePaid(var)
        }
    }
    impl ::std::convert::From<NominateNewOwnerCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: NominateNewOwnerCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::NominateNewOwner(var)
        }
    }
    impl ::std::convert::From<NominatedOwnerCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: NominatedOwnerCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::NominatedOwner(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: OwnerCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RecoverERC20Call> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: RecoverERC20Call) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::RecoverERC20(var)
        }
    }
    impl ::std::convert::From<SetCuratorCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: SetCuratorCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::SetCurator(var)
        }
    }
    impl ::std::convert::From<SetGaugeControllerCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: SetGaugeControllerCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::SetGaugeController(var)
        }
    }
    impl ::std::convert::From<SetGaugeStateCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: SetGaugeStateCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::SetGaugeState(var)
        }
    }
    impl ::std::convert::From<SetTimelockCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: SetTimelockCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::SetTimelock(var)
        }
    }
    impl ::std::convert::From<TimelockAddressCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: TimelockAddressCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::TimelockAddress(var)
        }
    }
    impl ::std::convert::From<ToggleDistributionsCall> for IOrionGaugeORNRewardsDistributorCalls {
        fn from(var: ToggleDistributionsCall) -> Self {
            IOrionGaugeORNRewardsDistributorCalls::ToggleDistributions(var)
        }
    }
    ///Container type for all return fields from the `curator_address` function with signature `curator_address()` and selector `0xc92073c1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct CuratorAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `currentReward` function with signature `currentReward(address)` and selector `0x9d18e4b0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct CurrentRewardReturn {
        pub reward_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `distributeReward` function with signature `distributeReward(address)` and selector `0x092193ab`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DistributeRewardReturn {
        pub weeks_elapsed: ::ethers::core::types::U256,
        pub reward_tally: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `distributionsOn` function with signature `distributionsOn()` and selector `0x1f8a7edf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DistributionsOnReturn(pub bool);
    ///Container type for all return fields from the `gauge_whitelist` function with signature `gauge_whitelist(address)` and selector `0x2fd37b08`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeWhitelistReturn(pub bool);
    ///Container type for all return fields from the `is_middleman` function with signature `is_middleman(address)` and selector `0x6ca81c1c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct IsMiddlemanReturn(pub bool);
    ///Container type for all return fields from the `last_time_gauge_paid` function with signature `last_time_gauge_paid(address)` and selector `0xe81e17c6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct LastTimeGaugePaidReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nominatedOwner` function with signature `nominatedOwner()` and selector `0x53a47bb7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NominatedOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `timelock_address` function with signature `timelock_address()` and selector `0xdc6663c7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TimelockAddressReturn(pub ::ethers::core::types::Address);
}
