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

use quicli::prelude::*;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "Dynamic Wallpaper", about = "Generate Dynamic Wallpapers that interpolate between each other")]
pub(crate) struct Cli {
    #[structopt(long = "folder", short = "n", parse(from_os_str))]
    folder: PathBuf
}

impl Cli {
    pub(crate) fn get() -> Self {
       Self::from_args()
    }

    pub(crate) fn folder(&self) -> PathBuf {
        self.folder.clone()
    }
}
