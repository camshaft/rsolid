/// Transforms the child element to a mirror of the original, as if it were the mirror image seen through a plane intersecting the origin.
///
/// The argument to mirror() is the normal vector of the origin-intersecting mirror plane used, meaning the vector coming perpendicularly out of the plane. Each coordinate of the original object is altered such that it becomes equidistant on the other side of this plane from the closest point on the plane. For example, mirror([1,0,0]), corresponding to a normal vector pointing in the x-axis direction, produces an object such that all positive x coordinates become negative x coordinates, and all negative x coordinates become positive x coordinates.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Mirror<const DIMENSIONS: usize> {
    value: Option<crate::types::Length3>,
}

#[inline]
pub fn mirror<const DIMENSIONS: usize>(
    value: impl Into<crate::types::Length3>,
) -> Mirror<DIMENSIONS> {
    Mirror::default().value(value)
}

impl<const DIMENSIONS: usize> Mirror<DIMENSIONS> {
    #[inline]
    pub fn value<T: Into<crate::types::Length3>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Mirror<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("mirror");
        if let Some(value) = self.value.as_ref() {
            s.field("v", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Mirror<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "mirror";
        let args = [(
            "v",
            self.value
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Mirror<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Mirror<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child,
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Mirror<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Mirror<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Mirror<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Mirror<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Mirror<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Mirror<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Mirror<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Offset generates a new 2d interior or exterior outline from an existing outline. There are two modes of operation: radial and delta.
///
/// The radial method creates a new outline as if a circle of some radius is rotated around the exterior (r > 0) or interior (r < 0) of the original outline.
/// The delta method creates a new outline with sides having a fixed distance outward (delta > 0) or inward (delta < 0) from the original outline.
///
/// The construction methods produce an outline that is either inside or outside of the original outline. For outlines using delta, when the outline goes around a corner, it can be given an optional chamfer.
///
/// Offset is useful for making thin walls by subtracting a negative-offset construction from the original, or the original from a positive offset construction.
///
/// Offset can be used to simulate some common solid modeling operations:
///
/// Fillet: offset(r=-3) offset(delta=+3) rounds all inside (concave) corners, and leaves flat walls unchanged. However, holes less than 2*r in diameter vanish.
///     Round: offset(r=+3) offset(delta=-3) rounds all outside (convex) corners, and leaves flat walls unchanged. However, walls less than 2*r thick vanish.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Offset<const DIMENSIONS: usize> {
    chamfer: Option<bool>,
    delta: Option<crate::types::Length>,
    radius: Option<crate::types::Length>,
}

#[inline]
pub fn offset<const DIMENSIONS: usize>(
    radius: impl Into<crate::types::Length>,
) -> Offset<DIMENSIONS> {
    Offset::default().radius(radius)
}

impl<const DIMENSIONS: usize> Offset<DIMENSIONS> {
    /// When using the delta parameter, this flag defines if edges should be chamfered (cut off with a straight line) or not (extended to their intersection). This parameter has no effect on radial offsets.
    #[inline]
    pub fn chamfer<T: Into<bool>>(mut self, chamfer: T) -> Self {
        self.chamfer = Some(chamfer.into());
        self
    }

    #[inline]
    pub fn delta<T: Into<crate::types::Length>>(mut self, delta: T) -> Self {
        self.delta = Some(delta.into());
        self
    }

    /// Specifies the radius of the circle that is rotated about the outline, either inside or outside. This mode produces rounded corners.
    #[inline]
    pub fn radius<T: Into<crate::types::Length>>(mut self, radius: T) -> Self {
        self.radius = Some(radius.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Offset<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("offset");
        if let Some(value) = self.chamfer.as_ref() {
            s.field("chamfer", value);
        }
        if let Some(value) = self.delta.as_ref() {
            s.field("delta", value);
        }
        if let Some(value) = self.radius.as_ref() {
            s.field("r", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Offset<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "offset";
        let args = [
            (
                "chamfer",
                self.chamfer
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "delta",
                self.delta
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
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Offset<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Offset<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child,
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Offset<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Offset<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Offset<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Offset<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Offset<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Offset<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Offset<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Modifies the size of the child object to match the given x,y, and z.
///
/// resize() is a CGAL operation, and like others such as render() operates with full geometry, so even in preview this takes time to process.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Resize<const DIMENSIONS: usize> {
    size: Option<crate::types::Length3>,
}

#[inline]
pub fn resize<const DIMENSIONS: usize>(
    size: impl Into<crate::types::Length3>,
) -> Resize<DIMENSIONS> {
    Resize::default().size(size)
}

impl<const DIMENSIONS: usize> Resize<DIMENSIONS> {
    #[inline]
    pub fn size<T: Into<crate::types::Length3>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Resize<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("resize");
        if let Some(value) = self.size.as_ref() {
            s.field("newsize", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Resize<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "resize";
        let args = [(
            "newsize",
            self.size
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Resize<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Resize<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child,
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Resize<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Resize<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Resize<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Resize<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Resize<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Resize<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Resize<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates a child 'a degrees about the axis around an arbitrary axis.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Rotate<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle3>,
    value: Option<crate::types::Length3>,
}

#[inline]
pub fn rotate<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle3>,
) -> Rotate<DIMENSIONS> {
    Rotate::default().angle(angle)
}

#[inline]
pub fn rotate_around<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle3>,
    value: impl Into<crate::types::Length3>,
) -> Rotate<DIMENSIONS> {
    Rotate::default().angle(angle).value(value)
}

impl<const DIMENSIONS: usize> Rotate<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle3>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }

    #[inline]
    pub fn value<T: Into<crate::types::Length3>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Rotate<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.value.as_ref() {
            s.field("v", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Rotate<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "rotate";
        let args = [
            (
                "a",
                self.angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "v",
                self.value
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Rotate<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Rotate<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child,
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Rotate<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Rotate<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Rotate<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Rotate<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Rotate<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Rotate<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Rotate<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Scales its child elements using the specified vector.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Scale<const DIMENSIONS: usize> {
    value: Option<crate::types::Scalar3>,
}

#[inline]
pub fn scale<const DIMENSIONS: usize>(
    value: impl Into<crate::types::Scalar3>,
) -> Scale<DIMENSIONS> {
    Scale::default().value(value)
}

impl<const DIMENSIONS: usize> Scale<DIMENSIONS> {
    #[inline]
    pub fn value<T: Into<crate::types::Scalar3>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Scale<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("scale");
        if let Some(value) = self.value.as_ref() {
            s.field("v", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Scale<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "scale";
        let args = [(
            "v",
            self.value
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Scale<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Scale<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child,
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Scale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Scale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Scale<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Scale<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Scale<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Scale<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Scale<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Translates (moves) its child elements along the specified vector.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Translate<const DIMENSIONS: usize> {
    value: Option<crate::types::Length3>,
}

#[inline]
pub fn translate<const DIMENSIONS: usize>(
    value: impl Into<crate::types::Length3>,
) -> Translate<DIMENSIONS> {
    Translate::default().value(value)
}

impl<const DIMENSIONS: usize> Translate<DIMENSIONS> {
    #[inline]
    pub fn value<T: Into<crate::types::Length3>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Translate<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("translate");
        if let Some(value) = self.value.as_ref() {
            s.field("v", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Translate<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "translate";
        let args = [(
            "v",
            self.value
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Translate<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Translate<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child,
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Translate<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Translate<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Translate<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Translate<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Translate<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Translate<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Translate<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
