/// The minimum angle for a fragment.
///
/// Even a huge circle does not have more fragments than 360 divided by this number. The default value is 12 (i.e. 30 fragments for a full circle). The minimum allowed value is 0.01. Attempting to set a lower value causes a warning.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct FragmentAngle<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    preview: Option<crate::types::Angle>,
}

#[inline]
pub fn fragment_angle<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> FragmentAngle<DIMENSIONS> {
    FragmentAngle::default().angle(angle)
}

impl<const DIMENSIONS: usize> FragmentAngle<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }

    #[inline]
    pub fn preview<T: Into<crate::types::Angle>>(mut self, preview: T) -> Self {
        self.preview = Some(preview.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for FragmentAngle<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("fragment_angle");
        if let Some(value) = self.angle.as_ref() {
            s.field("v", value);
        }
        if let Some(value) = self.preview.as_ref() {
            s.field("p", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for FragmentAngle<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name =
            f.module("(v=12, p=undef) { $fa = ($preview && !is_undef(p)) ? p : v; children(); }");
        let args = [
            (
                "v",
                self.angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "p",
                self.preview
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for FragmentAngle<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for FragmentAngle<DIMENSIONS> {
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
    for FragmentAngle<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for FragmentAngle<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for FragmentAngle<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for FragmentAngle<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<FragmentAngle<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: FragmentAngle<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for FragmentAngle<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// The number of fragments and usually has the default value of 0.
///
/// When this variable has a value greater than zero, the `fragment_angle` and `fragment_size` are ignored, and a full circle is rendered using this number of fragments.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct FragmentCount<const DIMENSIONS: usize> {
    count: Option<crate::types::Scalar>,
    preview: Option<crate::types::Length>,
}

#[inline]
pub fn fragment_count<const DIMENSIONS: usize>(
    count: impl Into<crate::types::Scalar>,
) -> FragmentCount<DIMENSIONS> {
    FragmentCount::default().count(count)
}

impl<const DIMENSIONS: usize> FragmentCount<DIMENSIONS> {
    #[inline]
    pub fn count<T: Into<crate::types::Scalar>>(mut self, count: T) -> Self {
        self.count = Some(count.into());
        self
    }

    #[inline]
    pub fn preview<T: Into<crate::types::Length>>(mut self, preview: T) -> Self {
        self.preview = Some(preview.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for FragmentCount<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("fragment_count");
        if let Some(value) = self.count.as_ref() {
            s.field("v", value);
        }
        if let Some(value) = self.preview.as_ref() {
            s.field("p", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for FragmentCount<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name =
            f.module("(v=0, p=undef) { $fn = ($preview && !is_undef(p)) ? p : v; children(); }");
        let args = [
            (
                "v",
                self.count
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "p",
                self.preview
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for FragmentCount<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for FragmentCount<DIMENSIONS> {
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
    for FragmentCount<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for FragmentCount<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for FragmentCount<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for FragmentCount<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<FragmentCount<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: FragmentCount<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for FragmentCount<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// The minimum size of a fragment.
///
/// The default value is 2 so very small circles have a smaller number of fragments than specified using `fragment_angle`. The minimum allowed value is 0.01. Attempting to set a lower value causes a warning.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct FragmentSize<const DIMENSIONS: usize> {
    preview: Option<crate::types::Length>,
    size: Option<crate::types::Length>,
}

#[inline]
pub fn fragment_size<const DIMENSIONS: usize>(
    size: impl Into<crate::types::Length>,
) -> FragmentSize<DIMENSIONS> {
    FragmentSize::default().size(size)
}

impl<const DIMENSIONS: usize> FragmentSize<DIMENSIONS> {
    #[inline]
    pub fn preview<T: Into<crate::types::Length>>(mut self, preview: T) -> Self {
        self.preview = Some(preview.into());
        self
    }

    #[inline]
    pub fn size<T: Into<crate::types::Length>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for FragmentSize<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("fragment_size");
        if let Some(value) = self.preview.as_ref() {
            s.field("p", value);
        }
        if let Some(value) = self.size.as_ref() {
            s.field("v", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for FragmentSize<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name =
            f.module("(p=undef, v=2) { $fs = ($preview && !is_undef(p)) ? p : v; children(); }");
        let args = [
            (
                "p",
                self.preview
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "v",
                self.size
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for FragmentSize<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for FragmentSize<DIMENSIONS> {
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
    for FragmentSize<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for FragmentSize<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for FragmentSize<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for FragmentSize<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<FragmentSize<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: FragmentSize<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for FragmentSize<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
