/// Rotates children around the X axis by the given number of degrees.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Xrot<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
}

/// Rotates children around the X axis by the given number of degrees.
#[inline]
pub fn xrot<const DIMENSIONS: usize>(angle: impl Into<crate::types::Angle>) -> Xrot<DIMENSIONS> {
    Xrot::default().angle(angle)
}

impl<const DIMENSIONS: usize> Xrot<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Xrot<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("xrot");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Xrot<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Display for Xrot<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Xrot<DIMENSIONS> {
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
    for Xrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Xrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Xrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Xrot<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Xrot<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Xrot<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Xrot<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the X axis by the given number of degrees at a centerpoint.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct XrotCp<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    centerpoint: Option<crate::types::Length3>,
}

#[inline]
pub fn xrot_cp<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> XrotCp<DIMENSIONS> {
    XrotCp::default().angle(angle)
}

impl<const DIMENSIONS: usize> XrotCp<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Debug for XrotCp<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("xrot_cp");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.centerpoint.as_ref() {
            s.field("cp", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for XrotCp<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Display for XrotCp<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for XrotCp<DIMENSIONS> {
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
    for XrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for XrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for XrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for XrotCp<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<XrotCp<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: XrotCp<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for XrotCp<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Y axis by the given number of degrees.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Yrot<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
}

/// Rotates children around the Y axis by the given number of degrees.
#[inline]
pub fn yrot<const DIMENSIONS: usize>(angle: impl Into<crate::types::Angle>) -> Yrot<DIMENSIONS> {
    Yrot::default().angle(angle)
}

impl<const DIMENSIONS: usize> Yrot<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Yrot<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("yrot");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Yrot<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Display for Yrot<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Yrot<DIMENSIONS> {
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
    for Yrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Yrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Yrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Yrot<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Yrot<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Yrot<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Yrot<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Y axis by the given number of degrees at a centerpoint.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct YrotCp<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    centerpoint: Option<crate::types::Length3>,
}

#[inline]
pub fn yrot_cp<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> YrotCp<DIMENSIONS> {
    YrotCp::default().angle(angle)
}

impl<const DIMENSIONS: usize> YrotCp<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Debug for YrotCp<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("yrot_cp");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.centerpoint.as_ref() {
            s.field("cp", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for YrotCp<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Display for YrotCp<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for YrotCp<DIMENSIONS> {
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
    for YrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for YrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for YrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for YrotCp<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<YrotCp<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: YrotCp<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for YrotCp<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Z axis by the given number of degrees.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Zrot<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
}

/// Rotates children around the Z axis by the given number of degrees.
#[inline]
pub fn zrot<const DIMENSIONS: usize>(angle: impl Into<crate::types::Angle>) -> Zrot<DIMENSIONS> {
    Zrot::default().angle(angle)
}

impl<const DIMENSIONS: usize> Zrot<DIMENSIONS> {
    #[inline]
    pub fn angle<T: Into<crate::types::Angle>>(mut self, angle: T) -> Self {
        self.angle = Some(angle.into());
        self
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Zrot<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("zrot");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Zrot<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Display for Zrot<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Zrot<DIMENSIONS> {
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
    for Zrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Zrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Zrot<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Zrot<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Zrot<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Zrot<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Zrot<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Rotates children around the Z axis by the given number of degrees at a centerpoint.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct ZrotCp<const DIMENSIONS: usize> {
    angle: Option<crate::types::Angle>,
    centerpoint: Option<crate::types::Length3>,
}

#[inline]
pub fn zrot_cp<const DIMENSIONS: usize>(
    angle: impl Into<crate::types::Angle>,
) -> ZrotCp<DIMENSIONS> {
    ZrotCp::default().angle(angle)
}

impl<const DIMENSIONS: usize> ZrotCp<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Debug for ZrotCp<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("zrot_cp");
        if let Some(value) = self.angle.as_ref() {
            s.field("a", value);
        }
        if let Some(value) = self.centerpoint.as_ref() {
            s.field("cp", value);
        }
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for ZrotCp<DIMENSIONS> {
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

impl<const DIMENSIONS: usize> ::core::fmt::Display for ZrotCp<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for ZrotCp<DIMENSIONS> {
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
    for ZrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for ZrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for ZrotCp<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for ZrotCp<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<ZrotCp<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: ZrotCp<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for ZrotCp<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
