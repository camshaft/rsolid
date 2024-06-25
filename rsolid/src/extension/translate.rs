/// Moves children back (in the Y- direction) by the given amount.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Back<const DIMENSIONS: usize> {
    y: Option<crate::types::Length>,
}

/// Moves children back (in the Y- direction) by the given amount.
#[inline]
pub fn back<const DIMENSIONS: usize>(y: impl Into<crate::types::Length>) -> Back<DIMENSIONS> {
    Back::default().y(y)
}

impl<const DIMENSIONS: usize> Back<DIMENSIONS> {
    #[inline]
    pub fn y<T: Into<crate::types::Length>>(mut self, y: T) -> Self {
        self.y = Some(y.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Back<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("back");
        if let Some(value) = self.y.as_ref() {
            s.field("y", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Back<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(y=undef) { translate([0, -y, 0]) children(); }");
        let args = [(
            "y",
            self.y
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Back<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Back<DIMENSIONS> {
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
    for Back<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Back<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Back<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Back<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Back<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Back<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Back<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Moves children down (in the Z- direction) by the given amount.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Down<const DIMENSIONS: usize> {
    z: Option<crate::types::Length>,
}

/// Moves children down (in the Z- direction) by the given amount.
#[inline]
pub fn down<const DIMENSIONS: usize>(z: impl Into<crate::types::Length>) -> Down<DIMENSIONS> {
    Down::default().z(z)
}

impl<const DIMENSIONS: usize> Down<DIMENSIONS> {
    #[inline]
    pub fn z<T: Into<crate::types::Length>>(mut self, z: T) -> Self {
        self.z = Some(z.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Down<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("down");
        if let Some(value) = self.z.as_ref() {
            s.field("z", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Down<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(z=undef) { translate([0, 0, -z]) children(); }");
        let args = [(
            "z",
            self.z
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Down<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Down<DIMENSIONS> {
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
    for Down<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Down<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Down<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Down<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Down<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Down<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Down<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Moves children forward (in the Y+ direction) by the given amount.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Fwd<const DIMENSIONS: usize> {
    y: Option<crate::types::Length>,
}

/// Moves children forward (in the Y+ direction) by the given amount.
#[inline]
pub fn fwd<const DIMENSIONS: usize>(y: impl Into<crate::types::Length>) -> Fwd<DIMENSIONS> {
    Fwd::default().y(y)
}

impl<const DIMENSIONS: usize> Fwd<DIMENSIONS> {
    #[inline]
    pub fn y<T: Into<crate::types::Length>>(mut self, y: T) -> Self {
        self.y = Some(y.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Fwd<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("fwd");
        if let Some(value) = self.y.as_ref() {
            s.field("y", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Fwd<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(y=undef) { translate([0, y, 0]) children(); }");
        let args = [(
            "y",
            self.y
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Fwd<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Fwd<DIMENSIONS> {
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
    for Fwd<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Fwd<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Fwd<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Fwd<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Fwd<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Fwd<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Fwd<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Moves children left (in the X- direction) by the given amount.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Left<const DIMENSIONS: usize> {
    x: Option<crate::types::Length>,
}

/// Moves children left (in the X- direction) by the given amount.
#[inline]
pub fn left<const DIMENSIONS: usize>(x: impl Into<crate::types::Length>) -> Left<DIMENSIONS> {
    Left::default().x(x)
}

impl<const DIMENSIONS: usize> Left<DIMENSIONS> {
    #[inline]
    pub fn x<T: Into<crate::types::Length>>(mut self, x: T) -> Self {
        self.x = Some(x.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Left<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("left");
        if let Some(value) = self.x.as_ref() {
            s.field("x", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Left<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(x=undef) { translate([-x, 0, 0]) children(); }");
        let args = [(
            "x",
            self.x
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Left<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Left<DIMENSIONS> {
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
    for Left<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Left<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Left<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Left<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Left<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Left<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Left<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Moves children right (in the X+ direction) by the given amount.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Right<const DIMENSIONS: usize> {
    x: Option<crate::types::Length>,
}

/// Moves children right (in the X+ direction) by the given amount.
#[inline]
pub fn right<const DIMENSIONS: usize>(x: impl Into<crate::types::Length>) -> Right<DIMENSIONS> {
    Right::default().x(x)
}

impl<const DIMENSIONS: usize> Right<DIMENSIONS> {
    #[inline]
    pub fn x<T: Into<crate::types::Length>>(mut self, x: T) -> Self {
        self.x = Some(x.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Right<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("right");
        if let Some(value) = self.x.as_ref() {
            s.field("x", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Right<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(x=undef) { translate([x, 0, 0]) children(); }");
        let args = [(
            "x",
            self.x
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Right<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Right<DIMENSIONS> {
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
    for Right<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Right<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Right<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Right<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Right<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Right<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Right<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Moves children up (in the Z+ direction) by the given amount.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Up<const DIMENSIONS: usize> {
    z: Option<crate::types::Length>,
}

/// Moves children up (in the Z+ direction) by the given amount.
#[inline]
pub fn up<const DIMENSIONS: usize>(z: impl Into<crate::types::Length>) -> Up<DIMENSIONS> {
    Up::default().z(z)
}

impl<const DIMENSIONS: usize> Up<DIMENSIONS> {
    #[inline]
    pub fn z<T: Into<crate::types::Length>>(mut self, z: T) -> Self {
        self.z = Some(z.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Up<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("up");
        if let Some(value) = self.z.as_ref() {
            s.field("z", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Up<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(z=undef) { translate([0, 0, z]) children(); }");
        let args = [(
            "z",
            self.z
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Up<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Up<DIMENSIONS> {
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
    for Up<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Up<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Up<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Up<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Up<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Up<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Up<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
