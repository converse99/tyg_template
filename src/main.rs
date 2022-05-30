// Copyright ⓒ 2022 Porter.
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

fn main() {
    match tyg_template::run() {
        Ok(_) => println!("The process completed normally"),
        Err(e) => eprintln!("tyg_template: {}", e),
    }
}
