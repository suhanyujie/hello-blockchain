use bincode;
use serde::{Serialize, Deserialize};

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let se = bincode::serialize(value).unwrap();
    se
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let de = bincode::deserialize(bytes).unwrap();
    de
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn test_se() {
        let p1 = Point{x: 1, y: 2};
        let se = my_serialize(&p1);
        let de: Point = my_deserialize(&se);
        dbg!(&de);
        assert!(de.x == (&p1).x);
    }
}
