/* Copyright (C) 2022 1274210585@qq.com
 *   This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or(at your option) any later version.
 *   This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
 *   You should have received a copy of the GNU General Public License along with this program.
 *   If not, see <https://www.gnu.org/licenses/>.
*/
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let z = s; //废除s变量
    {
        let z = 10;
        println!("{}", z);
    }
    println!("{}", z);
    let s = z.clone(); //克隆z变量
    println!("{}", s);
}
