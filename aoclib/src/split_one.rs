// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Split a string into two parts on the given character.
///
/// Panics if the character is not present.
///
/// ```
/// use aoclib::split_one;
/// assert_eq!(split_one("abc:xyz", ':'), ("abc", "xyz"));
/// assert_eq!(split_one("abc:", ':'), ("abc", ""));
/// assert_eq!(split_one("abc::", ':'), ("abc", ":"));
/// ```
pub fn split_one(s: &str, sep: char) -> (&str, &str) {
    try_split_one(s, sep).unwrap()
}

pub fn try_split_one(s: &str, sep: char) -> Option<(&str, &str)> {
    if let Some(pos) = s
        .char_indices()
        .find_map(|(pos, c)| if c == sep { Some(pos) } else { None })
    {
        let (a, b) = s.split_at(pos);
        let b = b.strip_prefix(sep).unwrap();
        Some((a, b))
    } else {
        None
    }
}
