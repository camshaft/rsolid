/// Creates a circle at the origin.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Circle {
    fragment_angle: Option<crate::types::Angle>,
    fragment_resolution: Option<crate::types::FragmentResolution>,
    fragment_size: Option<crate::types::Length>,
    radius: Option<crate::types::Length>,
}

#[inline]
pub fn circle(radius: impl Into<crate::types::Length>) -> Circle {
    Circle::default().radius(radius)
}

impl Circle {
    /// Fragment angle in degrees
    #[inline]
    pub fn fragment_angle<T: Into<crate::types::Angle>>(mut self, fragment_angle: T) -> Self {
        self.fragment_angle = Some(fragment_angle.into());
        self
    }

    /// Fragment resolution
    #[inline]
    pub fn fragment_resolution<T: Into<crate::types::FragmentResolution>>(
        mut self,
        fragment_resolution: T,
    ) -> Self {
        self.fragment_resolution = Some(fragment_resolution.into());
        self
    }

    /// Fragment size in mm
    #[inline]
    pub fn fragment_size<T: Into<crate::types::Length>>(mut self, fragment_size: T) -> Self {
        self.fragment_size = Some(fragment_size.into());
        self
    }

    /// circle radius
    #[inline]
    pub fn radius<T: Into<crate::types::Length>>(mut self, radius: T) -> Self {
        self.radius = Some(radius.into());
        self
    }
}

impl ::core::fmt::Debug for Circle {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("circle");
        if let Some(value) = self.fragment_angle.as_ref() {
            s.field("$fa", value);
        }
        if let Some(value) = self.fragment_resolution.as_ref() {
            s.field("$fn", value);
        }
        if let Some(value) = self.fragment_size.as_ref() {
            s.field("$fs", value);
        }
        if let Some(value) = self.radius.as_ref() {
            s.field("r", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Circle {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "circle";
        let args = [
            (
                "$fa",
                self.fragment_angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "$fn",
                self.fragment_resolution
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "$fs",
                self.fragment_size
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "r",
                self.radius
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, false)
    }
}

impl ::core::fmt::Display for Circle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Add<T> for Circle {
    type Output = crate::Object<2>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Sub<T> for Circle {
    type Output = crate::Object<2>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::BitOr<T> for Circle {
    type Output = crate::Object<2>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<2>> ::core::ops::Shr<F> for Circle {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Circle> for crate::Object<2> {
    #[inline]
    fn from(value: Circle) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<2> for Circle {
    #[inline]
    fn into_object(self) -> crate::Object<2> {
        crate::Object::new(self)
    }
}

/// Creates a square or rectangle in the first quadrant.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Square {
    center: Option<bool>,
    size: Option<crate::types::Length2>,
}

#[inline]
pub fn square(size: impl Into<crate::types::Length2>) -> Square {
    Square::default().size(size)
}

impl Square {
    /// false (default), 1st (positive) quadrant, one corner at (0,0)
    /// true, square is centered at (0,0)
    #[inline]
    pub fn center<T: Into<bool>>(mut self, center: T) -> Self {
        self.center = Some(center.into());
        self
    }

    /// single value, square with both sides this length
    /// 2 value array [x,y], rectangle with dimensions x and y
    #[inline]
    pub fn size<T: Into<crate::types::Length2>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl ::core::fmt::Debug for Square {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("square");
        if let Some(value) = self.center.as_ref() {
            s.field("center", value);
        }
        if let Some(value) = self.size.as_ref() {
            s.field("size", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Square {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "square";
        let args = [
            (
                "center",
                self.center
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "size",
                self.size
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, false)
    }
}

impl ::core::fmt::Display for Square {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Add<T> for Square {
    type Output = crate::Object<2>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Sub<T> for Square {
    type Output = crate::Object<2>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::BitOr<T> for Square {
    type Output = crate::Object<2>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<2>> ::core::ops::Shr<F> for Square {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Square> for crate::Object<2> {
    #[inline]
    fn from(value: Square) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<2> for Square {
    #[inline]
    fn into_object(self) -> crate::Object<2> {
        crate::Object::new(self)
    }
}
