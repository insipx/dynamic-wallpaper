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
//! Locates images that are placed in a directory

use std::{
    fs::{self, ReadDir},
    path::PathBuf,
    str::FromStr
       
};
use crate::err::Error;

pub struct ImageLocator {
    folder_path: PathBuf,
    entries: Vec<PathBuf>
}

impl ImageLocator {

    pub fn new(folder_path: PathBuf) -> Result<Self, Error> {
        let entries = fs::read_dir(folder_path.to_str().expect("Path should be infallible; qed"))?;
        let entries = entries
            .map(|e| {
                e.expect("Paths are infallible").path()
            })
            .collect::<Vec<PathBuf>>();
        Ok(Self {
            folder_path: folder_path,
            entries,
        })
    }

    pub fn entries(&self) -> std::slice::Iter<PathBuf> {
        self.entries.iter()
    }

}
