/// Linear Extrusion is an operation that takes a 2D object as input and generates a 3D object as a result.
///
/// Extrusion follows the V vector which defaults to the Z axis, for specifying a custom value a version > 2021.01 is needed.
///
/// In OpenSCAD Extrusion is always performed on the projection (shadow) of the 2d object xy plane; so if you rotate or apply other transformations to the 2d object before extrusion, its shadow shape is what is extruded.
///
/// Although the extrusion is linear along the V vector, a twist parameter is available that causes the object to be rotated around the V vector as it is extruding upward. This can be used to rotate the object at its center, as if it is a spiral pillar, or produce a helical extrusion around the V vector, like a pig's tail.
///
/// A scale parameter is also included so that the object can be expanded or contracted over the extent of the extrusion, allowing extrusions to be flared inward or outward.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct LinearExtrude {
    center: Option<bool>,
    convexity: Option<crate::types::Scalar>,
    height: Option<crate::types::Length>,
    vector: Option<crate::types::Length3>,
}

#[inline]
pub fn linear_extrude(height: impl Into<crate::types::Length>) -> LinearExtrude {
    LinearExtrude::default().height(height)
}

impl LinearExtrude {
    #[inline]
    pub fn center<T: Into<bool>>(mut self, center: T) -> Self {
        self.center = Some(center.into());
        self
    }

    #[inline]
    pub fn convexity<T: Into<crate::types::Scalar>>(mut self, convexity: T) -> Self {
        self.convexity = Some(convexity.into());
        self
    }

    #[inline]
    pub fn height<T: Into<crate::types::Length>>(mut self, height: T) -> Self {
        self.height = Some(height.into());
        self
    }

    #[inline]
    pub fn vector<T: Into<crate::types::Length3>>(mut self, vector: T) -> Self {
        self.vector = Some(vector.into());
        self
    }
}

