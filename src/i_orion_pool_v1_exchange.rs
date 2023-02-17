pub use i_orion_pool_v1_exchange::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_orion_pool_v1_exchange {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IOrionPoolV1Exchange was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"ethToTokenSwapInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tokenToEthSwapInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IORIONPOOLV1EXCHANGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IOrionPoolV1Exchange<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IOrionPoolV1Exchange<M> {
        fn clone(&self) -> Self {
            IOrionPoolV1Exchange(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOrionPoolV1Exchange<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IOrionPoolV1Exchange<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOrionPoolV1Exchange))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOrionPoolV1Exchange<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IORIONPOOLV1EXCHANGE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ethToTokenSwapInput` (0xf39b5b9b) function
        pub fn eth_to_token_swap_input(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 155, 91, 155], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidity` (0xf88bf15a) function
        pub fn remove_liquidity(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([248, 139, 241, 90], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenToEthSwapInput` (0x95e3c50b) function
        pub fn token_to_eth_swap_input(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([149, 227, 197, 11], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IOrionPoolV1Exchange<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `ethToTokenSwapInput` function with signature `ethToTokenSwapInput(uint256,uint256)` and selector `0xf39b5b9b`
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
        name = "ethToTokenSwapInput",
        abi = "ethToTokenSwapInput(uint256,uint256)"
    )]
    pub struct EthToTokenSwapInputCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(uint256,uint256,uint256,uint256)` and selector `0xf88bf15a`
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
        name = "removeLiquidity",
        abi = "removeLiquidity(uint256,uint256,uint256,uint256)"
    )]
    pub struct RemoveLiquidityCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `tokenToEthSwapInput` function with signature `tokenToEthSwapInput(uint256,uint256,uint256)` and selector `0x95e3c50b`
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
        name = "tokenToEthSwapInput",
        abi = "tokenToEthSwapInput(uint256,uint256,uint256)"
    )]
    pub struct TokenToEthSwapInputCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IOrionPoolV1ExchangeCalls {
        BalanceOf(BalanceOfCall),
        EthToTokenSwapInput(EthToTokenSwapInputCall),
        RemoveLiquidity(RemoveLiquidityCall),
        TokenToEthSwapInput(TokenToEthSwapInputCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOrionPoolV1ExchangeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV1ExchangeCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <EthToTokenSwapInputCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV1ExchangeCalls::EthToTokenSwapInput(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV1ExchangeCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <TokenToEthSwapInputCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV1ExchangeCalls::TokenToEthSwapInput(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV1ExchangeCalls::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOrionPoolV1ExchangeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOrionPoolV1ExchangeCalls::BalanceOf(element) => element.encode(),
                IOrionPoolV1ExchangeCalls::EthToTokenSwapInput(element) => element.encode(),
                IOrionPoolV1ExchangeCalls::RemoveLiquidity(element) => element.encode(),
                IOrionPoolV1ExchangeCalls::TokenToEthSwapInput(element) => element.encode(),
                IOrionPoolV1ExchangeCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOrionPoolV1ExchangeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOrionPoolV1ExchangeCalls::BalanceOf(element) => element.fmt(f),
                IOrionPoolV1ExchangeCalls::EthToTokenSwapInput(element) => element.fmt(f),
                IOrionPoolV1ExchangeCalls::RemoveLiquidity(element) => element.fmt(f),
                IOrionPoolV1ExchangeCalls::TokenToEthSwapInput(element) => element.fmt(f),
                IOrionPoolV1ExchangeCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IOrionPoolV1ExchangeCalls {
        fn from(var: BalanceOfCall) -> Self {
            IOrionPoolV1ExchangeCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<EthToTokenSwapInputCall> for IOrionPoolV1ExchangeCalls {
        fn from(var: EthToTokenSwapInputCall) -> Self {
            IOrionPoolV1ExchangeCalls::EthToTokenSwapInput(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for IOrionPoolV1ExchangeCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            IOrionPoolV1ExchangeCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<TokenToEthSwapInputCall> for IOrionPoolV1ExchangeCalls {
        fn from(var: TokenToEthSwapInputCall) -> Self {
            IOrionPoolV1ExchangeCalls::TokenToEthSwapInput(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for IOrionPoolV1ExchangeCalls {
        fn from(var: TransferFromCall) -> Self {
            IOrionPoolV1ExchangeCalls::TransferFrom(var)
        }
    }
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
    ///Container type for all return fields from the `ethToTokenSwapInput` function with signature `ethToTokenSwapInput(uint256,uint256)` and selector `0xf39b5b9b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct EthToTokenSwapInputReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(uint256,uint256,uint256,uint256)` and selector `0xf88bf15a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `tokenToEthSwapInput` function with signature `tokenToEthSwapInput(uint256,uint256,uint256)` and selector `0x95e3c50b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TokenToEthSwapInputReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TransferFromReturn(pub bool);
}
