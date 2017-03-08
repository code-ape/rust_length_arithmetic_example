use std::marker::PhantomData;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::mem;
use std::cmp::Ordering;


#[derive(Debug, Clone, Copy, Eq, Ord)]
struct Length<T: LengthUnit> {
    nm: i64,
    unit: PhantomData<T>,
}


trait LengthUnit: Copy + Eq {
    fn singular_name() -> String;
    fn num_nm_in_unit() -> i64;
}

macro_rules! NewLength {
    ($struct_name:ident, $string_name:expr , $nm_conv:expr) => {
        
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        struct $struct_name; // unit-like struct
        
        impl LengthUnit for $struct_name {
            #[inline(always)]
            fn singular_name() -> String { $string_name.to_string() }
            #[inline(always)]
            fn num_nm_in_unit() -> i64 { $nm_conv }
        }

    };
}

NewLength!(Meters, "meter", 1_000_000_000);
NewLength!(Millimeters, "millimeter", 1_000_000);
NewLength!(Kilometers, "kilometer", 1_000_000_000_000);


impl<T> fmt::Display for Length<T> where T: LengthUnit {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_val = (self.nm as f64) / (T::num_nm_in_unit() as f64);
        let name_plural_s = match num_val {
            1_f64 => "",
            _ => "s"
        };
        write!(f,
               "{} {}{}",
               (self.nm as f64) / (T::num_nm_in_unit() as f64),
               T::singular_name(),
               name_plural_s)
    }
}


macro_rules! ImplFromLengthUnit {
    ($N:ty) => {
        // from number $N to Length<T>
        impl<T> From<$N> for Length<T> where T: LengthUnit {
            fn from(n: $N) -> Self {
                Length {
                    nm: (n as i64) * T::num_nm_in_unit(),
                    unit: PhantomData
                }
            }
        }

        // from number &'a $N to Length<T>, needed for conversion macros
        impl<'a, T> From<&'a $N> for Length<T> where T: LengthUnit {
            fn from(n: &'a $N) -> Self {
                Length {
                    nm: (*n as i64) * T::num_nm_in_unit(),
                    unit: PhantomData
                }
            }
        }

        // from Length<T> to number $N
        impl<T> From<Length<T>> for $N where T: LengthUnit {
            fn from(l: Length<T>) -> $N {
                ((l.nm as f64) / (T::num_nm_in_unit() as f64)) as $N
            }
        }
    };
}

// Implement conversions for i64 and f64
ImplFromLengthUnit!(i64);
ImplFromLengthUnit!(f64);


// transformation operation from one length type to another
// must use a reference to avoid conflicting with compilers
// auto generated From for Length<_>
impl<'a, T1, T2> From<&'a Length<T1>> for Length<T2>
    where T1: LengthUnit,
          T2: LengthUnit
{
    fn from(l: &'a Length<T1>) -> Self {
        Length {
            nm: l.nm,
            unit: PhantomData,
        }
    }
}


// Allow lengths to be added
impl<T1, T2> Add<Length<T2>> for Length<T1>
    where T1: LengthUnit,
          T2: LengthUnit
{
    type Output = Length<T1>;

    fn add(self, other: Length<T2>) -> Length<T1> {
        Length {
            nm: self.nm + other.nm,
            unit: PhantomData,
        }
    }
}

// Allow lengths to be subtracted
impl<T1, T2> Sub<Length<T2>> for Length<T1>
    where T1: LengthUnit,
          T2: LengthUnit
{
    type Output = Length<T1>;

    fn sub(self, other: Length<T2>) -> Length<T1> {
        Length {
            nm: self.nm - other.nm,
            unit: PhantomData,
        }
    }
}

// Allow lengths to be divided
// this yields a number as a length divided by a length is just a number
impl<T1, T2> Div<Length<T2>> for Length<T1>
    where T1: LengthUnit,
          T2: LengthUnit
{
    type Output = f64;

    fn div(self, other: Length<T2>) -> f64 {
        (self.nm as f64) / (other.nm as f64)
    }
}

