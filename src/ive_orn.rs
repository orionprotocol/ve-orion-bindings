pub use ive_orn::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod ive_orn {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IveORN was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"apply_smart_wallet_checker\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"apply_transfer_ownership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_t\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_block\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfAt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newController\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeController\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"checkpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"commit_smart_wallet_checker\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"commit_transfer_ownership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_unlock_time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"create_lock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit_for\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"emergencyUnlockActive\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"epoch\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"future_admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"future_smart_wallet_checker\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_last_user_slope\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increase_amount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_unlock_time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increase_unlock_time\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"locked\",\"outputs\":[{\"internalType\":\"struct IveORN.LockedBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"end\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"locked__end\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"arg0\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"point_history\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"bias\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"slope\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ts\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blk\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fxs_amt\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"recoverERC20\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"arg0\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"slope_changes\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"smart_wallet_checker\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"toggleEmergencyUnlock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalFXSSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_block\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalFXSSupplyAt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"t\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_block\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupplyAt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transfersEnabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"arg0\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"user_point_epoch\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"arg0\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"arg1\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"user_point_history\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"bias\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"slope\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ts\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blk\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fxs_amt\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_idx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"user_point_history__ts\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IVEORN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IveORN<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IveORN<M> {
        fn clone(&self) -> Self {
            IveORN(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IveORN<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IveORN<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IveORN))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IveORN<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IVEORN_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `apply_smart_wallet_checker` (0x8e5b490f) function
        pub fn apply_smart_wallet_checker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 91, 73, 15], ())
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
        ///Calls the contract's `balanceOf` (0x00fdd58e) function
        pub fn balance_of_with_t(
            &self,
            addr: ::ethers::core::types::Address,
            t: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (addr, t))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfAt` (0x4ee2cd7e) function
        pub fn balance_of_at(
            &self,
            addr: ::ethers::core::types::Address,
            block: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 226, 205, 126], (addr, block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeController` (0x3cebb823) function
        pub fn change_controller(
            &self,
            new_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 235, 184, 35], new_controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoint` (0xc2c4c5c1) function
        pub fn checkpoint(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 196, 197, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commit_smart_wallet_checker` (0x57f901e2) function
        pub fn commit_smart_wallet_checker(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 249, 1, 226], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commit_transfer_ownership` (0x6b441a40) function
        pub fn commit_transfer_ownership(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 68, 26, 64], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `controller` (0xf77c4791) function
        pub fn controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([247, 124, 71, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `create_lock` (0x65fc3873) function
        pub fn create_lock(
            &self,
            value: ::ethers::core::types::U256,
            unlock_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 252, 56, 115], (value, unlock_time))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit_for` (0x3a46273e) function
        pub fn deposit_for(
            &self,
            addr: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 70, 39, 62], (addr, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emergencyUnlockActive` (0xf8946485) function
        pub fn emergency_unlock_active(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 148, 100, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `epoch` (0x900cf0cf) function
        pub fn epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 12, 240, 207], ())
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
        ///Calls the contract's `future_smart_wallet_checker` (0x8ff36fd1) function
        pub fn future_smart_wallet_checker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([143, 243, 111, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_last_user_slope` (0x7c74a174) function
        pub fn get_last_user_slope(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([124, 116, 161, 116], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increase_amount` (0x4957677c) function
        pub fn increase_amount(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 87, 103, 124], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increase_unlock_time` (0xeff7a612) function
        pub fn increase_unlock_time(
            &self,
            unlock_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 247, 166, 18], unlock_time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xcbf9fe5f) function
        pub fn locked(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, LockedBalance> {
            self.0
                .method_hash([203, 249, 254, 95], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked__end` (0xadc63589) function
        pub fn locked_end(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 198, 53, 137], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `point_history` (0xd1febfb9) function
        pub fn point_history(
            &self,
            arg_0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                i128,
                i128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([209, 254, 191, 185], arg_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recoverERC20` (0x8980f11f) function
        pub fn recover_erc20(
            &self,
            token_addr: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 128, 241, 31], (token_addr, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slope_changes` (0x71197484) function
        pub fn slope_changes(
            &self,
            arg_0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([113, 25, 116, 132], arg_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `smart_wallet_checker` (0x7175d4f7) function
        pub fn smart_wallet_checker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([113, 117, 212, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supply` (0x047fc9aa) function
        pub fn supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 127, 201, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toggleEmergencyUnlock` (0x88c2b3e3) function
        pub fn toggle_emergency_unlock(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 194, 179, 227], ())
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
        ///Calls the contract's `totalFXSSupply` (0xc3ad8956) function
        pub fn total_fxs_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 173, 137, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalFXSSupplyAt` (0x4f8ab24f) function
        pub fn total_fxs_supply_at(
            &self,
            block: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 138, 178, 79], block)
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
        ///Calls the contract's `totalSupply` (0xbd85b039) function
        pub fn total_supply_with_t(
            &self,
            t: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 133, 176, 57], t)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupplyAt` (0x981b24d0) function
        pub fn total_supply_at(
            &self,
            block: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 27, 36, 208], block)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfersEnabled` (0xbef97c87) function
        pub fn transfers_enabled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([190, 249, 124, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `user_point_epoch` (0x010ae757) function
        pub fn user_point_epoch(
            &self,
            arg_0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 10, 231, 87], arg_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `user_point_history` (0x28d09d47) function
        pub fn user_point_history(
            &self,
            arg_0: ::ethers::core::types::Address,
            arg_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                i128,
                i128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([40, 208, 157, 71], (arg_0, arg_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `user_point_history__ts` (0xda020a18) function
        pub fn user_point_history_ts(
            &self,
            addr: ::ethers::core::types::Address,
            idx: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 2, 10, 24], (addr, idx))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IveORN<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all input parameters for the `apply_smart_wallet_checker` function with signature `apply_smart_wallet_checker()` and selector `0x8e5b490f`
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
        name = "apply_smart_wallet_checker",
        abi = "apply_smart_wallet_checker()"
    )]
    pub struct ApplySmartWalletCheckerCall;
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
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfWithTCall {
        pub addr: ::ethers::core::types::Address,
        pub t: ::ethers::core::types::U256,
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
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOfAt` function with signature `balanceOfAt(address,uint256)` and selector `0x4ee2cd7e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOfAt", abi = "balanceOfAt(address,uint256)")]
    pub struct BalanceOfAtCall {
        pub addr: ::ethers::core::types::Address,
        pub block: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeController` function with signature `changeController(address)` and selector `0x3cebb823`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "changeController", abi = "changeController(address)")]
    pub struct ChangeControllerCall {
        pub new_controller: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `commit_smart_wallet_checker` function with signature `commit_smart_wallet_checker(address)` and selector `0x57f901e2`
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
        name = "commit_smart_wallet_checker",
        abi = "commit_smart_wallet_checker(address)"
    )]
    pub struct CommitSmartWalletCheckerCall {
        pub addr: ::ethers::core::types::Address,
    }
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
    pub struct CommitTransferOwnershipCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `controller` function with signature `controller()` and selector `0xf77c4791`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "controller", abi = "controller()")]
    pub struct ControllerCall;
    ///Container type for all input parameters for the `create_lock` function with signature `create_lock(uint256,uint256)` and selector `0x65fc3873`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "create_lock", abi = "create_lock(uint256,uint256)")]
    pub struct CreateLockCall {
        pub value: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `deposit_for` function with signature `deposit_for(address,uint256)` and selector `0x3a46273e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "deposit_for", abi = "deposit_for(address,uint256)")]
    pub struct DepositForCall {
        pub addr: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emergencyUnlockActive` function with signature `emergencyUnlockActive()` and selector `0xf8946485`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "emergencyUnlockActive", abi = "emergencyUnlockActive()")]
    pub struct EmergencyUnlockActiveCall;
    ///Container type for all input parameters for the `epoch` function with signature `epoch()` and selector `0x900cf0cf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "epoch", abi = "epoch()")]
    pub struct EpochCall;
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
    ///Container type for all input parameters for the `future_smart_wallet_checker` function with signature `future_smart_wallet_checker()` and selector `0x8ff36fd1`
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
        name = "future_smart_wallet_checker",
        abi = "future_smart_wallet_checker()"
    )]
    pub struct FutureSmartWalletCheckerCall;
    ///Container type for all input parameters for the `get_last_user_slope` function with signature `get_last_user_slope(address)` and selector `0x7c74a174`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "get_last_user_slope", abi = "get_last_user_slope(address)")]
    pub struct GetLastUserSlopeCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `increase_amount` function with signature `increase_amount(uint256)` and selector `0x4957677c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "increase_amount", abi = "increase_amount(uint256)")]
    pub struct IncreaseAmountCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `increase_unlock_time` function with signature `increase_unlock_time(uint256)` and selector `0xeff7a612`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "increase_unlock_time", abi = "increase_unlock_time(uint256)")]
    pub struct IncreaseUnlockTimeCall {
        pub unlock_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `locked` function with signature `locked(address)` and selector `0xcbf9fe5f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "locked", abi = "locked(address)")]
    pub struct LockedCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `locked__end` function with signature `locked__end(address)` and selector `0xadc63589`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "locked__end", abi = "locked__end(address)")]
    pub struct LockedEndCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `point_history` function with signature `point_history(uint256)` and selector `0xd1febfb9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "point_history", abi = "point_history(uint256)")]
    pub struct PointHistoryCall {
        pub arg_0: ::ethers::core::types::U256,
    }
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
        pub token_addr: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `slope_changes` function with signature `slope_changes(uint256)` and selector `0x71197484`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "slope_changes", abi = "slope_changes(uint256)")]
    pub struct SlopeChangesCall {
        pub arg_0: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `smart_wallet_checker` function with signature `smart_wallet_checker()` and selector `0x7175d4f7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "smart_wallet_checker", abi = "smart_wallet_checker()")]
    pub struct SmartWalletCheckerCall;
    ///Container type for all input parameters for the `supply` function with signature `supply()` and selector `0x047fc9aa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "supply", abi = "supply()")]
    pub struct SupplyCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `toggleEmergencyUnlock` function with signature `toggleEmergencyUnlock()` and selector `0x88c2b3e3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "toggleEmergencyUnlock", abi = "toggleEmergencyUnlock()")]
    pub struct ToggleEmergencyUnlockCall;
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
    ///Container type for all input parameters for the `totalFXSSupply` function with signature `totalFXSSupply()` and selector `0xc3ad8956`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalFXSSupply", abi = "totalFXSSupply()")]
    pub struct TotalFXSSupplyCall;
    ///Container type for all input parameters for the `totalFXSSupplyAt` function with signature `totalFXSSupplyAt(uint256)` and selector `0x4f8ab24f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalFXSSupplyAt", abi = "totalFXSSupplyAt(uint256)")]
    pub struct TotalFXSSupplyAtCall {
        pub block: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply(uint256)` and selector `0xbd85b039`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply(uint256)")]
    pub struct TotalSupplyWithTCall {
        pub t: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupplyAt` function with signature `totalSupplyAt(uint256)` and selector `0x981b24d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupplyAt", abi = "totalSupplyAt(uint256)")]
    pub struct TotalSupplyAtCall {
        pub block: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transfersEnabled` function with signature `transfersEnabled()` and selector `0xbef97c87`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfersEnabled", abi = "transfersEnabled()")]
    pub struct TransfersEnabledCall;
    ///Container type for all input parameters for the `user_point_epoch` function with signature `user_point_epoch(address)` and selector `0x010ae757`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "user_point_epoch", abi = "user_point_epoch(address)")]
    pub struct UserPointEpochCall {
        pub arg_0: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `user_point_history` function with signature `user_point_history(address,uint256)` and selector `0x28d09d47`
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
        name = "user_point_history",
        abi = "user_point_history(address,uint256)"
    )]
    pub struct UserPointHistoryCall {
        pub arg_0: ::ethers::core::types::Address,
        pub arg_1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `user_point_history__ts` function with signature `user_point_history__ts(address,uint256)` and selector `0xda020a18`
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
        name = "user_point_history__ts",
        abi = "user_point_history__ts(address,uint256)"
    )]
    pub struct UserPointHistoryTsCall {
        pub addr: ::ethers::core::types::Address,
        pub idx: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw()` and selector `0x3ccfd60b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IveORNCalls {
        Admin(AdminCall),
        ApplySmartWalletChecker(ApplySmartWalletCheckerCall),
        ApplyTransferOwnership(ApplyTransferOwnershipCall),
        BalanceOfWithT(BalanceOfWithTCall),
        BalanceOf(BalanceOfCall),
        BalanceOfAt(BalanceOfAtCall),
        ChangeController(ChangeControllerCall),
        Checkpoint(CheckpointCall),
        CommitSmartWalletChecker(CommitSmartWalletCheckerCall),
        CommitTransferOwnership(CommitTransferOwnershipCall),
        Controller(ControllerCall),
        CreateLock(CreateLockCall),
        Decimals(DecimalsCall),
        DepositFor(DepositForCall),
        EmergencyUnlockActive(EmergencyUnlockActiveCall),
        Epoch(EpochCall),
        FutureAdmin(FutureAdminCall),
        FutureSmartWalletChecker(FutureSmartWalletCheckerCall),
        GetLastUserSlope(GetLastUserSlopeCall),
        IncreaseAmount(IncreaseAmountCall),
        IncreaseUnlockTime(IncreaseUnlockTimeCall),
        Locked(LockedCall),
        LockedEnd(LockedEndCall),
        Name(NameCall),
        PointHistory(PointHistoryCall),
        RecoverERC20(RecoverERC20Call),
        SlopeChanges(SlopeChangesCall),
        SmartWalletChecker(SmartWalletCheckerCall),
        Supply(SupplyCall),
        Symbol(SymbolCall),
        ToggleEmergencyUnlock(ToggleEmergencyUnlockCall),
        Token(TokenCall),
        TotalFXSSupply(TotalFXSSupplyCall),
        TotalFXSSupplyAt(TotalFXSSupplyAtCall),
        TotalSupply(TotalSupplyCall),
        TotalSupplyWithT(TotalSupplyWithTCall),
        TotalSupplyAt(TotalSupplyAtCall),
        TransfersEnabled(TransfersEnabledCall),
        UserPointEpoch(UserPointEpochCall),
        UserPointHistory(UserPointHistoryCall),
        UserPointHistoryTs(UserPointHistoryTsCall),
        Version(VersionCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for IveORNCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ApplySmartWalletCheckerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IveORNCalls::ApplySmartWalletChecker(decoded));
            }
            if let Ok(decoded) =
                <ApplyTransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IveORNCalls::ApplyTransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfWithTCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::BalanceOfWithT(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfAtCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::BalanceOfAt(decoded));
            }
            if let Ok(decoded) =
                <ChangeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::ChangeController(decoded));
            }
            if let Ok(decoded) =
                <CheckpointCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Checkpoint(decoded));
            }
            if let Ok(decoded) =
                <CommitSmartWalletCheckerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IveORNCalls::CommitSmartWalletChecker(decoded));
            }
            if let Ok(decoded) =
                <CommitTransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IveORNCalls::CommitTransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <CreateLockCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::CreateLock(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DepositForCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::DepositFor(decoded));
            }
            if let Ok(decoded) =
                <EmergencyUnlockActiveCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::EmergencyUnlockActive(decoded));
            }
            if let Ok(decoded) =
                <EpochCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Epoch(decoded));
            }
            if let Ok(decoded) =
                <FutureAdminCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::FutureAdmin(decoded));
            }
            if let Ok(decoded) =
                <FutureSmartWalletCheckerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IveORNCalls::FutureSmartWalletChecker(decoded));
            }
            if let Ok(decoded) =
                <GetLastUserSlopeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::GetLastUserSlope(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAmountCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::IncreaseAmount(decoded));
            }
            if let Ok(decoded) =
                <IncreaseUnlockTimeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::IncreaseUnlockTime(decoded));
            }
            if let Ok(decoded) =
                <LockedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Locked(decoded));
            }
            if let Ok(decoded) =
                <LockedEndCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::LockedEnd(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PointHistoryCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::PointHistory(decoded));
            }
            if let Ok(decoded) =
                <RecoverERC20Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::RecoverERC20(decoded));
            }
            if let Ok(decoded) =
                <SlopeChangesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::SlopeChanges(decoded));
            }
            if let Ok(decoded) =
                <SmartWalletCheckerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::SmartWalletChecker(decoded));
            }
            if let Ok(decoded) =
                <SupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Supply(decoded));
            }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <ToggleEmergencyUnlockCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::ToggleEmergencyUnlock(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <TotalFXSSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::TotalFXSSupply(decoded));
            }
            if let Ok(decoded) =
                <TotalFXSSupplyAtCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::TotalFXSSupplyAt(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyWithTCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::TotalSupplyWithT(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyAtCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::TotalSupplyAt(decoded));
            }
            if let Ok(decoded) =
                <TransfersEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::TransfersEnabled(decoded));
            }
            if let Ok(decoded) =
                <UserPointEpochCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::UserPointEpoch(decoded));
            }
            if let Ok(decoded) =
                <UserPointHistoryCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::UserPointHistory(decoded));
            }
            if let Ok(decoded) =
                <UserPointHistoryTsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::UserPointHistoryTs(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IveORNCalls::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IveORNCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IveORNCalls::Admin(element) => element.encode(),
                IveORNCalls::ApplySmartWalletChecker(element) => element.encode(),
                IveORNCalls::ApplyTransferOwnership(element) => element.encode(),
                IveORNCalls::BalanceOfWithT(element) => element.encode(),
                IveORNCalls::BalanceOf(element) => element.encode(),
                IveORNCalls::BalanceOfAt(element) => element.encode(),
                IveORNCalls::ChangeController(element) => element.encode(),
                IveORNCalls::Checkpoint(element) => element.encode(),
                IveORNCalls::CommitSmartWalletChecker(element) => element.encode(),
                IveORNCalls::CommitTransferOwnership(element) => element.encode(),
                IveORNCalls::Controller(element) => element.encode(),
                IveORNCalls::CreateLock(element) => element.encode(),
                IveORNCalls::Decimals(element) => element.encode(),
                IveORNCalls::DepositFor(element) => element.encode(),
                IveORNCalls::EmergencyUnlockActive(element) => element.encode(),
                IveORNCalls::Epoch(element) => element.encode(),
                IveORNCalls::FutureAdmin(element) => element.encode(),
                IveORNCalls::FutureSmartWalletChecker(element) => element.encode(),
                IveORNCalls::GetLastUserSlope(element) => element.encode(),
                IveORNCalls::IncreaseAmount(element) => element.encode(),
                IveORNCalls::IncreaseUnlockTime(element) => element.encode(),
                IveORNCalls::Locked(element) => element.encode(),
                IveORNCalls::LockedEnd(element) => element.encode(),
                IveORNCalls::Name(element) => element.encode(),
                IveORNCalls::PointHistory(element) => element.encode(),
                IveORNCalls::RecoverERC20(element) => element.encode(),
                IveORNCalls::SlopeChanges(element) => element.encode(),
                IveORNCalls::SmartWalletChecker(element) => element.encode(),
                IveORNCalls::Supply(element) => element.encode(),
                IveORNCalls::Symbol(element) => element.encode(),
                IveORNCalls::ToggleEmergencyUnlock(element) => element.encode(),
                IveORNCalls::Token(element) => element.encode(),
                IveORNCalls::TotalFXSSupply(element) => element.encode(),
                IveORNCalls::TotalFXSSupplyAt(element) => element.encode(),
                IveORNCalls::TotalSupply(element) => element.encode(),
                IveORNCalls::TotalSupplyWithT(element) => element.encode(),
                IveORNCalls::TotalSupplyAt(element) => element.encode(),
                IveORNCalls::TransfersEnabled(element) => element.encode(),
                IveORNCalls::UserPointEpoch(element) => element.encode(),
                IveORNCalls::UserPointHistory(element) => element.encode(),
                IveORNCalls::UserPointHistoryTs(element) => element.encode(),
                IveORNCalls::Version(element) => element.encode(),
                IveORNCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IveORNCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IveORNCalls::Admin(element) => element.fmt(f),
                IveORNCalls::ApplySmartWalletChecker(element) => element.fmt(f),
                IveORNCalls::ApplyTransferOwnership(element) => element.fmt(f),
                IveORNCalls::BalanceOfWithT(element) => element.fmt(f),
                IveORNCalls::BalanceOf(element) => element.fmt(f),
                IveORNCalls::BalanceOfAt(element) => element.fmt(f),
                IveORNCalls::ChangeController(element) => element.fmt(f),
                IveORNCalls::Checkpoint(element) => element.fmt(f),
                IveORNCalls::CommitSmartWalletChecker(element) => element.fmt(f),
                IveORNCalls::CommitTransferOwnership(element) => element.fmt(f),
                IveORNCalls::Controller(element) => element.fmt(f),
                IveORNCalls::CreateLock(element) => element.fmt(f),
                IveORNCalls::Decimals(element) => element.fmt(f),
                IveORNCalls::DepositFor(element) => element.fmt(f),
                IveORNCalls::EmergencyUnlockActive(element) => element.fmt(f),
                IveORNCalls::Epoch(element) => element.fmt(f),
                IveORNCalls::FutureAdmin(element) => element.fmt(f),
                IveORNCalls::FutureSmartWalletChecker(element) => element.fmt(f),
                IveORNCalls::GetLastUserSlope(element) => element.fmt(f),
                IveORNCalls::IncreaseAmount(element) => element.fmt(f),
                IveORNCalls::IncreaseUnlockTime(element) => element.fmt(f),
                IveORNCalls::Locked(element) => element.fmt(f),
                IveORNCalls::LockedEnd(element) => element.fmt(f),
                IveORNCalls::Name(element) => element.fmt(f),
                IveORNCalls::PointHistory(element) => element.fmt(f),
                IveORNCalls::RecoverERC20(element) => element.fmt(f),
                IveORNCalls::SlopeChanges(element) => element.fmt(f),
                IveORNCalls::SmartWalletChecker(element) => element.fmt(f),
                IveORNCalls::Supply(element) => element.fmt(f),
                IveORNCalls::Symbol(element) => element.fmt(f),
                IveORNCalls::ToggleEmergencyUnlock(element) => element.fmt(f),
                IveORNCalls::Token(element) => element.fmt(f),
                IveORNCalls::TotalFXSSupply(element) => element.fmt(f),
                IveORNCalls::TotalFXSSupplyAt(element) => element.fmt(f),
                IveORNCalls::TotalSupply(element) => element.fmt(f),
                IveORNCalls::TotalSupplyWithT(element) => element.fmt(f),
                IveORNCalls::TotalSupplyAt(element) => element.fmt(f),
                IveORNCalls::TransfersEnabled(element) => element.fmt(f),
                IveORNCalls::UserPointEpoch(element) => element.fmt(f),
                IveORNCalls::UserPointHistory(element) => element.fmt(f),
                IveORNCalls::UserPointHistoryTs(element) => element.fmt(f),
                IveORNCalls::Version(element) => element.fmt(f),
                IveORNCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AdminCall> for IveORNCalls {
        fn from(var: AdminCall) -> Self {
            IveORNCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ApplySmartWalletCheckerCall> for IveORNCalls {
        fn from(var: ApplySmartWalletCheckerCall) -> Self {
            IveORNCalls::ApplySmartWalletChecker(var)
        }
    }
    impl ::std::convert::From<ApplyTransferOwnershipCall> for IveORNCalls {
        fn from(var: ApplyTransferOwnershipCall) -> Self {
            IveORNCalls::ApplyTransferOwnership(var)
        }
    }
    impl ::std::convert::From<BalanceOfWithTCall> for IveORNCalls {
        fn from(var: BalanceOfWithTCall) -> Self {
            IveORNCalls::BalanceOfWithT(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IveORNCalls {
        fn from(var: BalanceOfCall) -> Self {
            IveORNCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfAtCall> for IveORNCalls {
        fn from(var: BalanceOfAtCall) -> Self {
            IveORNCalls::BalanceOfAt(var)
        }
    }
    impl ::std::convert::From<ChangeControllerCall> for IveORNCalls {
        fn from(var: ChangeControllerCall) -> Self {
            IveORNCalls::ChangeController(var)
        }
    }
    impl ::std::convert::From<CheckpointCall> for IveORNCalls {
        fn from(var: CheckpointCall) -> Self {
            IveORNCalls::Checkpoint(var)
        }
    }
    impl ::std::convert::From<CommitSmartWalletCheckerCall> for IveORNCalls {
        fn from(var: CommitSmartWalletCheckerCall) -> Self {
            IveORNCalls::CommitSmartWalletChecker(var)
        }
    }
    impl ::std::convert::From<CommitTransferOwnershipCall> for IveORNCalls {
        fn from(var: CommitTransferOwnershipCall) -> Self {
            IveORNCalls::CommitTransferOwnership(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for IveORNCalls {
        fn from(var: ControllerCall) -> Self {
            IveORNCalls::Controller(var)
        }
    }
    impl ::std::convert::From<CreateLockCall> for IveORNCalls {
        fn from(var: CreateLockCall) -> Self {
            IveORNCalls::CreateLock(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for IveORNCalls {
        fn from(var: DecimalsCall) -> Self {
            IveORNCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DepositForCall> for IveORNCalls {
        fn from(var: DepositForCall) -> Self {
            IveORNCalls::DepositFor(var)
        }
    }
    impl ::std::convert::From<EmergencyUnlockActiveCall> for IveORNCalls {
        fn from(var: EmergencyUnlockActiveCall) -> Self {
            IveORNCalls::EmergencyUnlockActive(var)
        }
    }
    impl ::std::convert::From<EpochCall> for IveORNCalls {
        fn from(var: EpochCall) -> Self {
            IveORNCalls::Epoch(var)
        }
    }
    impl ::std::convert::From<FutureAdminCall> for IveORNCalls {
        fn from(var: FutureAdminCall) -> Self {
            IveORNCalls::FutureAdmin(var)
        }
    }
    impl ::std::convert::From<FutureSmartWalletCheckerCall> for IveORNCalls {
        fn from(var: FutureSmartWalletCheckerCall) -> Self {
            IveORNCalls::FutureSmartWalletChecker(var)
        }
    }
    impl ::std::convert::From<GetLastUserSlopeCall> for IveORNCalls {
        fn from(var: GetLastUserSlopeCall) -> Self {
            IveORNCalls::GetLastUserSlope(var)
        }
    }
    impl ::std::convert::From<IncreaseAmountCall> for IveORNCalls {
        fn from(var: IncreaseAmountCall) -> Self {
            IveORNCalls::IncreaseAmount(var)
        }
    }
    impl ::std::convert::From<IncreaseUnlockTimeCall> for IveORNCalls {
        fn from(var: IncreaseUnlockTimeCall) -> Self {
            IveORNCalls::IncreaseUnlockTime(var)
        }
    }
    impl ::std::convert::From<LockedCall> for IveORNCalls {
        fn from(var: LockedCall) -> Self {
            IveORNCalls::Locked(var)
        }
    }
    impl ::std::convert::From<LockedEndCall> for IveORNCalls {
        fn from(var: LockedEndCall) -> Self {
            IveORNCalls::LockedEnd(var)
        }
    }
    impl ::std::convert::From<NameCall> for IveORNCalls {
        fn from(var: NameCall) -> Self {
            IveORNCalls::Name(var)
        }
    }
    impl ::std::convert::From<PointHistoryCall> for IveORNCalls {
        fn from(var: PointHistoryCall) -> Self {
            IveORNCalls::PointHistory(var)
        }
    }
    impl ::std::convert::From<RecoverERC20Call> for IveORNCalls {
        fn from(var: RecoverERC20Call) -> Self {
            IveORNCalls::RecoverERC20(var)
        }
    }
    impl ::std::convert::From<SlopeChangesCall> for IveORNCalls {
        fn from(var: SlopeChangesCall) -> Self {
            IveORNCalls::SlopeChanges(var)
        }
    }
    impl ::std::convert::From<SmartWalletCheckerCall> for IveORNCalls {
        fn from(var: SmartWalletCheckerCall) -> Self {
            IveORNCalls::SmartWalletChecker(var)
        }
    }
    impl ::std::convert::From<SupplyCall> for IveORNCalls {
        fn from(var: SupplyCall) -> Self {
            IveORNCalls::Supply(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for IveORNCalls {
        fn from(var: SymbolCall) -> Self {
            IveORNCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<ToggleEmergencyUnlockCall> for IveORNCalls {
        fn from(var: ToggleEmergencyUnlockCall) -> Self {
            IveORNCalls::ToggleEmergencyUnlock(var)
        }
    }
    impl ::std::convert::From<TokenCall> for IveORNCalls {
        fn from(var: TokenCall) -> Self {
            IveORNCalls::Token(var)
        }
    }
    impl ::std::convert::From<TotalFXSSupplyCall> for IveORNCalls {
        fn from(var: TotalFXSSupplyCall) -> Self {
            IveORNCalls::TotalFXSSupply(var)
        }
    }
    impl ::std::convert::From<TotalFXSSupplyAtCall> for IveORNCalls {
        fn from(var: TotalFXSSupplyAtCall) -> Self {
            IveORNCalls::TotalFXSSupplyAt(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IveORNCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IveORNCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TotalSupplyWithTCall> for IveORNCalls {
        fn from(var: TotalSupplyWithTCall) -> Self {
            IveORNCalls::TotalSupplyWithT(var)
        }
    }
    impl ::std::convert::From<TotalSupplyAtCall> for IveORNCalls {
        fn from(var: TotalSupplyAtCall) -> Self {
            IveORNCalls::TotalSupplyAt(var)
        }
    }
    impl ::std::convert::From<TransfersEnabledCall> for IveORNCalls {
        fn from(var: TransfersEnabledCall) -> Self {
            IveORNCalls::TransfersEnabled(var)
        }
    }
    impl ::std::convert::From<UserPointEpochCall> for IveORNCalls {
        fn from(var: UserPointEpochCall) -> Self {
            IveORNCalls::UserPointEpoch(var)
        }
    }
    impl ::std::convert::From<UserPointHistoryCall> for IveORNCalls {
        fn from(var: UserPointHistoryCall) -> Self {
            IveORNCalls::UserPointHistory(var)
        }
    }
    impl ::std::convert::From<UserPointHistoryTsCall> for IveORNCalls {
        fn from(var: UserPointHistoryTsCall) -> Self {
            IveORNCalls::UserPointHistoryTs(var)
        }
    }
    impl ::std::convert::From<VersionCall> for IveORNCalls {
        fn from(var: VersionCall) -> Self {
            IveORNCalls::Version(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IveORNCalls {
        fn from(var: WithdrawCall) -> Self {
            IveORNCalls::Withdraw(var)
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
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfWithTReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `balanceOfAt` function with signature `balanceOfAt(address,uint256)` and selector `0x4ee2cd7e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfAtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `controller` function with signature `controller()` and selector `0xf77c4791`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct ControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `emergencyUnlockActive` function with signature `emergencyUnlockActive()` and selector `0xf8946485`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct EmergencyUnlockActiveReturn(pub bool);
    ///Container type for all return fields from the `epoch` function with signature `epoch()` and selector `0x900cf0cf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct EpochReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `future_smart_wallet_checker` function with signature `future_smart_wallet_checker()` and selector `0x8ff36fd1`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct FutureSmartWalletCheckerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `get_last_user_slope` function with signature `get_last_user_slope(address)` and selector `0x7c74a174`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetLastUserSlopeReturn(pub i128);
    ///Container type for all return fields from the `locked` function with signature `locked(address)` and selector `0xcbf9fe5f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct LockedReturn(pub LockedBalance);
    ///Container type for all return fields from the `locked__end` function with signature `locked__end(address)` and selector `0xadc63589`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct LockedEndReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    ///Container type for all return fields from the `point_history` function with signature `point_history(uint256)` and selector `0xd1febfb9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PointHistoryReturn {
        pub bias: i128,
        pub slope: i128,
        pub ts: ::ethers::core::types::U256,
        pub blk: ::ethers::core::types::U256,
        pub fxs_amt: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `slope_changes` function with signature `slope_changes(uint256)` and selector `0x71197484`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SlopeChangesReturn(pub i128);
    ///Container type for all return fields from the `smart_wallet_checker` function with signature `smart_wallet_checker()` and selector `0x7175d4f7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SmartWalletCheckerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supply` function with signature `supply()` and selector `0x047fc9aa`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
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
    ///Container type for all return fields from the `totalFXSSupply` function with signature `totalFXSSupply()` and selector `0xc3ad8956`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalFXSSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalFXSSupplyAt` function with signature `totalFXSSupplyAt(uint256)` and selector `0x4f8ab24f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalFXSSupplyAtReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply(uint256)` and selector `0xbd85b039`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyWithTReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupplyAt` function with signature `totalSupplyAt(uint256)` and selector `0x981b24d0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyAtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfersEnabled` function with signature `transfersEnabled()` and selector `0xbef97c87`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TransfersEnabledReturn(pub bool);
    ///Container type for all return fields from the `user_point_epoch` function with signature `user_point_epoch(address)` and selector `0x010ae757`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct UserPointEpochReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `user_point_history` function with signature `user_point_history(address,uint256)` and selector `0x28d09d47`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct UserPointHistoryReturn {
        pub bias: i128,
        pub slope: i128,
        pub ts: ::ethers::core::types::U256,
        pub blk: ::ethers::core::types::U256,
        pub fxs_amt: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `user_point_history__ts` function with signature `user_point_history__ts(address,uint256)` and selector `0xda020a18`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct UserPointHistoryTsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn(pub String);
    ///`LockedBalance(int128,uint256)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    pub struct LockedBalance {
        pub amount: i128,
        pub end: ::ethers::core::types::U256,
    }
}
