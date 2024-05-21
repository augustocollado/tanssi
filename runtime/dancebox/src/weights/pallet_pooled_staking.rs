// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_pooled_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_pooled_staking
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/dancebox_weights/pallet_pooled_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::ParityDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_pooled_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pooled_staking::WeightInfo for SubstrateWeight<T> {
	/// Storage: `PooledStaking::Pools` (r:12 w:5)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::PendingOperations` (r:1 w:1)
	/// Proof: `PooledStaking::PendingOperations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1465`
		//  Estimated: `32155`
		// Minimum execution time: 197_066_000 picoseconds.
		Weight::from_parts(200_174_000, 32155)
			.saturating_add(T::DbWeight::get().reads(18_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: `PooledStaking::PendingOperations` (r:100 w:100)
	/// Proof: `PooledStaking::PendingOperations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::Pools` (r:1000 w:800)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn execute_pending_operations(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545 + b * (390 ±0)`
		//  Estimated: `3694 + b * (25141 ±0)`
		// Minimum execution time: 132_680_000 picoseconds.
		Weight::from_parts(25_732_047, 3694)
			// Standard Error: 33_922
			.saturating_add(Weight::from_parts(100_199_696, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((11_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((9_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 25141).saturating_mul(b.into()))
	}
	/// Storage: `PooledStaking::Pools` (r:13 w:9)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::PendingOperations` (r:1 w:1)
	/// Proof: `PooledStaking::PendingOperations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_undelegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `725`
		//  Estimated: `33890`
		// Minimum execution time: 149_789_000 picoseconds.
		Weight::from_parts(151_920_000, 33890)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: `PooledStaking::Pools` (r:300 w:100)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn claim_manual_rewards(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398 + b * (456 ±0)`
		//  Estimated: `6196 + b * (7882 ±0)`
		// Minimum execution time: 84_188_000 picoseconds.
		Weight::from_parts(37_369_391, 6196)
			// Standard Error: 16_378
			.saturating_add(Weight::from_parts(53_969_787, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7882).saturating_mul(b.into()))
	}
	/// Storage: `PooledStaking::Pools` (r:4 w:1)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn rebalance_hold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1019`
		//  Estimated: `11909`
		// Minimum execution time: 122_325_000 picoseconds.
		Weight::from_parts(124_469_000, 11909)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `PooledStaking::Pools` (r:600 w:100)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::NextKeys` (r:100 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 100]`.
	fn update_candidate_position(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `400 + b * (356 ±0)`
		//  Estimated: `1882 + b * (15206 ±0)`
		// Minimum execution time: 52_761_000 picoseconds.
		Weight::from_parts(53_377_000, 1882)
			// Standard Error: 25_407
			.saturating_add(Weight::from_parts(39_517_239, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 15206).saturating_mul(b.into()))
	}
	/// Storage: `PooledStaking::Pools` (r:12 w:8)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn swap_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `31168`
		// Minimum execution time: 110_457_000 picoseconds.
		Weight::from_parts(111_430_000, 31168)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `PooledStaking::Pools` (r:9 w:5)
	/// Proof: `PooledStaking::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PooledStaking::SortedEligibleCandidates` (r:1 w:1)
	/// Proof: `PooledStaking::SortedEligibleCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn distribute_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1341`
		//  Estimated: `24606`
		// Minimum execution time: 190_002_000 picoseconds.
		Weight::from_parts(191_745_000, 24606)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}