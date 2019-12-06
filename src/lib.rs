

pub struct Cacher<C, V>
    where C: Fn(V) -> V, V: Copy
{
    value: Option<V>,
    calc: C
}

impl<C, V> Cacher<C, V>
    where C: Fn(V) -> V, V: Copy
{
    pub fn new(calc: C) -> Cacher<C, V> {
        Cacher {
            value: None,
            calc,
        }
    }

    pub fn get_value(&mut self, val: V) -> V {
        match self.value {
            Some(val) => val,
            None => {
                self.value = Some((self.calc)(val));
                self.value.unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_integers() {
        let mut c = Cacher::new(|val| val);
        assert_eq!(c.get_value(1), 1);
        assert_eq!(c.get_value(5), 1);
    }

    #[test]
    fn works_with_str() {
        let mut c = Cacher::new(|val| val);
        assert_eq!(c.get_value("test"), "test");
        assert_eq!(c.get_value("fsfsf"), "test");
    }
}
