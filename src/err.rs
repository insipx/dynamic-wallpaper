// Copyright 2017-2019 Andrew I. Plaza
// This file is part of dynamic-wallpaper.

// dynamic-wallpaper is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// dynamic-wallpaper is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with dynamic-wallpaper.  If not, see <http://www.gnu.org/licenses/>.
//! Error Handling

use failure::Fail;
use std::io::Error as IoError;
use std::convert::Infallible;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Io {}", _0)]
    Io(#[fail(cause)] IoError),
    #[fail(display = "Infallible error, this should never happen {}", _0)]
    NeverError(#[fail(cause)] Infallible)
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Error {
        Error::NeverError(err)
    }
}
