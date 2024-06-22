/// Creates a cube or rectangular prism (i.e., a "box") in the first octant.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Cube {
    center: Option<bool>,
    size: Option<crate::types::Length3>,
}

#[inline]
pub fn cube(size: impl Into<crate::types::Length3>) -> Cube {
    Cube::default().size(size)
}

impl Cube {
    /// When center is true, the cube is centered on the origin.
    #[inline]
    pub fn center<T: Into<bool>>(mut self, center: T) -> Self {
        self.center = Some(center.into());
        self
    }

    /// When a single value is provided, the cube's sides will all be this length.
    /// When a coordinate is provided, a rectangular prism with dimensions x, y and z will be created.
    #[inline]
    pub fn size<T: Into<crate::types::Length3>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl ::core::fmt::Debug for Cube {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("cube");
        if let Some(value) = self.center.as_ref() {
            s.field("center", value);
        }
        if let Some(value) = self.size.as_ref() {
            s.field("size", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Cube {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "cube";
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

impl ::core::fmt::Display for Cube {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Add<T> for Cube {
    type Output = crate::Object<3>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Sub<T> for Cube {
    type Output = crate::Object<3>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::BitOr<T> for Cube {
    type Output = crate::Object<3>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<3>> ::core::ops::Shr<F> for Cube {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Cube> for crate::Object<3> {
    #[inline]
    fn from(value: Cube) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<3> for Cube {
    #[inline]
    fn into_object(self) -> crate::Object<3> {
        crate::Object::new(self)
    }
}

/// Creates a cylinder or cone centered about the z axis.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Cylinder {
    center: Option<bool>,
    height: Option<crate::types::Length>,
    radius: Option<crate::types::Length>,
    radius_bottom: Option<crate::types::Length>,
    radius_top: Option<crate::types::Length>,
}

#[inline]
pub fn cylinder(
    height: impl Into<crate::types::Length>,
    radius: impl Into<crate::types::Length>,
) -> Cylinder {
    Cylinder::default().height(height).radius(radius)
}

#[inline]
pub fn cone(
    height: impl Into<crate::types::Length>,
    radius_bottom: impl Into<crate::types::Length>,
    radius_top: impl Into<crate::types::Length>,
) -> Cylinder {
    Cylinder::default()
        .height(height)
        .radius_bottom(radius_bottom)
        .radius_top(radius_top)
}

impl Cylinder {
    /// If false (default), z ranges from 0 to h. If true, z ranges from -h/2 to +h/2.
    #[inline]
    pub fn center<T: Into<bool>>(mut self, center: T) -> Self {
        self.center = Some(center.into());
        self
    }

    /// Height of the cylinder or cone
    #[inline]
    pub fn height<T: Into<crate::types::Length>>(mut self, height: T) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Radius of the cylinder
    #[inline]
    pub fn radius<T: Into<crate::types::Length>>(mut self, radius: T) -> Self {
        self.radius = Some(radius.into());
        self
    }

    /// Radius of bottom cone
    #[inline]
    pub fn radius_bottom<T: Into<crate::types::Length>>(mut self, radius_bottom: T) -> Self {
        self.radius_bottom = Some(radius_bottom.into());
        self
    }

    /// Radius of top cone
    #[inline]
    pub fn radius_top<T: Into<crate::types::Length>>(mut self, radius_top: T) -> Self {
        self.radius_top = Some(radius_top.into());
        self
    }
}

impl ::core::fmt::Debug for Cylinder {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("cylinder");
        if let Some(value) = self.center.as_ref() {
            s.field("center", value);
        }
        if let Some(value) = self.height.as_ref() {
            s.field("h", value);
        }
        if let Some(value) = self.radius.as_ref() {
            s.field("r", value);
        }
        if let Some(value) = self.radius_bottom.as_ref() {
            s.field("r1", value);
        }
        if let Some(value) = self.radius_top.as_ref() {
            s.field("r2", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Cylinder {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "cylinder";
        let args = [
            (
                "center",
                self.center
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "h",
                self.height
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "r",
                self.radius
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "r1",
                self.radius_bottom
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "r2",
                self.radius_top
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, false)
    }
}

impl ::core::fmt::Display for Cylinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Add<T> for Cylinder {
    type Output = crate::Object<3>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Sub<T> for Cylinder {
    type Output = crate::Object<3>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::BitOr<T> for Cylinder {
    type Output = crate::Object<3>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<3>> ::core::ops::Shr<F> for Cylinder {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Cylinder> for crate::Object<3> {
    #[inline]
    fn from(value: Cylinder) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<3> for Cylinder {
    #[inline]
    fn into_object(self) -> crate::Object<3> {
        crate::Object::new(self)
    }
}

/// Creates a sphere at the origin of the coordinate system.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Sphere {
    fragment_angle: Option<crate::types::Angle>,
    fragment_resolution: Option<crate::types::FragmentResolution>,
    fragment_size: Option<crate::types::Length>,
    radius: Option<crate::types::Length>,
}

#[inline]
pub fn sphere(radius: impl Into<crate::types::Length>) -> Sphere {
    Sphere::default().radius(radius)
}

impl Sphere {
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

    /// This is the radius of the sphere. The resolution of the sphere is based on the size of the sphere and the $fa, $fs and $fn variables.
    #[inline]
    pub fn radius<T: Into<crate::types::Length>>(mut self, radius: T) -> Self {
        self.radius = Some(radius.into());
        self
    }
}

impl ::core::fmt::Debug for Sphere {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("sphere");
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

impl crate::scad::Scad for Sphere {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "sphere";
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

impl ::core::fmt::Display for Sphere {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Add<T> for Sphere {
    type Output = crate::Object<3>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Sub<T> for Sphere {
    type Output = crate::Object<3>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::BitOr<T> for Sphere {
    type Output = crate::Object<3>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<3>> ::core::ops::Shr<F> for Sphere {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Sphere> for crate::Object<3> {
    #[inline]
    fn from(value: Sphere) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<3> for Sphere {
    #[inline]
    fn into_object(self) -> crate::Object<3> {
        crate::Object::new(self)
    }
}
