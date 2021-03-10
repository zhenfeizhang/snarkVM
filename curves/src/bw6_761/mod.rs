// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(nightly, doc(include = "../../documentation/the_aleo_curves/04_bw6-761.md"))]

pub mod fr;
#[doc(inline)]
pub use fr::*;

pub mod fq;
#[doc(inline)]
pub use fq::*;

pub mod fq3;
#[doc(inline)]
pub use fq3::*;

pub mod fq6;
#[doc(inline)]
pub use fq6::*;

pub mod g1;
#[doc(inline)]
pub use g1::*;

pub mod g2;
#[doc(inline)]
pub use g2::*;

pub mod parameters;
#[doc(inline)]
pub use parameters::*;

#[cfg(test)]
mod tests;