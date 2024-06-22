use crate::{
    scad::{self, Scad},
    IntoObject, Object,
};
use core::fmt;

macro_rules! impl_block {
    ([$($acc:ident($a_value:tt),)*]) => {
        // done
    };
    ($head:ident($h_value:tt), $($tail:ident($t_value:tt), )* [$($acc:ident($a_value:tt),)*]) => {
        impl<$head: IntoObject<DIM> $(, $acc: IntoObject<DIM>)*, const DIM: usize> fmt::Display for ($($acc, )* $head ,) {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(&self.to_scad())
            }
        }

        impl<$head: IntoObject<DIM> $(, $acc: IntoObject<DIM>)*, const DIM: usize> Scad for ($($acc, )* $head ,) {
            fn assign(&self, f: &mut scad::Formatter) -> scad::Assignment {

                todo!()
            }
        }

        impl<$head: IntoObject<DIM> $(, $acc: IntoObject<DIM>)*, const DIM: usize> IntoObject<DIM> for ($($acc, )* $head ,) {
            fn into_object(self) -> Object<DIM> {
                Object::new(($(
                    self.$a_value.into_object(),
                )* self.$h_value.into_object(),))
            }
        }

        impl_block!($($tail($t_value),)* [$($acc($a_value),)* $head($h_value),]);
    }
}

impl_block!(
    // A(0),
    //B(1),
    /*
    C(2),
    D(3),
    E(4),
    F(5),
    G(6),
    H(7),
    I(8),
    J(9),
    K(10),
    L(11),
    M(12),
    N(13),
    O(14),
    P(15),
    Q(16),
    R(17),
    S(18),
    T(19),
    U(20),
    V(21),
    W(22),
    X(23),
    Y(24),
    Z(25),
    AA(26),
    AB(27),
    AC(28),
    AD(29),
    AE(30),
    AF(31),
    AG(32),
    */
    []
);

#[derive(Clone, Debug)]
pub(crate) struct Block<const DIMENSIONS: usize>(pub Object<DIMENSIONS>, pub Object<DIMENSIONS>);

impl<const DIMENSIONS: usize> fmt::Display for Block<DIMENSIONS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_scad())
    }
}

impl<const DIMENSIONS: usize> Scad for Block<DIMENSIONS> {
    fn assign(&self, f: &mut scad::Formatter) -> scad::Assignment {
        let a = self.0.assign(f);
        let b = self.1.assign(f);
        //f.emit(format_args!("{{ {a}; {b}; }}"), true)
        scad::Assignment::Inline {
            code: format!("{{ {a}; {b}; }}"),
        }
    }
}

impl<const DIMENSIONS: usize> IntoObject<DIMENSIONS> for Block<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> Object<DIMENSIONS> {
        Object::new(self)
    }
}
