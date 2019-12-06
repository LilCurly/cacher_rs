use std::marker::PhantomData;

pub struct Cacher<C, V, R>
    where C: Fn(V) -> R, R: Copy
{
    value: Option<R>,
    calc: C,
    phantom: PhantomData<V>,
}

impl<C, V, R> Cacher<C, V, R>
    where C: Fn(V) -> R, R: Copy
{
    pub fn new(calc: C) -> Cacher<C, V, R> {
        Cacher {
            value: None,
            calc,
            phantom: PhantomData
        }
    }

    pub fn get_value(&mut self, val: V) -> R {
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

    #[test]
    fn works_with_different_return_types() {
        let mut c = Cacher::new(|val| "test");
        assert_eq!(c.get_value(1), "test");
    }
}
