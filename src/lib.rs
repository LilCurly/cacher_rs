

pub struct Cacher<C>
    where C: Fn(u32) -> u32
{
    value: Option<u32>,
    calc: C
}

impl<C> Cacher<C>
    where C: Fn(u32) -> u32
{
    pub fn new(calc: C) -> Cacher<C> {
        Cacher {
            value: None,
            calc,
        }
    }

    pub fn get_value(&mut self, val: u32) -> u32 {
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
    fn it_works() {
        let mut c = Cacher::new(|val| val);
        assert_eq!(c.get_value(1), 1);
    }
}
