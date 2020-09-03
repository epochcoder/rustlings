// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryFrom, TryInto};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// try to index color :)
impl Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => panic!("Cannot index color!")
        }
    }
}

impl IndexMut<usize> for Color {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            _ => panic!("Cannot index color!")
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}

impl Color {

    fn validate_component(component: &i16) -> Result<(), String> {
        if *component >= 0 && *component <= 255 {
            return Ok(());
        }

        return Err(format!("Could not validate component: {}", *component));
    }
}

// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need create implementation for a tuple of three integer,
// an array of three integer and slice of integer.
//
// Note, that implementation for tuple and array will be checked at compile-time,
// but slice implementation need check slice length!
// Also note, that chunk of correct rgb color must be integer in range 0..=255.

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        Self::validate_component(&tuple.0)?;
        Self::validate_component(&tuple.1)?;
        Self::validate_component(&tuple.2)?;

        Ok(Color {
            red: tuple.0 as u8,
            green: tuple.1 as u8,
            blue: tuple.2 as u8
        })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        if arr.len() != 3 {
            return Err("Should be 3 elements long".to_string());
        }

        let mut color = Color::default();
        for i in 0..3 {
            Self::validate_component(&arr[i])?;
            color[i] = *&arr[i] as u8;
        }

        Ok(color)
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err("Should be 3 elements long".to_string());
        }

        let mut color = Color::default();
        for i in 0..3 {
            Self::validate_component(&slice[i])?;
            color[i] = *&slice[i] as u8;
        }

        Ok(color)
    }
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since From is implemented for Color, we should be able to use Into
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use Into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_positive() {
        let _ = Color::try_from((256, 1000, 10000)).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_negative() {
        let _ = Color::try_from((-1, -10, -256)).unwrap();
    }
    #[test]
    fn test_tuple_correct() {
        let c: Color = (183, 65, 14).try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_array_out_of_range_positive() {
        let _: Color = [1000, 10000, 256].try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_array_out_of_range_negative() {
        let _: Color = [-10, -256, -1].try_into().unwrap();
    }
    #[test]
    fn test_array_correct() {
        let c: Color = [183, 65, 14].try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        let _ = Color::try_from(&arr[..]).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        let _ = Color::try_from(&arr[..]).unwrap();
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c = Color::try_from(&v[..]).unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }
    #[test]
    #[should_panic]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        let _ = Color::try_from(&v[..]).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        let _ = Color::try_from(&v[..]).unwrap();
    }
}