impl ::core::fmt::Debug for LinearExtrude {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("linear_extrude");
        if let Some(value) = self.center.as_ref() {
            s.field("center", value);
        }
        if let Some(value) = self.convexity.as_ref() {
            s.field("convexity", value);
        }
        if let Some(value) = self.height.as_ref() {
            s.field("height", value);
        }
        if let Some(value) = self.vector.as_ref() {
            s.field("v", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for LinearExtrude {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "linear_extrude";
        let args = [
            (
                "center",
                self.center
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "convexity",
                self.convexity
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "height",
                self.height
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "v",
                self.vector
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl ::core::fmt::Display for LinearExtrude {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl crate::Operator<2> for LinearExtrude {
    type Output = crate::Object<3>;

    fn apply(self, child: &crate::Object<2>) -> Self::Output {
        let obj: crate::operator::Wrapped<2, 3> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Add<T> for LinearExtrude {
    type Output = crate::Object<3>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Sub<T> for LinearExtrude {
    type Output = crate::Object<3>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::BitOr<T> for LinearExtrude {
    type Output = crate::Object<3>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<3>> ::core::ops::Shr<F> for LinearExtrude {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<LinearExtrude> for crate::Object<3> {
    #[inline]
    fn from(value: LinearExtrude) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<3> for LinearExtrude {
    #[inline]
    fn into_object(self) -> crate::Object<3> {
        crate::Object::new(self)
    }
}

/// Rotational extrusion spins a 2D shape around the Z-axis to form a solid which has rotational symmetry. One way to think of this operation is to imagine a Potter's wheel placed on the X-Y plane with its axis of rotation pointing up towards +Z. Then place the to-be-made object on this virtual Potter's wheel (possibly extending down below the X-Y plane towards -Z). The to-be-made object is the cross-section of the object on the X-Y plane (keeping only the right half, X >= 0). That is the 2D shape that will be fed to rotate_extrude() as the child in order to generate this solid. Note that the object started on the X-Y plane but is tilted up (rotated +90 degrees about the X-axis) to extrude.
///
/// Since a 2D shape is rendered by OpenSCAD on the X-Y plane, an alternative way to think of this operation is as follows: spins a 2D shape around the Y-axis to form a solid. The resultant solid is placed so that its axis of rotation lies along the Z-axis.
///
/// Just like the [`linear_extrude`], the extrusion is always performed on the projection of the 2D polygon to the XY plane. Transformations like rotate, translate, etc. applied to the 2D polygon before extrusion modify the projection of the 2D polygon to the XY plane and therefore also modify the appearance of the final 3D object.
///
/// * A translation in Z of the 2D polygon has no effect on the result (as also the projection is not affected).
/// * A translation in X increases the diameter of the final object.
/// * A translation in Y results in a shift of the final object in Z direction.
/// * A rotation about the X or Y axis distorts the cross section of the final object, as also the projection to the XY plane is distorted.
///
/// Don't get confused, as OpenSCAD renders 2D polygons with a certain height in the Z direction, so the 2D object (with its height) appears to have a bigger projection to the XY plane. But for the projection to the XY plane and also for the later extrusion only the base polygon without height is used.
///
/// It can not be used to produce a helix or screw threads. (These things can be done with [`linear_extrude`] using the twist parameter.)
///
/// The 2D shape must lie completely on either the right (recommended) or the left side of the Y-axis. More precisely speaking, every vertex of the shape must have either x >= 0 or x <= 0. If the shape spans the X axis a warning appears in the console windows and the rotate_extrude() is ignored. If the 2D shape touches the Y axis, i.e. at x=0, it must be a line that touches, not a point, as a point results in a zero thickness 3D object, which is invalid and results in a CGAL error. For OpenSCAD versions prior to 2016.xxxx, if the shape is in the negative axis the resulting faces are oriented inside-out, which may cause undesired effects.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateExtrude {
    angle: Option<crate::types::Angle>,
    convexity: Option<crate::types::Scalar>,
    fragment_angle: Option<crate::types::Angle>,
    fragment_resolution: Option<crate::types::FragmentResolution>,
    fragment_size: Option<crate::types::Length>,
}

#[inline]
pub fn rotate_extrude() -> RotateExtrude {
    RotateExtrude::default()
}

impl RotateExtrude {
    /// Defaults to 360. Specifies the number of degrees to sweep, starting at the positive X axis. The direction of the sweep follows the Right Hand Rule, hence a negative angle sweeps clockwise.
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }

    /// If the extrusion fails for a non-trival 2D shape, try setting the convexity parameter (the default is not 10, but 10 is a "good" value to try). See explanation further down.
    #[inline]
    pub fn convexity<T: Into<crate::types::Scalar>>(mut self, convexity: T) -> Self {
        self.convexity = Some(convexity.into());
        self
    }

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
}

impl ::core::fmt::Debug for RotateExtrude {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_extrude");
        if let Some(value) = self.angle.as_ref() {
            s.field("angle", value);
        }
        if let Some(value) = self.convexity.as_ref() {
            s.field("convexity", value);
        }
        if let Some(value) = self.fragment_angle.as_ref() {
            s.field("$fa", value);
        }
        if let Some(value) = self.fragment_resolution.as_ref() {
            s.field("$fn", value);
        }
        if let Some(value) = self.fragment_size.as_ref() {
            s.field("$fs", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for RotateExtrude {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "rotate_extrude";
        let args = [
            (
                "angle",
                self.angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "convexity",
                self.convexity
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
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
        ];
        f.call(name, args, true)
    }
}

impl ::core::fmt::Display for RotateExtrude {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl crate::Operator<2> for RotateExtrude {
    type Output = crate::Object<3>;

    fn apply(self, child: &crate::Object<2>) -> Self::Output {
        let obj: crate::operator::Wrapped<2, 3> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Add<T> for RotateExtrude {
    type Output = crate::Object<3>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Sub<T> for RotateExtrude {
    type Output = crate::Object<3>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::BitOr<T> for RotateExtrude {
    type Output = crate::Object<3>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<3>> ::core::ops::Shr<F> for RotateExtrude {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<RotateExtrude> for crate::Object<3> {
    #[inline]
    fn from(value: RotateExtrude) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<3> for RotateExtrude {
    #[inline]
    fn into_object(self) -> crate::Object<3> {
        crate::Object::new(self)
    }
}
