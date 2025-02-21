// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use parity_db::{Key, Value};

pub trait Db: Send + Sync + 'static {
	type Options;

	fn open(path: &std::path::Path) -> Self;
	fn with_options(options: &Self::Options) -> Self;
	fn get(&self, key: &Key) -> Option<Value>;
	fn commit<I: IntoIterator<Item=(Key, Option<Value>)>>(&self, tx: I);
}
