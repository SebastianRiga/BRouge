/*
 * Copyright (c)  Sebastian Riga 2023.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated
 * documentation files (the “Software”), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies
 * or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
 * PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VarArgs {
    contents: HashMap<String, Box<dyn Any>>,
}

impl VarArgs {
    pub fn new() -> Self {
        Self { contents: HashMap::new() }
    }

    pub fn insert(&mut self, key: &str, value: impl Any) -> &mut Self {
        self.contents.insert(String::from(key), Box::new(value));
        self
    }

    pub fn get<T: Any + Copy>(&self, key: &str, default: T) -> T {
        if self.contents.contains_key(key) {
            if let Some(value) = self.contents.get(key).unwrap().downcast_ref::<T>() {
                return *value;
            }
        }
        default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_access() {
        let mut bundle = VarArgs::new();

        bundle
            .insert("test1", true)
            .insert("test2", 2)
            .insert("test3", 4.5)
            .insert("test4", "test4");

        assert_eq!(true, bundle.get("test1", false));
        assert_eq!(2, bundle.get("test2", 0));
        assert_eq!(4.5, bundle.get("test3", 0.0));
        assert_eq!("test4", bundle.get("test4", ""));
    }
}
