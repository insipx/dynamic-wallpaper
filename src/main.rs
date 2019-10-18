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

use image::GenericImageView;
mod err;
mod image_locator;
mod cli;

use failure::Error;
use cli::Cli;
use image_locator::ImageLocator;

fn main() -> Result<(), Error> {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let config = Cli::get();
    let folder = ImageLocator::new(config.folder())?;

    for image in folder.entries() {
        let img = image::open(image)?;
        println!("===================================");
        println!("dimensions {:?}", img.dimensions());
        println!("{:?}", img.color());
        println!("===================================");
    }
    Ok(())
    // Write the contents of this image to the Writer in PNG format.
    // img.save("test.png").unwrap();
}
