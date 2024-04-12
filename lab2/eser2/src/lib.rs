pub mod solution {
    use std::ops::{Add, AddAssign, Div, Sub};

    #[derive(Copy, Clone)]
    pub struct ComplexNumber {
        pub real: f64,
        pub i: f64
    }



    impl Add for ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                i: self.i + rhs.i,
            }
        }
    }
    impl Add<f64> for ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: f64) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs,
                i: self.i,
            }
        }
    }

    impl AddAssign for ComplexNumber{
       fn add_assign(&mut self, rhs: ComplexNumber) {
                self.real += rhs.real;
                self.i += rhs.i;
        }

    }

    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                i: self.i + rhs.i,
            }
        }
    }


    impl Sub for ComplexNumber {
        type Output = ComplexNumber;
        fn sub(self, rhs: ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real - rhs.real,
                i: self.i - rhs.i,
            }
        }
    }

    impl Div for ComplexNumber {
        type Output = ComplexNumber;
        fn div(self, rhs: ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: ((self.real * rhs.real) +(self.i + rhs.i))/(rhs.real*rhs.real + rhs.i*rhs.i),
                i: ((self.i * rhs.real)- (self.real*rhs.i))/(rhs.real*rhs.real + rhs.i*rhs.i),
            }
        }
    }

    impl Default for ComplexNumber {
        // default color is black
        fn default() -> Self {
            ComplexNumber {
                real: 0.0,
                i: 0.0,
            }
        }
    }
    impl ComplexNumber{
        pub fn new(a:f64, b:f64) -> ComplexNumber{
            let c = ComplexNumber{real: a, i: b};
            c
        }

        pub fn real(&self) -> f64{
            self.real
        }

        pub fn imag(&self) -> f64{
            self.i
        }

        pub fn to_tuple(&self) -> (f64, f64){
            (self.real, self.i)
        }

        pub fn from_real(a: f64) -> ComplexNumber{
            ComplexNumber{real: a, i:0.0 }
        }
    }

}