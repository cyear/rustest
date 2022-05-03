/* Copyright (C) 2022 1274210585@qq.com
 *   This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or(at your option) any later version.
 *   This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
 *   You should have received a copy of the GNU General Public License along with this program.  
 *   If not, see <https://www.gnu.org/licenses/>.
*/
fn main() {
    println!("hello");
    let mut num = 3;
    println!("number: {}", num);
    num = {
        num + 1
    };
    println!("number: {}", num);
    let mut n = 1;
    'one: loop {
        loop {
            num += 1;
            if num > 50 {
                break;
            }
        }
            n += 1;
            if n > 10 {
                break 'one;
        }
    };
    println!("number: {}", num);
}
