pub use i_orion_pool_v2_router_02_ext::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_orion_pool_v2_router_02_ext {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IOrionPoolV2Router02Ext was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapExactTokensForTokensAutoRoute\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapTokensForExactTokensAutoRoute\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IORIONPOOLV2ROUTER02EXT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IOrionPoolV2Router02Ext<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IOrionPoolV2Router02Ext<M> {
        fn clone(&self) -> Self {
            IOrionPoolV2Router02Ext(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOrionPoolV2Router02Ext<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IOrionPoolV2Router02Ext<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOrionPoolV2Router02Ext))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOrionPoolV2Router02Ext<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IORIONPOOLV2ROUTER02EXT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `swapExactTokensForTokensAutoRoute` (0x5da89813) function
        pub fn swap_exact_tokens_for_tokens_auto_route(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([93, 168, 152, 19], (amount_in, amount_out_min, path, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactTokensAutoRoute` (0xbba22d0e) function
        pub fn swap_tokens_for_exact_tokens_auto_route(
            &self,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([187, 162, 45, 14], (amount_out, amount_in_max, path, to))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IOrionPoolV2Router02Ext<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `swapExactTokensForTokensAutoRoute` function with signature `swapExactTokensForTokensAutoRoute(uint256,uint256,address[],address)` and selector `0x5da89813`
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
        name = "swapExactTokensForTokensAutoRoute",
        abi = "swapExactTokensForTokensAutoRoute(uint256,uint256,address[],address)"
    )]
    pub struct SwapExactTokensForTokensAutoRouteCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapTokensForExactTokensAutoRoute` function with signature `swapTokensForExactTokensAutoRoute(uint256,uint256,address[],address)` and selector `0xbba22d0e`
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
        name = "swapTokensForExactTokensAutoRoute",
        abi = "swapTokensForExactTokensAutoRoute(uint256,uint256,address[],address)"
    )]
    pub struct SwapTokensForExactTokensAutoRouteCall {
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IOrionPoolV2Router02ExtCalls {
        SwapExactTokensForTokensAutoRoute(SwapExactTokensForTokensAutoRouteCall),
        SwapTokensForExactTokensAutoRoute(SwapTokensForExactTokensAutoRouteCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOrionPoolV2Router02ExtCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SwapExactTokensForTokensAutoRouteCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IOrionPoolV2Router02ExtCalls::SwapExactTokensForTokensAutoRoute(decoded),
                );
            }
            if let Ok(decoded) =
                <SwapTokensForExactTokensAutoRouteCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IOrionPoolV2Router02ExtCalls::SwapTokensForExactTokensAutoRoute(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOrionPoolV2Router02ExtCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOrionPoolV2Router02ExtCalls::SwapExactTokensForTokensAutoRoute(element) => {
                    element.encode()
                }
                IOrionPoolV2Router02ExtCalls::SwapTokensForExactTokensAutoRoute(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IOrionPoolV2Router02ExtCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOrionPoolV2Router02ExtCalls::SwapExactTokensForTokensAutoRoute(element) => {
                    element.fmt(f)
                }
                IOrionPoolV2Router02ExtCalls::SwapTokensForExactTokensAutoRoute(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensAutoRouteCall> for IOrionPoolV2Router02ExtCalls {
        fn from(var: SwapExactTokensForTokensAutoRouteCall) -> Self {
            IOrionPoolV2Router02ExtCalls::SwapExactTokensForTokensAutoRoute(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactTokensAutoRouteCall> for IOrionPoolV2Router02ExtCalls {
        fn from(var: SwapTokensForExactTokensAutoRouteCall) -> Self {
            IOrionPoolV2Router02ExtCalls::SwapTokensForExactTokensAutoRoute(var)
        }
    }
    ///Container type for all return fields from the `swapExactTokensForTokensAutoRoute` function with signature `swapExactTokensForTokensAutoRoute(uint256,uint256,address[],address)` and selector `0x5da89813`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SwapExactTokensForTokensAutoRouteReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapTokensForExactTokensAutoRoute` function with signature `swapTokensForExactTokensAutoRoute(uint256,uint256,address[],address)` and selector `0xbba22d0e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SwapTokensForExactTokensAutoRouteReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
}
