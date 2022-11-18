// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_evm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_evm.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_evm::WeightInfo for WeightInfo<T> {
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create() -> Weight {
		(128_068_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create2() -> Weight {
		(123_603_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: EVM NetworkContractIndex (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_nft_contract() -> Weight {
		(141_310_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_predeploy_contract() -> Weight {
		(141_976_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:2 w:1)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:0)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn call() -> Weight {
		(123_136_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn transfer_maintainer() -> Weight {
		(92_080_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_contract() -> Weight {
		(108_438_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_free() -> Weight {
		(19_882_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn enable_contract_development() -> Weight {
		(96_780_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn disable_contract_development() -> Weight {
		(97_383_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM CodeInfos (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: EVM Codes (r:0 w:2)
	fn set_code(c: u32, ) -> Weight {
		(152_012_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts Accounts (r:1 w:0)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: IdleScheduler NextTaskId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: IdleScheduler Tasks (r:0 w:1)
	// Storage: EVM Codes (r:0 w:1)
	fn selfdestruct() -> Weight {
		(117_550_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}
