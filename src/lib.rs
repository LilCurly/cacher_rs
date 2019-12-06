/**
 * 
 * Cacher by Roman Muzikantov
 * Last update: 06/12/2019
 * 
 * */

use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<'a, C, V, R>
    where C: Fn(&[V]) -> R, R: Clone, V: Eq + Hash
{
    value: HashMap<&'a [V], R>,
    calc: C,
}

impl<'a, C, V, R> Cacher<'a, C, V, R>
    where C: Fn(&[V]) -> R, R: Clone, V: Eq + Hash
{
    pub fn new(calc: C) -> Cacher<'a, C, V, R> {
        Cacher {
            value: HashMap::new(),
            calc,
        }
    }

    pub fn get_value(&mut self, val: &'a [V]) -> R {
        if !self.value.contains_key(val) {
            self.value.insert(val, (self.calc)(val));
        }
        self.value.get(val).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_integers() {
        let mut c = Cacher::new(|val| val[0]);
        assert_eq!(c.get_value(&[1]), 1);
        assert_eq!(c.get_value(&[5]), 5);
    }

    #[test]
    fn works_with_str() {
        let mut c = Cacher::new(|val| val[0]);
        assert_eq!(c.get_value(&["test"]), "test");
        assert_eq!(c.get_value(&["second test"]), "second test");
    }

    #[test]
    fn works_with_different_return_types() {
        let mut c = Cacher::new(|_| "test");
        assert_eq!(c.get_value(&[1]), "test");
    }

    #[test]
    fn works_with_bigger_slice() {
        let mut c = Cacher::new(|val| val[0] + val[1]);
        assert_eq!(c.get_value(&[1, 2]), 3);
    }

    #[test]
    fn works_with_string() {
        let mut c = Cacher::new(|val| String::from(val[0]));
        assert_eq!(c.get_value(&["test"]), String::from("test"));
    }
}