// Macro to implement multiplication and division both ways
// for $num_type and Length
macro_rules! ImplMulandDivLengthAndNum {
    ($num_type:ty) => {
        impl<T> Mul<$num_type> for Length<T> where T: LengthUnit {
            type Output = Length<T>;
        
            fn mul(self, other: $num_type) -> Length<T> {
                Length {
                    nm: ((self.nm as $num_type) * other) as i64,
                    unit: PhantomData,
                }
            }
        }
        impl<T> Mul<Length<T>> for $num_type where T: LengthUnit {
            type Output = Length<T>;
        
            fn mul(self, other: Length<T>) -> Length<T> {
                Length {
                    nm: ((other.nm as $num_type) * self) as i64,
                    unit: PhantomData,
                }
            }
        }
        impl<T> Div<$num_type> for Length<T> where T: LengthUnit {
            type Output = Length<T>;
        
            fn div(self, other: $num_type) -> Length<T> {
                Length {
                    nm: ((self.nm as $num_type) / other) as i64,
                    unit: PhantomData,
                }
            }
        }
        impl<T> Div<Length<T>> for $num_type where T: LengthUnit {
            type Output = Length<T>;
        
            fn div(self, other: Length<T>) -> Length<T> {
                Length {
                    nm: ((other.nm as $num_type) / self) as i64,
                    unit: PhantomData,
                }
            }
        }
    };
}

// implement multiplication and division of Lengths for i64 and u64
ImplMulandDivLengthAndNum!(i64);
ImplMulandDivLengthAndNum!(f64);


// implement PartialEq for comparing Lengths with different units
impl<T1, T2> PartialEq<Length<T2>> for Length<T1> where T1: LengthUnit, T2: LengthUnit {
    fn eq(&self, other: &Length<T2>) -> bool {
        self.nm == other.nm
    }
}

// implement PartialORd for ordering Lengths with different units
impl<T1,T2> PartialOrd<Length<T2>> for Length<T1> where T1: LengthUnit, T2: LengthUnit {
    fn partial_cmp(&self, other: &Length<T2>) -> Option<Ordering> {
        Some(self.nm.cmp(&other.nm))
    }
}


// calculate circumference of given radius
// allows total abstraction over concept of units
fn circumference<T>(r: Length<T>) -> Length<T> where T: LengthUnit {
    2 * r * std::f64::consts::PI
}

// convert a number or length to meters
macro_rules! meters {
    ($num:expr) => (Length::<Meters>::from(&$num));
}

// convert a number or length to millimeters
macro_rules! millimeters {
    ($num:expr) => (Length::<Millimeters>::from(&$num));
}

// convert a number or length to kilometers
macro_rules! kilometers {
    ($num:expr) => (Length::<Kilometers>::from(&$num));
}

// main function which allows easy and clean use
// it will print the following:
//
// l1 = 10 millimeters
// l2 = 5 meters
// l3 = (5 * l1) + l2 = 5050 millimeters
// l3_meters = 5.05
// circumference(radius = 10 millimeters) = 62.831853 millimeters
// l3 > l2 : true
// l3 / l2 = 1.01
// size_of(Length<Meters>) = 8 bytes
fn main() {             

    let l1 = millimeters!(10);
    let l2 = meters!(5);
    let l3 = (5 * l1) + l2;
    let l3_meters = f64::from(meters!(l3));
    let c1 = circumference(l1);

    println!("l1 = {}", l1);
    println!("l2 = {}", l2);
    println!("l3 = (5 * l1) + l2 = {}", l3);
    println!("l3_meters = {}", l3_meters);
    println!("circumference(radius = {}) = {}", l1, c1);
    println!("l3 > l2 : {}", l3 > l2);
    println!("l3 / l2 = {}", l3 / l2);

    println!("size_of(Length<Meters>) = {} bytes",
        mem::size_of::<Length<Meters>>());

}

// implementation of sqrt that ended up not being used in the article
// based on bitwise algorithm here:
//  https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
impl<T> Length<T> where T: LengthUnit {

    fn sqrt(self) -> Result<Self, String> {
        let n = self.nm;
        if n < 0 {
            return Err("Can't take sqrt of negative number".to_string());
        }

        // Find greatest shift.
        let mut shift = 2;
        let mut n_shifted = n >> shift;
        while n_shifted != 0 && n_shifted != n {
            shift += 2;
            n_shifted = n >> shift;
        }
        shift -= 2;
        // Find digits of result.
        let mut result = 0;
        let mut candidate_result;
        while shift >= 0 {
            result = result << 1;
            candidate_result = result + 1;
            if (candidate_result * candidate_result) <= (n >> shift) {
                result = candidate_result;
            }
            shift -= 2;
        }

        Ok(Length {
            nm: result,
            unit: PhantomData,
        })
    }
}
