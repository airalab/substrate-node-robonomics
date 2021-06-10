///////////////////////////////////////////////////////////////////////////////
//
//  Copyright 2018-2021 Robonomics Network <research@robonomics.network>
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
///////////////////////////////////////////////////////////////////////////////
//! SubXt compatible robonomics-rws pallet.

use codec::{Codec, Encode, EncodeLike};
use sp_runtime::traits::Extrinsic;
use sp_runtime::traits::Member;
use sp_runtime::OpaqueExtrinsic;
use std::fmt::Debug;
use substrate_subxt::system::System;
use substrate_subxt_proc_macro::{module, Call};

/// The subset of the `pallet_robonomics_rws::Config` that a client must implement.
#[module]
pub trait RWS: System {
    type Call: Codec + EncodeLike + Member + Default;
}

/// Wrap extrinsic call.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RwsCall<T: RWS> {
    call: T::Call,
}
