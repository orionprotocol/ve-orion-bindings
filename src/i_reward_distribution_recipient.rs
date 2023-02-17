pub use i_reward_distribution_recipient::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_reward_distribution_recipient {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IRewardDistributionRecipient was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOwner\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"notifyRewardAmount\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_rewardDistribution\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRewardDistribution\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IREWARDDISTRIBUTIONRECIPIENT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct IRewardDistributionRecipient<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IRewardDistributionRecipient<M> {
        fn clone(&self) -> Self {
            IRewardDistributionRecipient(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IRewardDistributionRecipient<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IRewardDistributionRecipient<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRewardDistributionRecipient))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IRewardDistributionRecipient<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IREWARDDISTRIBUTIONRECIPIENT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `isOwner` (0x8f32d59b) function
        pub fn is_owner(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([143, 50, 213, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardAmount` (0x3c6b16ab) function
        pub fn notify_reward_amount(
            &self,
            reward: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 107, 22, 171], reward)
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRewardDistribution` (0x0d68b761) function
        pub fn set_reward_distribution(
            &self,
            reward_distribution: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 104, 183, 97], reward_distribution)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IRewardDistributionRecipient<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner()` and selector `0x8f32d59b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOwner", abi = "isOwner()")]
    pub struct IsOwnerCall;
    ///Container type for all input parameters for the `notifyRewardAmount` function with signature `notifyRewardAmount(uint256)` and selector `0x3c6b16ab`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(uint256)")]
    pub struct NotifyRewardAmountCall {
        pub reward: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setRewardDistribution` function with signature `setRewardDistribution(address)` and selector `0x0d68b761`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRewardDistribution", abi = "setRewardDistribution(address)")]
    pub struct SetRewardDistributionCall {
        pub reward_distribution: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IRewardDistributionRecipientCalls {
        IsOwner(IsOwnerCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetRewardDistribution(SetRewardDistributionCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for IRewardDistributionRecipientCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardDistributionRecipientCalls::IsOwner(decoded));
            }
            if let Ok(decoded) =
                <NotifyRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardDistributionRecipientCalls::NotifyRewardAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardDistributionRecipientCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardDistributionRecipientCalls::RenounceOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetRewardDistributionCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardDistributionRecipientCalls::SetRewardDistribution(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardDistributionRecipientCalls::TransferOwnership(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IRewardDistributionRecipientCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IRewardDistributionRecipientCalls::IsOwner(element) => element.encode(),
                IRewardDistributionRecipientCalls::NotifyRewardAmount(element) => element.encode(),
                IRewardDistributionRecipientCalls::Owner(element) => element.encode(),
                IRewardDistributionRecipientCalls::RenounceOwnership(element) => element.encode(),
                IRewardDistributionRecipientCalls::SetRewardDistribution(element) => {
                    element.encode()
                }
                IRewardDistributionRecipientCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRewardDistributionRecipientCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRewardDistributionRecipientCalls::IsOwner(element) => element.fmt(f),
                IRewardDistributionRecipientCalls::NotifyRewardAmount(element) => element.fmt(f),
                IRewardDistributionRecipientCalls::Owner(element) => element.fmt(f),
                IRewardDistributionRecipientCalls::RenounceOwnership(element) => element.fmt(f),
                IRewardDistributionRecipientCalls::SetRewardDistribution(element) => element.fmt(f),
                IRewardDistributionRecipientCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsOwnerCall> for IRewardDistributionRecipientCalls {
        fn from(var: IsOwnerCall) -> Self {
            IRewardDistributionRecipientCalls::IsOwner(var)
        }
    }
    impl ::std::convert::From<NotifyRewardAmountCall> for IRewardDistributionRecipientCalls {
        fn from(var: NotifyRewardAmountCall) -> Self {
            IRewardDistributionRecipientCalls::NotifyRewardAmount(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IRewardDistributionRecipientCalls {
        fn from(var: OwnerCall) -> Self {
            IRewardDistributionRecipientCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for IRewardDistributionRecipientCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            IRewardDistributionRecipientCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetRewardDistributionCall> for IRewardDistributionRecipientCalls {
        fn from(var: SetRewardDistributionCall) -> Self {
            IRewardDistributionRecipientCalls::SetRewardDistribution(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for IRewardDistributionRecipientCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            IRewardDistributionRecipientCalls::TransferOwnership(var)
        }
    }
    ///Container type for all return fields from the `isOwner` function with signature `isOwner()` and selector `0x8f32d59b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct IsOwnerReturn(pub bool);
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
}
