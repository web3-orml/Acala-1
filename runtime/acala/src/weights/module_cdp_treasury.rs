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

//! Autogenerated weights for module_cdp_treasury
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

/// Weight functions for module_cdp_treasury.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_cdp_treasury::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:1 w:0)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	// Storage: Auction AuctionsIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AuctionManager CollateralAuctions (r:0 w:1)
	// Storage: Auction AuctionEndTime (r:0 w:1)
	// Storage: Auction Auctions (r:0 w:1)
	fn auction_collateral(b: u32, ) -> Weight {
		(32_933_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((11_065_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Tokens Accounts (r:8 w:8)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:5 w:0)
	// Storage: Dex LiquidityPool (r:5 w:1)
	// Storage: StableAsset Pools (r:2 w:1)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: System Account (r:3 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	fn exchange_collateral_to_stable() -> Weight {
		(320_109_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(32 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:0 w:1)
	fn set_expected_collateral_auction_size() -> Weight {
		(13_820_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	fn extract_surplus_to_treasury() -> Weight {
		(38_882_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
