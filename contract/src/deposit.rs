pub use deposit_contract::*;
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
pub mod deposit_contract {
    const _: () = {
        ::core::include_bytes!(
            "../abi/Deposit.abi",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawal_credentials",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deposit_data_root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("get_deposit_count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_deposit_count"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("get_deposit_root"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_deposit_root"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DepositEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DepositEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawal_credentials",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static DEPOSITCONTRACT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0[`\x1F\x81\x10\x15a\x01\x02W`\x02`!\x82` \x81\x10a\0,W\xFE[\x01T`!\x83` \x81\x10a\0;W\xFE[\x01T`@Q` \x01\x80\x83\x81R` \x01\x82\x81R` \x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\0\x92W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\0sV[Q\x81Q` \x93\x84\x03a\x01\0\n`\0\x19\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\0\xD1W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\0\xE6W`\0\x80\xFD[PQ`!`\x01\x83\x01` \x81\x10a\0\xF8W\xFE[\x01U`\x01\x01a\0\x14V[Pa\x18\xD6\x80b\0\x01\x13`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0DW\x80c\"\x89Q\x18\x14a\0\xA4W\x80cb\x1F\xD10\x14a\x01\xBAW\x80c\xC5\xF2\x89/\x14a\x02DW[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[Pa\0\x90`\x04\x806\x03` \x81\x10\x15a\0gW`\0\x80\xFD[P5\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\x02kV[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\xB8`\x04\x806\x03`\x80\x81\x10\x15a\0\xBAW`\0\x80\xFD[\x81\x01\x90` \x81\x01\x815d\x01\0\0\0\0\x81\x11\x15a\0\xD5W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\0\xE7W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x01\tW`\0\x80\xFD[\x91\x93\x90\x92\x90\x91` \x81\x01\x905d\x01\0\0\0\0\x81\x11\x15a\x01'W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x019W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x01[W`\0\x80\xFD[\x91\x93\x90\x92\x90\x91` \x81\x01\x905d\x01\0\0\0\0\x81\x11\x15a\x01yW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x01\x8BW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x01\xADW`\0\x80\xFD[\x91\x93P\x91P5a\x03\x04V[\0[4\x80\x15a\x01\xC6W`\0\x80\xFD[Pa\x01\xCFa\x10\xB5V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x02\tW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xF1V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x026W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02PW`\0\x80\xFD[Pa\x02Ya\x10\xC7V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x02\xFEWP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x85d\t\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x92\x91PPV[`0\x86\x14a\x03]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80a\x18\x05`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[` \x84\x14a\x03\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`6\x81R` \x01\x80a\x17\x9C`6\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``\x82\x14a\x04\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`)\x81R` \x01\x80a\x18x`)\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\x004\x10\x15a\x04pW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80a\x18R`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[c;\x9A\xCA\x004\x06\x15a\x04\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`3\x81R` \x01\x80a\x17\xD2`3\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[c;\x9A\xCA\x004\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x055W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80a\x18+`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``a\x05@\x82a\x14\xBAV[\x90P\x7Fd\x9B\xBCb\xD0\xE3\x13B\xAF\xEAN\\\xD8-@I\xE7\xE1\xEE\x91/\xC0\x88\x9A\xA7\x90\x80;\xE3\x908\xC5\x89\x89\x89\x89\x85\x8A\x8Aa\x05u` Ta\x14\xBAV[`@\x80Q`\xA0\x80\x82R\x81\x01\x89\x90R\x90\x81\x90` \x82\x01\x90\x82\x01``\x83\x01`\x80\x84\x01`\xC0\x85\x01\x8E\x8E\x80\x82\x847`\0\x83\x82\x01R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x91\x01\x87\x81\x03\x86R\x8C\x81R` \x01\x90P\x8C\x8C\x80\x82\x847`\0\x83\x82\x01\x81\x90R`\x1F\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x88\x81\x03\x86R\x8CQ\x81R\x8CQ` \x91\x82\x01\x93\x91\x8E\x01\x92P\x90\x81\x90\x84\x90\x84\x90[\x83\x81\x10\x15a\x06HW\x81\x81\x01Q\x83\x82\x01R` \x01a\x060V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x06uW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x86\x81\x03\x83R\x88\x81R` \x01\x89\x89\x80\x82\x847`\0\x83\x82\x01\x81\x90R`\x1F\x90\x91\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x88\x81\x03\x84R\x89Q\x81R\x89Q` \x91\x82\x01\x93\x91\x8B\x01\x92P\x90\x81\x90\x84\x90\x84\x90[\x83\x81\x10\x15a\x06\xEFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xD7V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x07\x1CW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x9DPPPPPPPPPPPPPP`@Q\x80\x91\x03\x90\xA1`\0`\x02\x8A\x8A`\0`\x80\x1B`@Q` \x01\x80\x84\x84\x80\x82\x847\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x91\x90\x93\x01\x90\x81R`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF0\x81\x84\x03\x01\x81R`\x10\x90\x92\x01\x90\x81\x90R\x81Q\x91\x95P\x93P\x83\x92P` \x85\x01\x91P\x80\x83\x83[` \x83\x10a\x07\xFCW\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x07\xBFV[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x08YW=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x08nW`\0\x80\xFD[PQ\x90P`\0`\x02\x80a\x08\x84`@\x84\x8A\x8Ca\x16\xFEV[`@Q` \x01\x80\x83\x83\x80\x82\x847\x80\x83\x01\x92PPP\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x08\xF8W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x08\xBBV[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\tUW=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\tjW`\0\x80\xFD[PQ`\x02a\t{\x89`@\x81\x8Da\x16\xFEV[`@Q`\0\x90` \x01\x80\x84\x84\x80\x82\x847\x91\x90\x91\x01\x92\x83RPP`@\x80Q\x80\x83\x03\x81R` \x92\x83\x01\x91\x82\x90R\x80Q\x90\x94P\x90\x92P\x82\x91\x84\x01\x90\x80\x83\x83[` \x83\x10a\t\xF4W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t\xB7V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\nQW=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\nfW`\0\x80\xFD[PQ`@\x80Q` \x81\x81\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90\x81\x90R\x81Q\x91\x92\x90\x91\x82\x91\x84\x01\x90\x80\x83\x83[` \x83\x10a\n\xDAW\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\n\x9DV[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0B7W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x0BLW`\0\x80\xFD[PQ`@\x80Q` \x81\x01\x85\x81R\x92\x93P`\0\x92`\x02\x92\x83\x92\x87\x92\x8F\x92\x8F\x92\x01\x83\x83\x80\x82\x847\x80\x83\x01\x92PPP\x93PPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B\xD9W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B\x9CV[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0C6W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x0CKW`\0\x80\xFD[PQ`@Q\x86Q`\x02\x91\x88\x91`\0\x91\x88\x91` \x91\x82\x01\x91\x82\x91\x90\x86\x01\x90\x80\x83\x83[` \x83\x10a\x0C\xA9W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0ClV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x18\x01\x82\x81R` \x01\x93PPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\rNW\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\r\x11V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\r\xABW=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\r\xC0W`\0\x80\xFD[PQ`@\x80Q` \x81\x81\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90\x81\x90R\x81Q\x91\x92\x90\x91\x82\x91\x84\x01\x90\x80\x83\x83[` \x83\x10a\x0E4W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\r\xF7V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0E\x91W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x0E\xA6W`\0\x80\xFD[PQ\x90P\x85\x81\x14a\x0F\x02W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`T\x81R` \x01\x80a\x17H`T\x919``\x01\x91PP`@Q\x80\x91\x03\x90\xFD[` Tc\xFF\xFF\xFF\xFF\x11a\x0F`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80a\x17'`!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[` \x80T`\x01\x01\x90\x81\x90U`\0[` \x81\x10\x15a\x10\xA9W\x81`\x01\x16`\x01\x14\x15a\x0F\xA0W\x82`\0\x82` \x81\x10a\x0F\x91W\xFE[\x01UPa\x10\xAC\x95PPPPPPV[`\x02`\0\x82` \x81\x10a\x0F\xAFW\xFE[\x01T\x84`@Q` \x01\x80\x83\x81R` \x01\x82\x81R` \x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x10%W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0F\xE8V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x10\x82W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x10\x97W`\0\x80\xFD[PQ\x92P`\x02\x82\x04\x91P`\x01\x01a\x0FnV[P\xFE[PPPPPPPV[``a\x10\xC2` Ta\x14\xBAV[\x90P\x90V[` T`\0\x90\x81\x90\x81[` \x81\x10\x15a\x12\xF0W\x81`\x01\x16`\x01\x14\x15a\x11\xE6W`\x02`\0\x82` \x81\x10a\x10\xF5W\xFE[\x01T\x84`@Q` \x01\x80\x83\x81R` \x01\x82\x81R` \x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x11kW\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x11.V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x11\xC8W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x11\xDDW`\0\x80\xFD[PQ\x92Pa\x12\xE2V[`\x02\x83`!\x83` \x81\x10a\x11\xF6W\xFE[\x01T`@Q` \x01\x80\x83\x81R` \x01\x82\x81R` \x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x12kW\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x12.V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x12\xC8W=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x12\xDDW`\0\x80\xFD[PQ\x92P[`\x02\x82\x04\x91P`\x01\x01a\x10\xD1V[P`\x02\x82a\x12\xFF` Ta\x14\xBAV[`\0`@\x1B`@Q` \x01\x80\x84\x81R` \x01\x83\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x13ZW\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x13\x1DV[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x95\x90\x95\x16\x92\x01\x91\x82RP`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF8\x01\x81R`\x18\x90\x92\x01\x90\x81\x90R\x81Q\x91\x95P\x93P\x83\x92\x85\x01\x91P\x80\x83\x83[` \x83\x10a\x14?W\x80Q\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x14\x02V[Q\x81Q` \x93\x84\x03a\x01\0\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R`@Q\x91\x90\x93\x01\x94P\x91\x92PP\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x14\x9CW=`\0\x80>=`\0\xFD[PPP`@Q=` \x81\x10\x15a\x14\xB1W`\0\x80\xFD[PQ\x92PPP\x90V[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P`\xC0\x82\x90\x1B\x80`\x07\x1A`\xF8\x1B\x82`\0\x81Q\x81\x10a\x14\xF4W\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\x06\x1A`\xF8\x1B\x82`\x01\x81Q\x81\x10a\x157W\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\x05\x1A`\xF8\x1B\x82`\x02\x81Q\x81\x10a\x15zW\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\x04\x1A`\xF8\x1B\x82`\x03\x81Q\x81\x10a\x15\xBDW\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\x03\x1A`\xF8\x1B\x82`\x04\x81Q\x81\x10a\x16\0W\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\x02\x1A`\xF8\x1B\x82`\x05\x81Q\x81\x10a\x16CW\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\x01\x1A`\xF8\x1B\x82`\x06\x81Q\x81\x10a\x16\x86W\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80`\0\x1A`\xF8\x1B\x82`\x07\x81Q\x81\x10a\x16\xC9W\xFE[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPP\x91\x90PV[`\0\x80\x85\x85\x11\x15a\x17\rW\x81\x82\xFD[\x83\x86\x11\x15a\x17\x19W\x81\x82\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV\xFEDepositContract: merkle tree fullDepositContract: reconstructed DepositData does not match supplied deposit_data_rootDepositContract: invalid withdrawal_credentials lengthDepositContract: deposit value not multiple of gweiDepositContract: invalid pubkey lengthDepositContract: deposit value too highDepositContract: deposit value too lowDepositContract: invalid signature length\xA2dipfsX\"\x12 \xDC\xEC\xA8pk)\xE9\x17\xDA\xCF%\xFC\xEE\xF9Z\xCA\xC8\xD9\rvZ\xC9&f<\xE4\ta\x95\x95+adsolcC\0\x06\x0B\x003";
    /// The bytecode of the contract.
    pub static DEPOSITCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    pub struct DepositContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DepositContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DepositContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DepositContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DepositContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DepositContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DepositContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPOSITCONTRACT_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                DEPOSITCONTRACT_ABI.clone(),
                DEPOSITCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deposit` (0x22895118) function
        pub fn deposit(
            &self,
            pubkey: ::ethers::core::types::Bytes,
            withdrawal_credentials: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
            deposit_data_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 137, 81, 24],
                    (pubkey, withdrawal_credentials, signature, deposit_data_root),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_deposit_count` (0x621fd130) function
        pub fn get_deposit_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([98, 31, 209, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_deposit_root` (0xc5f2892f) function
        pub fn get_deposit_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([197, 242, 137, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DepositEvent` event
        pub fn deposit_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEventFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEventFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DepositContract<M> {
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
    #[ethevent(
        name = "DepositEvent",
        abi = "DepositEvent(bytes,bytes,bytes,bytes,bytes)"
    )]
    pub struct DepositEventFilter {
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub amount: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub index: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(bytes,bytes,bytes,bytes32)` and selector `0x22895118`
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
    #[ethcall(name = "deposit", abi = "deposit(bytes,bytes,bytes,bytes32)")]
    pub struct DepositCall {
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub deposit_data_root: [u8; 32],
    }
    ///Container type for all input parameters for the `get_deposit_count` function with signature `get_deposit_count()` and selector `0x621fd130`
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
    #[ethcall(name = "get_deposit_count", abi = "get_deposit_count()")]
    pub struct GetDepositCountCall;
    ///Container type for all input parameters for the `get_deposit_root` function with signature `get_deposit_root()` and selector `0xc5f2892f`
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
    #[ethcall(name = "get_deposit_root", abi = "get_deposit_root()")]
    pub struct GetDepositRootCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
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
    pub enum DepositContractCalls {
        Deposit(DepositCall),
        GetDepositCount(GetDepositCountCall),
        GetDepositRoot(GetDepositRootCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for DepositContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <GetDepositCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDepositCount(decoded));
            }
            if let Ok(decoded)
                = <GetDepositRootCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDepositRoot(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DepositContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDepositRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DepositContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for DepositContractCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetDepositCountCall> for DepositContractCalls {
        fn from(value: GetDepositCountCall) -> Self {
            Self::GetDepositCount(value)
        }
    }
    impl ::core::convert::From<GetDepositRootCall> for DepositContractCalls {
        fn from(value: GetDepositRootCall) -> Self {
            Self::GetDepositRoot(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DepositContractCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `get_deposit_count` function with signature `get_deposit_count()` and selector `0x621fd130`
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
    pub struct GetDepositCountReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `get_deposit_root` function with signature `get_deposit_root()` and selector `0xc5f2892f`
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
    pub struct GetDepositRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
