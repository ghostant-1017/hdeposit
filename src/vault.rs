pub use vault::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod vault {
    const _: () = {
        ::core::include_bytes!("../abi/Vault.abi",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_depositor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_feeCollector"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_depositFee"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_elFee"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_ethDepositContract"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("collectFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collectFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectedDepositFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collectedDepositFee",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pubkeys"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signatures"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("depositDataRoots"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalCredentials",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ns"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositor"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("elFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("elFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ethDepositContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ethDepositContract"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeCollector"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeCollector"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onSplitFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onSplitFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("paused"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("preDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("preDeposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("n"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalCredential",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("createELFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDepositFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDepositFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_depositFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDepositor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setDepositor"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_depositor"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setELFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setELFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_elFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFeeCollector"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeCollector"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_feeCollector"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setWithdrawalELFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setWithdrawalELFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawal"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_elFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("toDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("toDeposit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalDepositFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalDepositFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSplitFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSplitFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawalAddr2FeeContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawalAddr2FeeContract",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawalELFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawalELFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CollectFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CollectFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("collector"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pubkeys"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("signatures"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("depositDataRoots"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnSplitFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OnSplitFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Paused"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PreDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PreDeposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("n"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("createELFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalCredential",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("elFeeContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetDepositFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetDepositFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("depositFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetDepositor"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetDepositor"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("depositor"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetELFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetELFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("elFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFeeCollector"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetFeeCollector"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("feeCollector"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetWithdrawalELFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetWithdrawalELFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("elFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct Vault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Vault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Vault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Vault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Vault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Vault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Vault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VAULT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `collectFee` (0xd4d5d32a) function
        pub fn collect_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 213, 211, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectedDepositFee` (0x4dad525b) function
        pub fn collected_deposit_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 173, 82, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x431977cf) function
        pub fn deposit(
            &self,
            pubkeys: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
            deposit_data_roots: ::std::vec::Vec<[u8; 32]>,
            withdrawal_credentials: ::ethers::core::types::Bytes,
            ns: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [67, 25, 119, 207],
                    (
                        pubkeys,
                        signatures,
                        deposit_data_roots,
                        withdrawal_credentials,
                        ns,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositFee` (0x67a52793) function
        pub fn deposit_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 165, 39, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositor` (0xc7c4ff46) function
        pub fn depositor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([199, 196, 255, 70], ())
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
        ///Calls the contract's `ethDepositContract` (0x3884545d) function
        pub fn eth_deposit_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([56, 132, 84, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeCollector` (0xc415b95c) function
        pub fn fee_collector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 21, 185, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onSplitFee` (0x2252f281) function
        pub fn on_split_fee(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 82, 242, 129], ())
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
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `preDeposit` (0x895b4d8f) function
        pub fn pre_deposit(
            &self,
            n: ::ethers::core::types::U256,
            withdrawal_credential: ::ethers::core::types::Bytes,
            create_el_fee: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [137, 91, 77, 143],
                    (n, withdrawal_credential, create_el_fee),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositFee` (0x490ae210) function
        pub fn set_deposit_fee(
            &self,
            deposit_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 10, 226, 16], deposit_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositor` (0xf2c098b7) function
        pub fn set_depositor(
            &self,
            depositor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 192, 152, 183], depositor)
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
        ///Calls the contract's `setFeeCollector` (0xa42dce80) function
        pub fn set_fee_collector(
            &self,
            fee_collector: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 45, 206, 128], fee_collector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawalELFee` (0x226c4301) function
        pub fn set_withdrawal_el_fee(
            &self,
            withdrawal: ::ethers::core::types::Address,
            el_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 108, 67, 1], (withdrawal, el_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toDeposit` (0x1c858920) function
        pub fn to_deposit(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([28, 133, 137, 32], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalDepositFee` (0x23ecd50e) function
        pub fn total_deposit_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([35, 236, 213, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSplitFee` (0xdf67faa8) function
        pub fn total_split_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([223, 103, 250, 168], ())
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
        ///Calls the contract's `withdrawalAddr2FeeContract` (0xc36099e4) function
        pub fn withdrawal_addr_2_fee_contract(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([195, 96, 153, 228], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawalELFee` (0x3bc115b7) function
        pub fn withdrawal_el_fee(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 193, 21, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CollectFee` event
        pub fn collect_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CollectFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `OnSplitFee` event
        pub fn on_split_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OnSplitFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PreDeposit` event
        pub fn pre_deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PreDepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetDepositFee` event
        pub fn set_deposit_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetDepositFeeFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetDepositor` event
        pub fn set_depositor_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetDepositorFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetELFee` event
        pub fn set_el_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetELFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetFeeCollector` event
        pub fn set_fee_collector_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetFeeCollectorFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetWithdrawalELFee` event
        pub fn set_withdrawal_el_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetWithdrawalELFeeFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VaultEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Vault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "CollectFee", abi = "CollectFee(address,uint256)")]
    pub struct CollectFeeFilter {
        pub collector: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(bytes,bytes,bytes32[])")]
    pub struct DepositFilter {
        pub pubkeys: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
        pub deposit_data_roots: ::std::vec::Vec<[u8; 32]>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OnSplitFee", abi = "OnSplitFee(address,uint256)")]
    pub struct OnSplitFeeFilter {
        pub fee_contract: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "PreDeposit",
        abi = "PreDeposit(address,uint256,bool,bytes,address)"
    )]
    pub struct PreDepositFilter {
        pub sender: ::ethers::core::types::Address,
        pub n: ::ethers::core::types::U256,
        pub create_el_fee: bool,
        pub withdrawal_credential: ::ethers::core::types::Bytes,
        pub el_fee_contract: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetDepositFee", abi = "SetDepositFee(uint256)")]
    pub struct SetDepositFeeFilter {
        pub deposit_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetDepositor", abi = "SetDepositor(address)")]
    pub struct SetDepositorFilter {
        pub depositor: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetELFee", abi = "SetELFee(uint256)")]
    pub struct SetELFeeFilter {
        pub el_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetFeeCollector", abi = "SetFeeCollector(address)")]
    pub struct SetFeeCollectorFilter {
        pub fee_collector: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "SetWithdrawalELFee",
        abi = "SetWithdrawalELFee(address,uint256)"
    )]
    pub struct SetWithdrawalELFeeFilter {
        pub user: ::ethers::core::types::Address,
        pub el_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VaultEvents {
        CollectFeeFilter(CollectFeeFilter),
        DepositFilter(DepositFilter),
        OnSplitFeeFilter(OnSplitFeeFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PreDepositFilter(PreDepositFilter),
        SetDepositFeeFilter(SetDepositFeeFilter),
        SetDepositorFilter(SetDepositorFilter),
        SetELFeeFilter(SetELFeeFilter),
        SetFeeCollectorFilter(SetFeeCollectorFilter),
        SetWithdrawalELFeeFilter(SetWithdrawalELFeeFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for VaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CollectFeeFilter::decode_log(log) {
                return Ok(VaultEvents::CollectFeeFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(VaultEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = OnSplitFeeFilter::decode_log(log) {
                return Ok(VaultEvents::OnSplitFeeFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VaultEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(VaultEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PreDepositFilter::decode_log(log) {
                return Ok(VaultEvents::PreDepositFilter(decoded));
            }
            if let Ok(decoded) = SetDepositFeeFilter::decode_log(log) {
                return Ok(VaultEvents::SetDepositFeeFilter(decoded));
            }
            if let Ok(decoded) = SetDepositorFilter::decode_log(log) {
                return Ok(VaultEvents::SetDepositorFilter(decoded));
            }
            if let Ok(decoded) = SetELFeeFilter::decode_log(log) {
                return Ok(VaultEvents::SetELFeeFilter(decoded));
            }
            if let Ok(decoded) = SetFeeCollectorFilter::decode_log(log) {
                return Ok(VaultEvents::SetFeeCollectorFilter(decoded));
            }
            if let Ok(decoded) = SetWithdrawalELFeeFilter::decode_log(log) {
                return Ok(VaultEvents::SetWithdrawalELFeeFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(VaultEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnSplitFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreDepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetELFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeCollectorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawalELFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CollectFeeFilter> for VaultEvents {
        fn from(value: CollectFeeFilter) -> Self {
            Self::CollectFeeFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for VaultEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<OnSplitFeeFilter> for VaultEvents {
        fn from(value: OnSplitFeeFilter) -> Self {
            Self::OnSplitFeeFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for VaultEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for VaultEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PreDepositFilter> for VaultEvents {
        fn from(value: PreDepositFilter) -> Self {
            Self::PreDepositFilter(value)
        }
    }
    impl ::core::convert::From<SetDepositFeeFilter> for VaultEvents {
        fn from(value: SetDepositFeeFilter) -> Self {
            Self::SetDepositFeeFilter(value)
        }
    }
    impl ::core::convert::From<SetDepositorFilter> for VaultEvents {
        fn from(value: SetDepositorFilter) -> Self {
            Self::SetDepositorFilter(value)
        }
    }
    impl ::core::convert::From<SetELFeeFilter> for VaultEvents {
        fn from(value: SetELFeeFilter) -> Self {
            Self::SetELFeeFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeCollectorFilter> for VaultEvents {
        fn from(value: SetFeeCollectorFilter) -> Self {
            Self::SetFeeCollectorFilter(value)
        }
    }
    impl ::core::convert::From<SetWithdrawalELFeeFilter> for VaultEvents {
        fn from(value: SetWithdrawalELFeeFilter) -> Self {
            Self::SetWithdrawalELFeeFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for VaultEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `collectFee` function with signature `collectFee()` and selector `0xd4d5d32a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "collectFee", abi = "collectFee()")]
    pub struct CollectFeeCall;
    ///Container type for all input parameters for the `collectedDepositFee` function with signature `collectedDepositFee()` and selector `0x4dad525b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "collectedDepositFee", abi = "collectedDepositFee()")]
    pub struct CollectedDepositFeeCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit(bytes,bytes,bytes32[],bytes,uint32[])` and selector `0x431977cf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit(bytes,bytes,bytes32[],bytes,uint32[])"
    )]
    pub struct DepositCall {
        pub pubkeys: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
        pub deposit_data_roots: ::std::vec::Vec<[u8; 32]>,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub ns: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `depositFee` function with signature `depositFee()` and selector `0x67a52793`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "depositFee", abi = "depositFee()")]
    pub struct DepositFeeCall;
    ///Container type for all input parameters for the `depositor` function with signature `depositor()` and selector `0xc7c4ff46`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "depositor", abi = "depositor()")]
    pub struct DepositorCall;
    ///Container type for all input parameters for the `elFee` function with signature `elFee()` and selector `0x30e739ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "elFee", abi = "elFee()")]
    pub struct ElFeeCall;
    ///Container type for all input parameters for the `ethDepositContract` function with signature `ethDepositContract()` and selector `0x3884545d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ethDepositContract", abi = "ethDepositContract()")]
    pub struct EthDepositContractCall;
    ///Container type for all input parameters for the `feeCollector` function with signature `feeCollector()` and selector `0xc415b95c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeCollector", abi = "feeCollector()")]
    pub struct FeeCollectorCall;
    ///Container type for all input parameters for the `onSplitFee` function with signature `onSplitFee()` and selector `0x2252f281`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "onSplitFee", abi = "onSplitFee()")]
    pub struct OnSplitFeeCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `preDeposit` function with signature `preDeposit(uint256,bytes,bool)` and selector `0x895b4d8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "preDeposit", abi = "preDeposit(uint256,bytes,bool)")]
    pub struct PreDepositCall {
        pub n: ::ethers::core::types::U256,
        pub withdrawal_credential: ::ethers::core::types::Bytes,
        pub create_el_fee: bool,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setDepositFee` function with signature `setDepositFee(uint256)` and selector `0x490ae210`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setDepositFee", abi = "setDepositFee(uint256)")]
    pub struct SetDepositFeeCall {
        pub deposit_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setDepositor` function with signature `setDepositor(address)` and selector `0xf2c098b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setDepositor", abi = "setDepositor(address)")]
    pub struct SetDepositorCall {
        pub depositor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setELFee` function with signature `setELFee(uint256)` and selector `0xd6d9c99a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setELFee", abi = "setELFee(uint256)")]
    pub struct SetELFeeCall {
        pub el_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFeeCollector` function with signature `setFeeCollector(address)` and selector `0xa42dce80`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFeeCollector", abi = "setFeeCollector(address)")]
    pub struct SetFeeCollectorCall {
        pub fee_collector: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setWithdrawalELFee` function with signature `setWithdrawalELFee(address,uint256)` and selector `0x226c4301`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setWithdrawalELFee",
        abi = "setWithdrawalELFee(address,uint256)"
    )]
    pub struct SetWithdrawalELFeeCall {
        pub withdrawal: ::ethers::core::types::Address,
        pub el_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `toDeposit` function with signature `toDeposit(bytes)` and selector `0x1c858920`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "toDeposit", abi = "toDeposit(bytes)")]
    pub struct ToDepositCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `totalDepositFee` function with signature `totalDepositFee()` and selector `0x23ecd50e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalDepositFee", abi = "totalDepositFee()")]
    pub struct TotalDepositFeeCall;
    ///Container type for all input parameters for the `totalSplitFee` function with signature `totalSplitFee()` and selector `0xdf67faa8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalSplitFee", abi = "totalSplitFee()")]
    pub struct TotalSplitFeeCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawalAddr2FeeContract` function with signature `withdrawalAddr2FeeContract(address)` and selector `0xc36099e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdrawalAddr2FeeContract",
        abi = "withdrawalAddr2FeeContract(address)"
    )]
    pub struct WithdrawalAddr2FeeContractCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdrawalELFee` function with signature `withdrawalELFee(address)` and selector `0x3bc115b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawalELFee", abi = "withdrawalELFee(address)")]
    pub struct WithdrawalELFeeCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VaultCalls {
        CollectFee(CollectFeeCall),
        CollectedDepositFee(CollectedDepositFeeCall),
        Deposit(DepositCall),
        DepositFee(DepositFeeCall),
        Depositor(DepositorCall),
        ElFee(ElFeeCall),
        EthDepositContract(EthDepositContractCall),
        FeeCollector(FeeCollectorCall),
        OnSplitFee(OnSplitFeeCall),
        Owner(OwnerCall),
        Paused(PausedCall),
        PreDeposit(PreDepositCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDepositFee(SetDepositFeeCall),
        SetDepositor(SetDepositorCall),
        SetELFee(SetELFeeCall),
        SetFeeCollector(SetFeeCollectorCall),
        SetWithdrawalELFee(SetWithdrawalELFeeCall),
        ToDeposit(ToDepositCall),
        TotalDepositFee(TotalDepositFeeCall),
        TotalSplitFee(TotalSplitFeeCall),
        TransferOwnership(TransferOwnershipCall),
        WithdrawalAddr2FeeContract(WithdrawalAddr2FeeContractCall),
        WithdrawalELFee(WithdrawalELFeeCall),
    }
    impl ::ethers::core::abi::AbiDecode for VaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CollectFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectFee(decoded));
            }
            if let Ok(decoded) =
                <CollectedDepositFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CollectedDepositFee(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DepositFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositFee(decoded));
            }
            if let Ok(decoded) = <DepositorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Depositor(decoded));
            }
            if let Ok(decoded) = <ElFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ElFee(decoded));
            }
            if let Ok(decoded) =
                <EthDepositContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EthDepositContract(decoded));
            }
            if let Ok(decoded) = <FeeCollectorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeCollector(decoded));
            }
            if let Ok(decoded) = <OnSplitFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnSplitFee(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PreDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PreDeposit(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetDepositFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetDepositFee(decoded));
            }
            if let Ok(decoded) = <SetDepositorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetDepositor(decoded));
            }
            if let Ok(decoded) = <SetELFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetELFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeCollectorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeCollector(decoded));
            }
            if let Ok(decoded) =
                <SetWithdrawalELFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetWithdrawalELFee(decoded));
            }
            if let Ok(decoded) = <ToDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ToDeposit(decoded));
            }
            if let Ok(decoded) =
                <TotalDepositFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalDepositFee(decoded));
            }
            if let Ok(decoded) = <TotalSplitFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalSplitFee(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalAddr2FeeContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawalAddr2FeeContract(decoded));
            }
            if let Ok(decoded) =
                <WithdrawalELFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawalELFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollectedDepositFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Depositor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ElFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EthDepositContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeCollector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnSplitFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDepositFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDepositor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetELFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeCollector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetWithdrawalELFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalDepositFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSplitFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawalAddr2FeeContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalELFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for VaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectedDepositFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Depositor(element) => ::core::fmt::Display::fmt(element, f),
                Self::ElFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::EthDepositContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeCollector(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnSplitFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetELFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeCollector(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawalELFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalDepositFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSplitFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalAddr2FeeContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalELFee(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CollectFeeCall> for VaultCalls {
        fn from(value: CollectFeeCall) -> Self {
            Self::CollectFee(value)
        }
    }
    impl ::core::convert::From<CollectedDepositFeeCall> for VaultCalls {
        fn from(value: CollectedDepositFeeCall) -> Self {
            Self::CollectedDepositFee(value)
        }
    }
    impl ::core::convert::From<DepositCall> for VaultCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositFeeCall> for VaultCalls {
        fn from(value: DepositFeeCall) -> Self {
            Self::DepositFee(value)
        }
    }
    impl ::core::convert::From<DepositorCall> for VaultCalls {
        fn from(value: DepositorCall) -> Self {
            Self::Depositor(value)
        }
    }
    impl ::core::convert::From<ElFeeCall> for VaultCalls {
        fn from(value: ElFeeCall) -> Self {
            Self::ElFee(value)
        }
    }
    impl ::core::convert::From<EthDepositContractCall> for VaultCalls {
        fn from(value: EthDepositContractCall) -> Self {
            Self::EthDepositContract(value)
        }
    }
    impl ::core::convert::From<FeeCollectorCall> for VaultCalls {
        fn from(value: FeeCollectorCall) -> Self {
            Self::FeeCollector(value)
        }
    }
    impl ::core::convert::From<OnSplitFeeCall> for VaultCalls {
        fn from(value: OnSplitFeeCall) -> Self {
            Self::OnSplitFee(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VaultCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PausedCall> for VaultCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PreDepositCall> for VaultCalls {
        fn from(value: PreDepositCall) -> Self {
            Self::PreDeposit(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VaultCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetDepositFeeCall> for VaultCalls {
        fn from(value: SetDepositFeeCall) -> Self {
            Self::SetDepositFee(value)
        }
    }
    impl ::core::convert::From<SetDepositorCall> for VaultCalls {
        fn from(value: SetDepositorCall) -> Self {
            Self::SetDepositor(value)
        }
    }
    impl ::core::convert::From<SetELFeeCall> for VaultCalls {
        fn from(value: SetELFeeCall) -> Self {
            Self::SetELFee(value)
        }
    }
    impl ::core::convert::From<SetFeeCollectorCall> for VaultCalls {
        fn from(value: SetFeeCollectorCall) -> Self {
            Self::SetFeeCollector(value)
        }
    }
    impl ::core::convert::From<SetWithdrawalELFeeCall> for VaultCalls {
        fn from(value: SetWithdrawalELFeeCall) -> Self {
            Self::SetWithdrawalELFee(value)
        }
    }
    impl ::core::convert::From<ToDepositCall> for VaultCalls {
        fn from(value: ToDepositCall) -> Self {
            Self::ToDeposit(value)
        }
    }
    impl ::core::convert::From<TotalDepositFeeCall> for VaultCalls {
        fn from(value: TotalDepositFeeCall) -> Self {
            Self::TotalDepositFee(value)
        }
    }
    impl ::core::convert::From<TotalSplitFeeCall> for VaultCalls {
        fn from(value: TotalSplitFeeCall) -> Self {
            Self::TotalSplitFee(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VaultCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawalAddr2FeeContractCall> for VaultCalls {
        fn from(value: WithdrawalAddr2FeeContractCall) -> Self {
            Self::WithdrawalAddr2FeeContract(value)
        }
    }
    impl ::core::convert::From<WithdrawalELFeeCall> for VaultCalls {
        fn from(value: WithdrawalELFeeCall) -> Self {
            Self::WithdrawalELFee(value)
        }
    }
    ///Container type for all return fields from the `collectedDepositFee` function with signature `collectedDepositFee()` and selector `0x4dad525b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CollectedDepositFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `depositFee` function with signature `depositFee()` and selector `0x67a52793`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `depositor` function with signature `depositor()` and selector `0xc7c4ff46`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `elFee` function with signature `elFee()` and selector `0x30e739ae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ElFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ethDepositContract` function with signature `ethDepositContract()` and selector `0x3884545d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EthDepositContractReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeCollector` function with signature `feeCollector()` and selector `0xc415b95c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeCollectorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `toDeposit` function with signature `toDeposit(bytes)` and selector `0x1c858920`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ToDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalDepositFee` function with signature `totalDepositFee()` and selector `0x23ecd50e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalDepositFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSplitFee` function with signature `totalSplitFee()` and selector `0xdf67faa8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalSplitFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdrawalAddr2FeeContract` function with signature `withdrawalAddr2FeeContract(address)` and selector `0xc36099e4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawalAddr2FeeContractReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `withdrawalELFee` function with signature `withdrawalELFee(address)` and selector `0x3bc115b7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawalELFeeReturn(pub ::ethers::core::types::U256);
}
