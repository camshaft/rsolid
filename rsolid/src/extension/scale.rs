/// Scales children by the given factor on the X axis.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Xscale<const DIMENSIONS: usize> {
    x: Option<crate::types::Scalar>,
}

/// Scales children by the given factor on the X axis.
#[inline]
pub fn xscale<const DIMENSIONS: usize>(x: impl Into<crate::types::Scalar>) -> Xscale<DIMENSIONS> {
    Xscale::default().x(x)
}

impl<const DIMENSIONS: usize> Xscale<DIMENSIONS> {
    #[inline]
    pub fn x<T: Into<crate::types::Scalar>>(mut self, x: T) -> Self {
        self.x = Some(x.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Xscale<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("xscale");
        if let Some(value) = self.x.as_ref() {
            s.field("x", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Xscale<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(x=undef) { scale([x, 1, 1]) children(); }");
        let args = [(
            "x",
            self.x
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Xscale<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Xscale<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: &crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Xscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Xscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Xscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Xscale<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Xscale<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Xscale<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Xscale<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Scales children by the given factor on the Y axis.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Yscale<const DIMENSIONS: usize> {
    y: Option<crate::types::Scalar>,
    z: Option<crate::types::Scalar>,
}

/// Scales children by the given factor on the Y axis.
#[inline]
pub fn yscale<const DIMENSIONS: usize>() -> Yscale<DIMENSIONS> {
    Yscale::default()
}

impl<const DIMENSIONS: usize> Yscale<DIMENSIONS> {
    #[inline]
    pub fn y<T: Into<crate::types::Scalar>>(mut self, y: T) -> Self {
        self.y = Some(y.into());
        self
    }

    #[inline]
    pub fn z<T: Into<crate::types::Scalar>>(mut self, z: T) -> Self {
        self.z = Some(z.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Yscale<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("yscale");
        if let Some(value) = self.y.as_ref() {
            s.field("y", value);
        }
        if let Some(value) = self.z.as_ref() {
            s.field("z", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Yscale<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(y=undef, z=undef) { scale([1, y, 1]) children(); }");
        let args = [
            (
                "y",
                self.y
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "z",
                self.z
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Yscale<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Yscale<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: &crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Yscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Yscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Yscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Yscale<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Yscale<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Yscale<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Yscale<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Scales children by the given factor on the Z axis.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Zscale<const DIMENSIONS: usize> {}

/// Scales children by the given factor on the Z axis.
#[inline]
pub fn zscale<const DIMENSIONS: usize>() -> Zscale<DIMENSIONS> {
    Zscale::default()
}

impl<const DIMENSIONS: usize> Zscale<DIMENSIONS> {}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Zscale<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("zscale");
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Zscale<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("() { scale([1, 1, z]) children(); }");
        let args = [];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Zscale<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Zscale<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: &crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Zscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Zscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Zscale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Zscale<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Zscale<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Zscale<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Zscale<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
