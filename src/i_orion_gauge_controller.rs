pub use i_orion_gauge_controller::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_orion_gauge_controller {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IOrionGaugeController was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add_gauge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add_type\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"apply_transfer_ownership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"change_gauge_weight\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"change_global_emission_rate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"change_type_weight\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"checkpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"checkpoint_gauge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"commit_transfer_ownership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"future_admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge_relative_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge_relative_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"gauge_relative_weight_write\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"gauge_relative_weight_write\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge_type_names\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge_types\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauges\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_gauge_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_total_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_type_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_weights_sum_per_type\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"global_emission_rate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"last_user_vote\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"n_gauge_types\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"n_gauges\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"points_sum\",\"outputs\":[{\"internalType\":\"struct IOrionGaugeController.Point\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"bias\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"slope\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"points_total\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"points_type_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"points_weight\",\"outputs\":[{\"internalType\":\"struct IOrionGaugeController.Point\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"bias\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"slope\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"time_sum\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"time_total\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"time_type_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"time_weight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"vote_for_gauge_weights\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vote_user_power\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vote_user_slopes\",\"outputs\":[{\"internalType\":\"struct IOrionGaugeController.VotedSlope\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"slope\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"power\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"end\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"voting_escrow\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IORIONGAUGECONTROLLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IOrionGaugeController<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IOrionGaugeController<M> {
        fn clone(&self) -> Self {
            IOrionGaugeController(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOrionGaugeController<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IOrionGaugeController<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOrionGaugeController))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOrionGaugeController<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IORIONGAUGECONTROLLER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `add_gauge` (0x18dfe921) function
        pub fn add_gauge(
            &self,
            p0: ::ethers::core::types::Address,
            p1: i128,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 223, 233, 33], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `add_type` (0x92d0d232) function
        pub fn add_type(
            &self,
            p0: String,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 208, 210, 50], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `apply_transfer_ownership` (0x6a1c05ae) function
        pub fn apply_transfer_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 28, 5, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `change_gauge_weight` (0xd4d2646e) function
        pub fn change_gauge_weight(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 210, 100, 110], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `change_global_emission_rate` (0x01c08210) function
        pub fn change_global_emission_rate(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 192, 130, 16], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `change_type_weight` (0xdb1ca260) function
        pub fn change_type_weight(
            &self,
            p0: i128,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 28, 162, 96], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoint` (0xc2c4c5c1) function
        pub fn checkpoint(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 196, 197, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoint_gauge` (0x615e5237) function
        pub fn checkpoint_gauge(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 94, 82, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commit_transfer_ownership` (0x6b441a40) function
        pub fn commit_transfer_ownership(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 68, 26, 64], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `future_admin` (0x17f7182a) function
        pub fn future_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([23, 247, 24, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_relative_weight` (0x6207d866) function
        pub fn gauge_relative_weight_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 7, 216, 102], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_relative_weight` (0xd3078c94) function
        pub fn gauge_relative_weight_1(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 7, 140, 148], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_relative_weight_write` (0x6472eee1) function
        pub fn gauge_relative_weight_write_1(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([100, 114, 238, 225], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_relative_weight_write` (0x95cfcec3) function
        pub fn gauge_relative_weight_write_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([149, 207, 206, 195], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_type_names` (0xd958a8fc) function
        pub fn gauge_type_names(
            &self,
            p0: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([217, 88, 168, 252], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauge_types` (0x3f9095b7) function
        pub fn gauge_types(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([63, 144, 149, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauges` (0xb0539187) function
        pub fn gauges(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([176, 83, 145, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_gauge_weight` (0x4e791a3a) function
        pub fn get_gauge_weight(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 121, 26, 58], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_total_weight` (0x6977ff92) function
        pub fn get_total_weight(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 119, 255, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_type_weight` (0x72fdccfa) function
        pub fn get_type_weight(
            &self,
            p0: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 253, 204, 250], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_weights_sum_per_type` (0x6f214a6a) function
        pub fn get_weights_sum_per_type(
            &self,
            p0: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 33, 74, 106], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `global_emission_rate` (0x0a3be757) function
        pub fn global_emission_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 59, 231, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `last_user_vote` (0x7e418fa0) function
        pub fn last_user_vote(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 65, 143, 160], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `n_gauge_types` (0x9fba03a1) function
        pub fn n_gauge_types(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([159, 186, 3, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `n_gauges` (0xe93841d0) function
        pub fn n_gauges(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([233, 56, 65, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `points_sum` (0xa9b48c01) function
        pub fn points_sum(
            &self,
            p0: i128,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Point> {
            self.0
                .method_hash([169, 180, 140, 1], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `points_total` (0x1142916b) function
        pub fn points_total(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([17, 66, 145, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `points_type_weight` (0xafd2bb49) function
        pub fn points_type_weight(
            &self,
            p0: i128,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 210, 187, 73], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `points_weight` (0xedba5273) function
        pub fn points_weight(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Point> {
            self.0
                .method_hash([237, 186, 82, 115], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `time_sum` (0x5a549158) function
        pub fn time_sum(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 84, 145, 88], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `time_total` (0x513872bd) function
        pub fn time_total(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 56, 114, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `time_type_weight` (0x51ce6b59) function
        pub fn time_type_weight(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 206, 107, 89], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `time_weight` (0xa4d7a250) function
        pub fn time_weight(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 215, 162, 80], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vote_for_gauge_weights` (0xd7136328) function
        pub fn vote_for_gauge_weights(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 19, 99, 40], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vote_user_power` (0x411e74b5) function
        pub fn vote_user_power(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 30, 116, 181], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vote_user_slopes` (0x0f467f98) function
        pub fn vote_user_slopes(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, VotedSlope> {
            self.0
                .method_hash([15, 70, 127, 152], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voting_escrow` (0xdfe05031) function
        pub fn voting_escrow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 224, 80, 49], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IOrionGaugeController<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `add_gauge` function with signature `add_gauge(address,int128,uint256)` and selector `0x18dfe921`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "add_gauge", abi = "add_gauge(address,int128,uint256)")]
    pub struct AddGaugeCall(
        pub ::ethers::core::types::Address,
        pub i128,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `add_type` function with signature `add_type(string,uint256)` and selector `0x92d0d232`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "add_type", abi = "add_type(string,uint256)")]
    pub struct AddTypeCall(pub String, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `apply_transfer_ownership` function with signature `apply_transfer_ownership()` and selector `0x6a1c05ae`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "apply_transfer_ownership", abi = "apply_transfer_ownership()")]
    pub struct ApplyTransferOwnershipCall;
    ///Container type for all input parameters for the `change_gauge_weight` function with signature `change_gauge_weight(address,uint256)` and selector `0xd4d2646e`
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
        name = "change_gauge_weight",
        abi = "change_gauge_weight(address,uint256)"
    )]
    pub struct ChangeGaugeWeightCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `change_global_emission_rate` function with signature `change_global_emission_rate(uint256)` and selector `0x01c08210`
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
        name = "change_global_emission_rate",
        abi = "change_global_emission_rate(uint256)"
    )]
    pub struct ChangeGlobalEmissionRateCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `change_type_weight` function with signature `change_type_weight(int128,uint256)` and selector `0xdb1ca260`
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
        name = "change_type_weight",
        abi = "change_type_weight(int128,uint256)"
    )]
    pub struct ChangeTypeWeightCall(pub i128, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `checkpoint` function with signature `checkpoint()` and selector `0xc2c4c5c1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "checkpoint", abi = "checkpoint()")]
    pub struct CheckpointCall;
    ///Container type for all input parameters for the `checkpoint_gauge` function with signature `checkpoint_gauge(address)` and selector `0x615e5237`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "checkpoint_gauge", abi = "checkpoint_gauge(address)")]
    pub struct CheckpointGaugeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `commit_transfer_ownership` function with signature `commit_transfer_ownership(address)` and selector `0x6b441a40`
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
        name = "commit_transfer_ownership",
        abi = "commit_transfer_ownership(address)"
    )]
    pub struct CommitTransferOwnershipCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `future_admin` function with signature `future_admin()` and selector `0x17f7182a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "future_admin", abi = "future_admin()")]
    pub struct FutureAdminCall;
    ///Container type for all input parameters for the `gauge_relative_weight` function with signature `gauge_relative_weight(address)` and selector `0x6207d866`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "gauge_relative_weight", abi = "gauge_relative_weight(address)")]
    pub struct GaugeRelativeWeight0Call(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `gauge_relative_weight` function with signature `gauge_relative_weight(address,uint256)` and selector `0xd3078c94`
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
        name = "gauge_relative_weight",
        abi = "gauge_relative_weight(address,uint256)"
    )]
    pub struct GaugeRelativeWeight1Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `gauge_relative_weight_write` function with signature `gauge_relative_weight_write(address,uint256)` and selector `0x6472eee1`
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
        name = "gauge_relative_weight_write",
        abi = "gauge_relative_weight_write(address,uint256)"
    )]
    pub struct GaugeRelativeWeightWrite1Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `gauge_relative_weight_write` function with signature `gauge_relative_weight_write(address)` and selector `0x95cfcec3`
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
        name = "gauge_relative_weight_write",
        abi = "gauge_relative_weight_write(address)"
    )]
    pub struct GaugeRelativeWeightWrite0Call(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `gauge_type_names` function with signature `gauge_type_names(int128)` and selector `0xd958a8fc`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "gauge_type_names", abi = "gauge_type_names(int128)")]
    pub struct GaugeTypeNamesCall(pub i128);
    ///Container type for all input parameters for the `gauge_types` function with signature `gauge_types(address)` and selector `0x3f9095b7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "gauge_types", abi = "gauge_types(address)")]
    pub struct GaugeTypesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `gauges` function with signature `gauges(uint256)` and selector `0xb0539187`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "gauges", abi = "gauges(uint256)")]
    pub struct GaugesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `get_gauge_weight` function with signature `get_gauge_weight(address)` and selector `0x4e791a3a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "get_gauge_weight", abi = "get_gauge_weight(address)")]
    pub struct GetGaugeWeightCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `get_total_weight` function with signature `get_total_weight()` and selector `0x6977ff92`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "get_total_weight", abi = "get_total_weight()")]
    pub struct GetTotalWeightCall;
    ///Container type for all input parameters for the `get_type_weight` function with signature `get_type_weight(int128)` and selector `0x72fdccfa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "get_type_weight", abi = "get_type_weight(int128)")]
    pub struct GetTypeWeightCall(pub i128);
    ///Container type for all input parameters for the `get_weights_sum_per_type` function with signature `get_weights_sum_per_type(int128)` and selector `0x6f214a6a`
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
        name = "get_weights_sum_per_type",
        abi = "get_weights_sum_per_type(int128)"
    )]
    pub struct GetWeightsSumPerTypeCall(pub i128);
    ///Container type for all input parameters for the `global_emission_rate` function with signature `global_emission_rate()` and selector `0x0a3be757`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "global_emission_rate", abi = "global_emission_rate()")]
    pub struct GlobalEmissionRateCall;
    ///Container type for all input parameters for the `last_user_vote` function with signature `last_user_vote(address,address)` and selector `0x7e418fa0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "last_user_vote", abi = "last_user_vote(address,address)")]
    pub struct LastUserVoteCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `n_gauge_types` function with signature `n_gauge_types()` and selector `0x9fba03a1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "n_gauge_types", abi = "n_gauge_types()")]
    pub struct NGaugeTypesCall;
    ///Container type for all input parameters for the `n_gauges` function with signature `n_gauges()` and selector `0xe93841d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "n_gauges", abi = "n_gauges()")]
    pub struct NGaugesCall;
    ///Container type for all input parameters for the `points_sum` function with signature `points_sum(int128,uint256)` and selector `0xa9b48c01`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "points_sum", abi = "points_sum(int128,uint256)")]
    pub struct PointsSumCall(pub i128, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `points_total` function with signature `points_total(uint256)` and selector `0x1142916b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "points_total", abi = "points_total(uint256)")]
    pub struct PointsTotalCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `points_type_weight` function with signature `points_type_weight(int128,uint256)` and selector `0xafd2bb49`
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
        name = "points_type_weight",
        abi = "points_type_weight(int128,uint256)"
    )]
    pub struct PointsTypeWeightCall(pub i128, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `points_weight` function with signature `points_weight(address,uint256)` and selector `0xedba5273`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "points_weight", abi = "points_weight(address,uint256)")]
    pub struct PointsWeightCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `time_sum` function with signature `time_sum(uint256)` and selector `0x5a549158`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "time_sum", abi = "time_sum(uint256)")]
    pub struct TimeSumCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `time_total` function with signature `time_total()` and selector `0x513872bd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "time_total", abi = "time_total()")]
    pub struct TimeTotalCall;
    ///Container type for all input parameters for the `time_type_weight` function with signature `time_type_weight(uint256)` and selector `0x51ce6b59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "time_type_weight", abi = "time_type_weight(uint256)")]
    pub struct TimeTypeWeightCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `time_weight` function with signature `time_weight(address)` and selector `0xa4d7a250`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "time_weight", abi = "time_weight(address)")]
    pub struct TimeWeightCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `vote_for_gauge_weights` function with signature `vote_for_gauge_weights(address,uint256)` and selector `0xd7136328`
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
        name = "vote_for_gauge_weights",
        abi = "vote_for_gauge_weights(address,uint256)"
    )]
    pub struct VoteForGaugeWeightsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `vote_user_power` function with signature `vote_user_power(address)` and selector `0x411e74b5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "vote_user_power", abi = "vote_user_power(address)")]
    pub struct VoteUserPowerCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `vote_user_slopes` function with signature `vote_user_slopes(address,address)` and selector `0x0f467f98`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "vote_user_slopes", abi = "vote_user_slopes(address,address)")]
    pub struct VoteUserSlopesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `voting_escrow` function with signature `voting_escrow()` and selector `0xdfe05031`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "voting_escrow", abi = "voting_escrow()")]
    pub struct VotingEscrowCall;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IOrionGaugeControllerCalls {
        AddGauge(AddGaugeCall),
        AddType(AddTypeCall),
        Admin(AdminCall),
        ApplyTransferOwnership(ApplyTransferOwnershipCall),
        ChangeGaugeWeight(ChangeGaugeWeightCall),
        ChangeGlobalEmissionRate(ChangeGlobalEmissionRateCall),
        ChangeTypeWeight(ChangeTypeWeightCall),
        Checkpoint(CheckpointCall),
        CheckpointGauge(CheckpointGaugeCall),
        CommitTransferOwnership(CommitTransferOwnershipCall),
        FutureAdmin(FutureAdminCall),
        GaugeRelativeWeight0(GaugeRelativeWeight0Call),
        GaugeRelativeWeight1(GaugeRelativeWeight1Call),
        GaugeRelativeWeightWrite1(GaugeRelativeWeightWrite1Call),
        GaugeRelativeWeightWrite0(GaugeRelativeWeightWrite0Call),
        GaugeTypeNames(GaugeTypeNamesCall),
        GaugeTypes(GaugeTypesCall),
        Gauges(GaugesCall),
        GetGaugeWeight(GetGaugeWeightCall),
        GetTotalWeight(GetTotalWeightCall),
        GetTypeWeight(GetTypeWeightCall),
        GetWeightsSumPerType(GetWeightsSumPerTypeCall),
        GlobalEmissionRate(GlobalEmissionRateCall),
        LastUserVote(LastUserVoteCall),
        NGaugeTypes(NGaugeTypesCall),
        NGauges(NGaugesCall),
        PointsSum(PointsSumCall),
        PointsTotal(PointsTotalCall),
        PointsTypeWeight(PointsTypeWeightCall),
        PointsWeight(PointsWeightCall),
        TimeSum(TimeSumCall),
        TimeTotal(TimeTotalCall),
        TimeTypeWeight(TimeTypeWeightCall),
        TimeWeight(TimeWeightCall),
        Token(TokenCall),
        VoteForGaugeWeights(VoteForGaugeWeightsCall),
        VoteUserPower(VoteUserPowerCall),
        VoteUserSlopes(VoteUserSlopesCall),
        VotingEscrow(VotingEscrowCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOrionGaugeControllerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::AddGauge(decoded));
            }
            if let Ok(decoded) =
                <AddTypeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::AddType(decoded));
            }
            if let Ok(decoded) =
                <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ApplyTransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IOrionGaugeControllerCalls::ApplyTransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ChangeGaugeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::ChangeGaugeWeight(decoded));
            }
            if let Ok(decoded) =
                <ChangeGlobalEmissionRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IOrionGaugeControllerCalls::ChangeGlobalEmissionRate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ChangeTypeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::ChangeTypeWeight(decoded));
            }
            if let Ok(decoded) =
                <CheckpointCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::Checkpoint(decoded));
            }
            if let Ok(decoded) =
                <CheckpointGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::CheckpointGauge(decoded));
            }
            if let Ok(decoded) =
                <CommitTransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IOrionGaugeControllerCalls::CommitTransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <FutureAdminCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::FutureAdmin(decoded));
            }
            if let Ok(decoded) =
                <GaugeRelativeWeight0Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GaugeRelativeWeight0(decoded));
            }
            if let Ok(decoded) =
                <GaugeRelativeWeight1Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GaugeRelativeWeight1(decoded));
            }
            if let Ok(decoded) =
                <GaugeRelativeWeightWrite1Call as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IOrionGaugeControllerCalls::GaugeRelativeWeightWrite1(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GaugeRelativeWeightWrite0Call as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IOrionGaugeControllerCalls::GaugeRelativeWeightWrite0(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GaugeTypeNamesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GaugeTypeNames(decoded));
            }
            if let Ok(decoded) =
                <GaugeTypesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GaugeTypes(decoded));
            }
            if let Ok(decoded) =
                <GaugesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::Gauges(decoded));
            }
            if let Ok(decoded) =
                <GetGaugeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GetGaugeWeight(decoded));
            }
            if let Ok(decoded) =
                <GetTotalWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GetTotalWeight(decoded));
            }
            if let Ok(decoded) =
                <GetTypeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GetTypeWeight(decoded));
            }
            if let Ok(decoded) =
                <GetWeightsSumPerTypeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GetWeightsSumPerType(decoded));
            }
            if let Ok(decoded) =
                <GlobalEmissionRateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::GlobalEmissionRate(decoded));
            }
            if let Ok(decoded) =
                <LastUserVoteCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::LastUserVote(decoded));
            }
            if let Ok(decoded) =
                <NGaugeTypesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::NGaugeTypes(decoded));
            }
            if let Ok(decoded) =
                <NGaugesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::NGauges(decoded));
            }
            if let Ok(decoded) =
                <PointsSumCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::PointsSum(decoded));
            }
            if let Ok(decoded) =
                <PointsTotalCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::PointsTotal(decoded));
            }
            if let Ok(decoded) =
                <PointsTypeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::PointsTypeWeight(decoded));
            }
            if let Ok(decoded) =
                <PointsWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::PointsWeight(decoded));
            }
            if let Ok(decoded) =
                <TimeSumCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::TimeSum(decoded));
            }
            if let Ok(decoded) =
                <TimeTotalCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::TimeTotal(decoded));
            }
            if let Ok(decoded) =
                <TimeTypeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::TimeTypeWeight(decoded));
            }
            if let Ok(decoded) =
                <TimeWeightCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::TimeWeight(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <VoteForGaugeWeightsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::VoteForGaugeWeights(decoded));
            }
            if let Ok(decoded) =
                <VoteUserPowerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::VoteUserPower(decoded));
            }
            if let Ok(decoded) =
                <VoteUserSlopesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::VoteUserSlopes(decoded));
            }
            if let Ok(decoded) =
                <VotingEscrowCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionGaugeControllerCalls::VotingEscrow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOrionGaugeControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOrionGaugeControllerCalls::AddGauge(element) => element.encode(),
                IOrionGaugeControllerCalls::AddType(element) => element.encode(),
                IOrionGaugeControllerCalls::Admin(element) => element.encode(),
                IOrionGaugeControllerCalls::ApplyTransferOwnership(element) => element.encode(),
                IOrionGaugeControllerCalls::ChangeGaugeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::ChangeGlobalEmissionRate(element) => element.encode(),
                IOrionGaugeControllerCalls::ChangeTypeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::Checkpoint(element) => element.encode(),
                IOrionGaugeControllerCalls::CheckpointGauge(element) => element.encode(),
                IOrionGaugeControllerCalls::CommitTransferOwnership(element) => element.encode(),
                IOrionGaugeControllerCalls::FutureAdmin(element) => element.encode(),
                IOrionGaugeControllerCalls::GaugeRelativeWeight0(element) => element.encode(),
                IOrionGaugeControllerCalls::GaugeRelativeWeight1(element) => element.encode(),
                IOrionGaugeControllerCalls::GaugeRelativeWeightWrite1(element) => element.encode(),
                IOrionGaugeControllerCalls::GaugeRelativeWeightWrite0(element) => element.encode(),
                IOrionGaugeControllerCalls::GaugeTypeNames(element) => element.encode(),
                IOrionGaugeControllerCalls::GaugeTypes(element) => element.encode(),
                IOrionGaugeControllerCalls::Gauges(element) => element.encode(),
                IOrionGaugeControllerCalls::GetGaugeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::GetTotalWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::GetTypeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::GetWeightsSumPerType(element) => element.encode(),
                IOrionGaugeControllerCalls::GlobalEmissionRate(element) => element.encode(),
                IOrionGaugeControllerCalls::LastUserVote(element) => element.encode(),
                IOrionGaugeControllerCalls::NGaugeTypes(element) => element.encode(),
                IOrionGaugeControllerCalls::NGauges(element) => element.encode(),
                IOrionGaugeControllerCalls::PointsSum(element) => element.encode(),
                IOrionGaugeControllerCalls::PointsTotal(element) => element.encode(),
                IOrionGaugeControllerCalls::PointsTypeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::PointsWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::TimeSum(element) => element.encode(),
                IOrionGaugeControllerCalls::TimeTotal(element) => element.encode(),
                IOrionGaugeControllerCalls::TimeTypeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::TimeWeight(element) => element.encode(),
                IOrionGaugeControllerCalls::Token(element) => element.encode(),
                IOrionGaugeControllerCalls::VoteForGaugeWeights(element) => element.encode(),
                IOrionGaugeControllerCalls::VoteUserPower(element) => element.encode(),
                IOrionGaugeControllerCalls::VoteUserSlopes(element) => element.encode(),
                IOrionGaugeControllerCalls::VotingEscrow(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOrionGaugeControllerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOrionGaugeControllerCalls::AddGauge(element) => element.fmt(f),
                IOrionGaugeControllerCalls::AddType(element) => element.fmt(f),
                IOrionGaugeControllerCalls::Admin(element) => element.fmt(f),
                IOrionGaugeControllerCalls::ApplyTransferOwnership(element) => element.fmt(f),
                IOrionGaugeControllerCalls::ChangeGaugeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::ChangeGlobalEmissionRate(element) => element.fmt(f),
                IOrionGaugeControllerCalls::ChangeTypeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::Checkpoint(element) => element.fmt(f),
                IOrionGaugeControllerCalls::CheckpointGauge(element) => element.fmt(f),
                IOrionGaugeControllerCalls::CommitTransferOwnership(element) => element.fmt(f),
                IOrionGaugeControllerCalls::FutureAdmin(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GaugeRelativeWeight0(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GaugeRelativeWeight1(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GaugeRelativeWeightWrite1(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GaugeRelativeWeightWrite0(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GaugeTypeNames(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GaugeTypes(element) => element.fmt(f),
                IOrionGaugeControllerCalls::Gauges(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GetGaugeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GetTotalWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GetTypeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GetWeightsSumPerType(element) => element.fmt(f),
                IOrionGaugeControllerCalls::GlobalEmissionRate(element) => element.fmt(f),
                IOrionGaugeControllerCalls::LastUserVote(element) => element.fmt(f),
                IOrionGaugeControllerCalls::NGaugeTypes(element) => element.fmt(f),
                IOrionGaugeControllerCalls::NGauges(element) => element.fmt(f),
                IOrionGaugeControllerCalls::PointsSum(element) => element.fmt(f),
                IOrionGaugeControllerCalls::PointsTotal(element) => element.fmt(f),
                IOrionGaugeControllerCalls::PointsTypeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::PointsWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::TimeSum(element) => element.fmt(f),
                IOrionGaugeControllerCalls::TimeTotal(element) => element.fmt(f),
                IOrionGaugeControllerCalls::TimeTypeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::TimeWeight(element) => element.fmt(f),
                IOrionGaugeControllerCalls::Token(element) => element.fmt(f),
                IOrionGaugeControllerCalls::VoteForGaugeWeights(element) => element.fmt(f),
                IOrionGaugeControllerCalls::VoteUserPower(element) => element.fmt(f),
                IOrionGaugeControllerCalls::VoteUserSlopes(element) => element.fmt(f),
                IOrionGaugeControllerCalls::VotingEscrow(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddGaugeCall> for IOrionGaugeControllerCalls {
        fn from(var: AddGaugeCall) -> Self {
            IOrionGaugeControllerCalls::AddGauge(var)
        }
    }
    impl ::std::convert::From<AddTypeCall> for IOrionGaugeControllerCalls {
        fn from(var: AddTypeCall) -> Self {
            IOrionGaugeControllerCalls::AddType(var)
        }
    }
    impl ::std::convert::From<AdminCall> for IOrionGaugeControllerCalls {
        fn from(var: AdminCall) -> Self {
            IOrionGaugeControllerCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ApplyTransferOwnershipCall> for IOrionGaugeControllerCalls {
        fn from(var: ApplyTransferOwnershipCall) -> Self {
            IOrionGaugeControllerCalls::ApplyTransferOwnership(var)
        }
    }
    impl ::std::convert::From<ChangeGaugeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: ChangeGaugeWeightCall) -> Self {
            IOrionGaugeControllerCalls::ChangeGaugeWeight(var)
        }
    }
    impl ::std::convert::From<ChangeGlobalEmissionRateCall> for IOrionGaugeControllerCalls {
        fn from(var: ChangeGlobalEmissionRateCall) -> Self {
            IOrionGaugeControllerCalls::ChangeGlobalEmissionRate(var)
        }
    }
    impl ::std::convert::From<ChangeTypeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: ChangeTypeWeightCall) -> Self {
            IOrionGaugeControllerCalls::ChangeTypeWeight(var)
        }
    }
    impl ::std::convert::From<CheckpointCall> for IOrionGaugeControllerCalls {
        fn from(var: CheckpointCall) -> Self {
            IOrionGaugeControllerCalls::Checkpoint(var)
        }
    }
    impl ::std::convert::From<CheckpointGaugeCall> for IOrionGaugeControllerCalls {
        fn from(var: CheckpointGaugeCall) -> Self {
            IOrionGaugeControllerCalls::CheckpointGauge(var)
        }
    }
    impl ::std::convert::From<CommitTransferOwnershipCall> for IOrionGaugeControllerCalls {
        fn from(var: CommitTransferOwnershipCall) -> Self {
            IOrionGaugeControllerCalls::CommitTransferOwnership(var)
        }
    }
    impl ::std::convert::From<FutureAdminCall> for IOrionGaugeControllerCalls {
        fn from(var: FutureAdminCall) -> Self {
            IOrionGaugeControllerCalls::FutureAdmin(var)
        }
    }
    impl ::std::convert::From<GaugeRelativeWeight0Call> for IOrionGaugeControllerCalls {
        fn from(var: GaugeRelativeWeight0Call) -> Self {
            IOrionGaugeControllerCalls::GaugeRelativeWeight0(var)
        }
    }
    impl ::std::convert::From<GaugeRelativeWeight1Call> for IOrionGaugeControllerCalls {
        fn from(var: GaugeRelativeWeight1Call) -> Self {
            IOrionGaugeControllerCalls::GaugeRelativeWeight1(var)
        }
    }
    impl ::std::convert::From<GaugeRelativeWeightWrite1Call> for IOrionGaugeControllerCalls {
        fn from(var: GaugeRelativeWeightWrite1Call) -> Self {
            IOrionGaugeControllerCalls::GaugeRelativeWeightWrite1(var)
        }
    }
    impl ::std::convert::From<GaugeRelativeWeightWrite0Call> for IOrionGaugeControllerCalls {
        fn from(var: GaugeRelativeWeightWrite0Call) -> Self {
            IOrionGaugeControllerCalls::GaugeRelativeWeightWrite0(var)
        }
    }
    impl ::std::convert::From<GaugeTypeNamesCall> for IOrionGaugeControllerCalls {
        fn from(var: GaugeTypeNamesCall) -> Self {
            IOrionGaugeControllerCalls::GaugeTypeNames(var)
        }
    }
    impl ::std::convert::From<GaugeTypesCall> for IOrionGaugeControllerCalls {
        fn from(var: GaugeTypesCall) -> Self {
            IOrionGaugeControllerCalls::GaugeTypes(var)
        }
    }
    impl ::std::convert::From<GaugesCall> for IOrionGaugeControllerCalls {
        fn from(var: GaugesCall) -> Self {
            IOrionGaugeControllerCalls::Gauges(var)
        }
    }
    impl ::std::convert::From<GetGaugeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: GetGaugeWeightCall) -> Self {
            IOrionGaugeControllerCalls::GetGaugeWeight(var)
        }
    }
    impl ::std::convert::From<GetTotalWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: GetTotalWeightCall) -> Self {
            IOrionGaugeControllerCalls::GetTotalWeight(var)
        }
    }
    impl ::std::convert::From<GetTypeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: GetTypeWeightCall) -> Self {
            IOrionGaugeControllerCalls::GetTypeWeight(var)
        }
    }
    impl ::std::convert::From<GetWeightsSumPerTypeCall> for IOrionGaugeControllerCalls {
        fn from(var: GetWeightsSumPerTypeCall) -> Self {
            IOrionGaugeControllerCalls::GetWeightsSumPerType(var)
        }
    }
    impl ::std::convert::From<GlobalEmissionRateCall> for IOrionGaugeControllerCalls {
        fn from(var: GlobalEmissionRateCall) -> Self {
            IOrionGaugeControllerCalls::GlobalEmissionRate(var)
        }
    }
    impl ::std::convert::From<LastUserVoteCall> for IOrionGaugeControllerCalls {
        fn from(var: LastUserVoteCall) -> Self {
            IOrionGaugeControllerCalls::LastUserVote(var)
        }
    }
    impl ::std::convert::From<NGaugeTypesCall> for IOrionGaugeControllerCalls {
        fn from(var: NGaugeTypesCall) -> Self {
            IOrionGaugeControllerCalls::NGaugeTypes(var)
        }
    }
    impl ::std::convert::From<NGaugesCall> for IOrionGaugeControllerCalls {
        fn from(var: NGaugesCall) -> Self {
            IOrionGaugeControllerCalls::NGauges(var)
        }
    }
    impl ::std::convert::From<PointsSumCall> for IOrionGaugeControllerCalls {
        fn from(var: PointsSumCall) -> Self {
            IOrionGaugeControllerCalls::PointsSum(var)
        }
    }
    impl ::std::convert::From<PointsTotalCall> for IOrionGaugeControllerCalls {
        fn from(var: PointsTotalCall) -> Self {
            IOrionGaugeControllerCalls::PointsTotal(var)
        }
    }
    impl ::std::convert::From<PointsTypeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: PointsTypeWeightCall) -> Self {
            IOrionGaugeControllerCalls::PointsTypeWeight(var)
        }
    }
    impl ::std::convert::From<PointsWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: PointsWeightCall) -> Self {
            IOrionGaugeControllerCalls::PointsWeight(var)
        }
    }
    impl ::std::convert::From<TimeSumCall> for IOrionGaugeControllerCalls {
        fn from(var: TimeSumCall) -> Self {
            IOrionGaugeControllerCalls::TimeSum(var)
        }
    }
    impl ::std::convert::From<TimeTotalCall> for IOrionGaugeControllerCalls {
        fn from(var: TimeTotalCall) -> Self {
            IOrionGaugeControllerCalls::TimeTotal(var)
        }
    }
    impl ::std::convert::From<TimeTypeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: TimeTypeWeightCall) -> Self {
            IOrionGaugeControllerCalls::TimeTypeWeight(var)
        }
    }
    impl ::std::convert::From<TimeWeightCall> for IOrionGaugeControllerCalls {
        fn from(var: TimeWeightCall) -> Self {
            IOrionGaugeControllerCalls::TimeWeight(var)
        }
    }
    impl ::std::convert::From<TokenCall> for IOrionGaugeControllerCalls {
        fn from(var: TokenCall) -> Self {
            IOrionGaugeControllerCalls::Token(var)
        }
    }
    impl ::std::convert::From<VoteForGaugeWeightsCall> for IOrionGaugeControllerCalls {
        fn from(var: VoteForGaugeWeightsCall) -> Self {
            IOrionGaugeControllerCalls::VoteForGaugeWeights(var)
        }
    }
    impl ::std::convert::From<VoteUserPowerCall> for IOrionGaugeControllerCalls {
        fn from(var: VoteUserPowerCall) -> Self {
            IOrionGaugeControllerCalls::VoteUserPower(var)
        }
    }
    impl ::std::convert::From<VoteUserSlopesCall> for IOrionGaugeControllerCalls {
        fn from(var: VoteUserSlopesCall) -> Self {
            IOrionGaugeControllerCalls::VoteUserSlopes(var)
        }
    }
    impl ::std::convert::From<VotingEscrowCall> for IOrionGaugeControllerCalls {
        fn from(var: VotingEscrowCall) -> Self {
            IOrionGaugeControllerCalls::VotingEscrow(var)
        }
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `future_admin` function with signature `future_admin()` and selector `0x17f7182a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct FutureAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gauge_relative_weight` function with signature `gauge_relative_weight(address)` and selector `0x6207d866`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeRelativeWeight0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gauge_relative_weight` function with signature `gauge_relative_weight(address,uint256)` and selector `0xd3078c94`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeRelativeWeight1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gauge_relative_weight_write` function with signature `gauge_relative_weight_write(address,uint256)` and selector `0x6472eee1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeRelativeWeightWrite1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gauge_relative_weight_write` function with signature `gauge_relative_weight_write(address)` and selector `0x95cfcec3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeRelativeWeightWrite0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gauge_type_names` function with signature `gauge_type_names(int128)` and selector `0xd958a8fc`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeTypeNamesReturn(pub String);
    ///Container type for all return fields from the `gauge_types` function with signature `gauge_types(address)` and selector `0x3f9095b7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugeTypesReturn(pub i128);
    ///Container type for all return fields from the `gauges` function with signature `gauges(uint256)` and selector `0xb0539187`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GaugesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `get_gauge_weight` function with signature `get_gauge_weight(address)` and selector `0x4e791a3a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetGaugeWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_total_weight` function with signature `get_total_weight()` and selector `0x6977ff92`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetTotalWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_type_weight` function with signature `get_type_weight(int128)` and selector `0x72fdccfa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetTypeWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_weights_sum_per_type` function with signature `get_weights_sum_per_type(int128)` and selector `0x6f214a6a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetWeightsSumPerTypeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `global_emission_rate` function with signature `global_emission_rate()` and selector `0x0a3be757`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GlobalEmissionRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `last_user_vote` function with signature `last_user_vote(address,address)` and selector `0x7e418fa0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct LastUserVoteReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `n_gauge_types` function with signature `n_gauge_types()` and selector `0x9fba03a1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NGaugeTypesReturn(pub i128);
    ///Container type for all return fields from the `n_gauges` function with signature `n_gauges()` and selector `0xe93841d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NGaugesReturn(pub i128);
    ///Container type for all return fields from the `points_sum` function with signature `points_sum(int128,uint256)` and selector `0xa9b48c01`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PointsSumReturn(pub Point);
    ///Container type for all return fields from the `points_total` function with signature `points_total(uint256)` and selector `0x1142916b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PointsTotalReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `points_type_weight` function with signature `points_type_weight(int128,uint256)` and selector `0xafd2bb49`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PointsTypeWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `points_weight` function with signature `points_weight(address,uint256)` and selector `0xedba5273`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PointsWeightReturn(pub Point);
    ///Container type for all return fields from the `time_sum` function with signature `time_sum(uint256)` and selector `0x5a549158`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TimeSumReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `time_total` function with signature `time_total()` and selector `0x513872bd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TimeTotalReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `time_type_weight` function with signature `time_type_weight(uint256)` and selector `0x51ce6b59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TimeTypeWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `time_weight` function with signature `time_weight(address)` and selector `0xa4d7a250`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TimeWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `vote_user_power` function with signature `vote_user_power(address)` and selector `0x411e74b5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct VoteUserPowerReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vote_user_slopes` function with signature `vote_user_slopes(address,address)` and selector `0x0f467f98`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct VoteUserSlopesReturn(pub VotedSlope);
    ///Container type for all return fields from the `voting_escrow` function with signature `voting_escrow()` and selector `0xdfe05031`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct VotingEscrowReturn(pub ::ethers::core::types::Address);
    ///`Point(uint256,uint256)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct Point {
        pub bias: ::ethers::core::types::U256,
        pub slope: ::ethers::core::types::U256,
    }
    ///`VotedSlope(uint256,uint256,uint256)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct VotedSlope {
        pub slope: ::ethers::core::types::U256,
        pub power: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
}
