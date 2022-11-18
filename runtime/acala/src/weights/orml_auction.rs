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

//! Autogenerated weights for orml_auction
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

/// Weight functions for orml_auction.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_auction::WeightInfo for WeightInfo<T> {
	// Storage: Auction Auctions (r:1 w:1)
	// Storage: AuctionManager CollateralAuctions (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: Auction AuctionEndTime (r:0 w:2)
	fn bid_collateral_auction() -> Weight {
		(84_652_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Auction AuctionEndTime (r:2 w:1)
	// Storage: Auction Auctions (r:1 w:1)
	// Storage: AuctionManager CollateralAuctions (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:1)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	fn on_finalize(c: u32, ) -> Weight {
		(24_165_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((47_057_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(c as Weight)))
	}
}
