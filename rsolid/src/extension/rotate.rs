/// Rotates children around the X axis by the given number of degrees.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateX<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
}

/// Rotates children around the X axis by the given number of degrees.
#[inline]
pub fn rotate_x<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> RotateX<DIMENSIONS> {
    RotateX::default().angle(angle)
}

impl<const DIMENSIONS: usize> RotateX<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for RotateX<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_x");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for RotateX<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(a=0) { rotate([a, 0, 0]) children(); }");
        let args = [(
            "a",
            self.angle
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for RotateX<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for RotateX<DIMENSIONS> {
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
    for RotateX<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for RotateX<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for RotateX<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for RotateX<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<RotateX<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: RotateX<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for RotateX<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the X axis by the given number of degrees at a centerpoint.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateXAround<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    centerpoint: Option<crate::types::Length3>,
}

#[inline]
pub fn rotate_x_around<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> RotateXAround<DIMENSIONS> {
    RotateXAround::default().angle(angle)
}

impl<const DIMENSIONS: usize> RotateXAround<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }

    #[inline]
    pub fn centerpoint<T: Into<crate::types::Length3>>(mut self, centerpoint: T) -> Self {
        self.centerpoint = Some(centerpoint.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for RotateXAround<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_x_around");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.centerpoint.as_ref() {
            s.field("cp", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for RotateXAround<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(a=0, cp=undef) { if (!is_undef(cp)) { translate(cp) rotate([a, 0, 0]) translate(-cp) children(); } else { rotate([a, 0, 0]) children(); } }");
        let args = [
            (
                "a",
                self.angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "cp",
                self.centerpoint
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for RotateXAround<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for RotateXAround<DIMENSIONS> {
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
    for RotateXAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for RotateXAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for RotateXAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for RotateXAround<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<RotateXAround<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: RotateXAround<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for RotateXAround<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Y axis by the given number of degrees.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateY<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
}

/// Rotates children around the Y axis by the given number of degrees.
#[inline]
pub fn rotate_y<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> RotateY<DIMENSIONS> {
    RotateY::default().angle(angle)
}

impl<const DIMENSIONS: usize> RotateY<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for RotateY<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_y");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for RotateY<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(a=0) { rotate([0, a, 0]) children(); }");
        let args = [(
            "a",
            self.angle
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for RotateY<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for RotateY<DIMENSIONS> {
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
    for RotateY<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for RotateY<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for RotateY<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for RotateY<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<RotateY<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: RotateY<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for RotateY<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Y axis by the given number of degrees at a centerpoint.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateYAround<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    centerpoint: Option<crate::types::Length3>,
}

#[inline]
pub fn rotate_y_around<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> RotateYAround<DIMENSIONS> {
    RotateYAround::default().angle(angle)
}

impl<const DIMENSIONS: usize> RotateYAround<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }

    #[inline]
    pub fn centerpoint<T: Into<crate::types::Length3>>(mut self, centerpoint: T) -> Self {
        self.centerpoint = Some(centerpoint.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for RotateYAround<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_y_around");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.centerpoint.as_ref() {
            s.field("cp", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for RotateYAround<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(a=0, cp=undef) { if (!is_undef(cp)) { translate(cp) rotate([0, a, 0]) translate(-cp) children(); } else { rotate([0, a, 0]) children(); } }");
        let args = [
            (
                "a",
                self.angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "cp",
                self.centerpoint
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for RotateYAround<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for RotateYAround<DIMENSIONS> {
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
    for RotateYAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for RotateYAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for RotateYAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for RotateYAround<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<RotateYAround<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: RotateYAround<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for RotateYAround<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Z axis by the given number of degrees.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateZ<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
}

/// Rotates children around the Z axis by the given number of degrees.
#[inline]
pub fn rotate_z<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> RotateZ<DIMENSIONS> {
    RotateZ::default().angle(angle)
}

impl<const DIMENSIONS: usize> RotateZ<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for RotateZ<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_z");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for RotateZ<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(a=0) { rotate([0, 0, a]) children(); }");
        let args = [(
            "a",
            self.angle
                .as_ref()
                .map(|value| crate::scad::Scad::assign(value, f)),
        )];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for RotateZ<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for RotateZ<DIMENSIONS> {
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
    for RotateZ<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for RotateZ<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for RotateZ<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for RotateZ<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<RotateZ<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: RotateZ<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for RotateZ<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Z axis by the given number of degrees at a centerpoint.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct RotateZAround<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    centerpoint: Option<crate::types::Length3>,
}

#[inline]
pub fn rotate_z_around<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> RotateZAround<DIMENSIONS> {
    RotateZAround::default().angle(angle)
}

impl<const DIMENSIONS: usize> RotateZAround<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }

    #[inline]
    pub fn centerpoint<T: Into<crate::types::Length3>>(mut self, centerpoint: T) -> Self {
        self.centerpoint = Some(centerpoint.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for RotateZAround<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("rotate_z_around");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.centerpoint.as_ref() {
            s.field("cp", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for RotateZAround<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("(a=0, cp=undef) { if (!is_undef(cp)) { translate(cp) rotate([0, 0, a]) translate(-cp) children(); } else { rotate([0, 0, a]) children(); } }");
        let args = [
            (
                "a",
                self.angle
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "cp",
                self.centerpoint
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for RotateZAround<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for RotateZAround<DIMENSIONS> {
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
    for RotateZAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for RotateZAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for RotateZAround<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for RotateZAround<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<RotateZAround<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: RotateZAround<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for RotateZAround<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
