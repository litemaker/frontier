// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use ethereum_types::{H256, H64, U256};
use jsonrpc_core::Result;

use sc_network::ExHashT;
use sc_transaction_pool::ChainApi;
use sp_runtime::traits::Block as BlockT;

use fc_rpc_core::types::*;

use crate::eth::EthApi;

impl<B: BlockT, C, P, BE, H: ExHashT, A: ChainApi, F> EthApi<B, C, P, BE, H, A, F> {
	pub fn is_mining(&self) -> Result<bool> {
		Ok(self.is_authority)
	}

	pub fn hashrate(&self) -> Result<U256> {
		Ok(U256::zero())
	}

	pub fn work(&self) -> Result<Work> {
		Ok(Work::default())
	}

	pub fn submit_hashrate(&self, _: U256, _: H256) -> Result<bool> {
		Ok(false)
	}

	pub fn submit_work(&self, _: H64, _: H256, _: H256) -> Result<bool> {
		Ok(false)
	}
}
