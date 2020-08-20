// Copyright 2020 Farzad FARID <farzy@farzy.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Set 1

mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;
mod challenge6;

pub fn main() {
    challenge1::main();
    challenge2::main();
    if let Err(error) = challenge3::main() {
        eprintln!("An error happened: {}", error);
    }
    if let Err(error) = challenge4::main() {
        eprintln!("An error happened: {}", error);
    }
    challenge5::main();
    if let Err(error) = challenge6::main() {
        eprintln!("An error happened: {}", error);
    }
}
