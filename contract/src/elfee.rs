pub use el_fee::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod el_fee {
    const _: () = {
        ::core::include_bytes!(
            "../abi/ELFee.abi",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_elFee"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_withdrawalAddr"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimedUserAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimedUserAmount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("elFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("elFee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setELFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setELFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_elFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("splitFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("splitFee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vaultAddr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vaultAddr"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawalAddr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawalAddr"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SplitFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SplitFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("protocolAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("userAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ELFEE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ELFee<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ELFee<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ELFee<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ELFee<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ELFee<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ELFee)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ELFee<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ELFEE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `claimedUserAmount` (0x96f0a422) function
        pub fn claimed_user_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([150, 240, 164, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `elFee` (0x30e739ae) function
        pub fn el_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 231, 57, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setELFee` (0xd6d9c99a) function
        pub fn set_el_fee(
            &self,
            el_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 217, 201, 154], el_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `splitFee` (0x49805555) function
        pub fn split_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 128, 85, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vaultAddr` (0xd27567f2) function
        pub fn vault_addr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 117, 103, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawalAddr` (0xaa6b6e51) function
        pub fn withdrawal_addr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([170, 107, 110, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SplitFee` event
        pub fn split_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SplitFeeFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SplitFeeFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ELFee<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SplitFee", abi = "SplitFee(uint256,uint256)")]
    pub struct SplitFeeFilter {
        pub protocol_amount: ::ethers::core::types::U256,
        pub user_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claimedUserAmount` function with signature `claimedUserAmount()` and selector `0x96f0a422`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claimedUserAmount", abi = "claimedUserAmount()")]
    pub struct ClaimedUserAmountCall;
    ///Container type for all input parameters for the `elFee` function with signature `elFee()` and selector `0x30e739ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "elFee", abi = "elFee()")]
    pub struct ElFeeCall;
    ///Container type for all input parameters for the `setELFee` function with signature `setELFee(uint256)` and selector `0xd6d9c99a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setELFee", abi = "setELFee(uint256)")]
    pub struct SetELFeeCall {
        pub el_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `splitFee` function with signature `splitFee()` and selector `0x49805555`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "splitFee", abi = "splitFee()")]
    pub struct SplitFeeCall;
    ///Container type for all input parameters for the `vaultAddr` function with signature `vaultAddr()` and selector `0xd27567f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vaultAddr", abi = "vaultAddr()")]
    pub struct VaultAddrCall;
    ///Container type for all input parameters for the `withdrawalAddr` function with signature `withdrawalAddr()` and selector `0xaa6b6e51`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawalAddr", abi = "withdrawalAddr()")]
    pub struct WithdrawalAddrCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ELFeeCalls {
        ClaimedUserAmount(ClaimedUserAmountCall),
        ElFee(ElFeeCall),
        SetELFee(SetELFeeCall),
        SplitFee(SplitFeeCall),
        VaultAddr(VaultAddrCall),
        WithdrawalAddr(WithdrawalAddrCall),
    }
    impl ::ethers::core::abi::AbiDecode for ELFeeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ClaimedUserAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ClaimedUserAmount(decoded));
            }
            if let Ok(decoded)
                = <ElFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ElFee(decoded));
            }
            if let Ok(decoded)
                = <SetELFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetELFee(decoded));
            }
            if let Ok(decoded)
                = <SplitFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SplitFee(decoded));
            }
            if let Ok(decoded)
                = <VaultAddrCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VaultAddr(decoded));
            }
            if let Ok(decoded)
                = <WithdrawalAddrCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawalAddr(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ELFeeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimedUserAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ElFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetELFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SplitFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VaultAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ELFeeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimedUserAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ElFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetELFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SplitFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::VaultAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalAddr(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimedUserAmountCall> for ELFeeCalls {
        fn from(value: ClaimedUserAmountCall) -> Self {
            Self::ClaimedUserAmount(value)
        }
    }
    impl ::core::convert::From<ElFeeCall> for ELFeeCalls {
        fn from(value: ElFeeCall) -> Self {
            Self::ElFee(value)
        }
    }
    impl ::core::convert::From<SetELFeeCall> for ELFeeCalls {
        fn from(value: SetELFeeCall) -> Self {
            Self::SetELFee(value)
        }
    }
    impl ::core::convert::From<SplitFeeCall> for ELFeeCalls {
        fn from(value: SplitFeeCall) -> Self {
            Self::SplitFee(value)
        }
    }
    impl ::core::convert::From<VaultAddrCall> for ELFeeCalls {
        fn from(value: VaultAddrCall) -> Self {
            Self::VaultAddr(value)
        }
    }
    impl ::core::convert::From<WithdrawalAddrCall> for ELFeeCalls {
        fn from(value: WithdrawalAddrCall) -> Self {
            Self::WithdrawalAddr(value)
        }
    }
    ///Container type for all return fields from the `claimedUserAmount` function with signature `claimedUserAmount()` and selector `0x96f0a422`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ClaimedUserAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `elFee` function with signature `elFee()` and selector `0x30e739ae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ElFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vaultAddr` function with signature `vaultAddr()` and selector `0xd27567f2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VaultAddrReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `withdrawalAddr` function with signature `withdrawalAddr()` and selector `0xaa6b6e51`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WithdrawalAddrReturn(pub ::ethers::core::types::Address);
}
