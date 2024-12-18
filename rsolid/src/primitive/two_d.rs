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

/// The function polygon() creates a multiple sided shape from a list of x,y coordinates. A polygon is the most powerful 2D object. It can create anything that circle and squares can, as well as much more. This includes irregular shapes with both concave and convex edges. In addition it can place holes within that shape.
#[derive(Clone, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Polygon {
    convexity: Option<crate::types::Scalar>,
    paths: Option<crate::types::VecLength2>,
    points: Option<crate::types::VecLength2>,
}

#[inline]
pub fn polygon(points: impl Into<crate::types::VecLength2>) -> Polygon {
    Polygon::default().points(points)
}

impl Polygon {
    /// Integer number of "inward" curves, ie. expected path crossings of an arbitrary line through the polygon
    #[inline]
    pub fn convexity<T: Into<crate::types::Scalar>>(mut self, convexity: T) -> Self {
        self.convexity = Some(convexity.into());
        self
    }

    /// A closed shape is created by returning from the last point specified to the first.
    ///
    /// ### Default
    /// If no path is specified, all points are used in the order listed.
    ///
    /// ### Single Vector
    /// The order to traverse the points. Uses indices from 0 to n-1. May be in a different order and use all or part, of the points listed.
    ///
    /// ### Multiple Vectors
    /// Creates primary and secondary shapes. Secondary shapes are subtracted from the primary shape (like difference()). Secondary shapes may be wholly or partially within the primary shape.
    #[inline]
    pub fn paths<T: Into<crate::types::VecLength2>>(mut self, paths: T) -> Self {
        self.paths = Some(paths.into());
        self
    }

    /// The list of x,y points of the polygon. : A vector of 2 element vectors.
    /// Note: points are indexed from 0 to n-1.
    #[inline]
    pub fn points<T: Into<crate::types::VecLength2>>(mut self, points: T) -> Self {
        self.points = Some(points.into());
        self
    }
}

impl ::core::fmt::Debug for Polygon {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("polygon");
        if let Some(value) = self.convexity.as_ref() {
            s.field("convexity", value);
        }
        if let Some(value) = self.paths.as_ref() {
            s.field("paths", value);
        }
        if let Some(value) = self.points.as_ref() {
            s.field("points", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Polygon {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "polygon";
        let args = [
            (
                "convexity",
                self.convexity
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "paths",
                self.paths
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "points",
                self.points
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, false)
    }
}

impl ::core::fmt::Display for Polygon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Add<T> for Polygon {
    type Output = crate::Object<2>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Sub<T> for Polygon {
    type Output = crate::Object<2>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::BitOr<T> for Polygon {
    type Output = crate::Object<2>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<2>> ::core::ops::Shr<F> for Polygon {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Polygon> for crate::Object<2> {
    #[inline]
    fn from(value: Polygon) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<2> for Polygon {
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

///
#[derive(Clone, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Text {
    font: Option<String>,
    halign: Option<String>,
    size: Option<crate::types::Scalar>,
    text: Option<String>,
    valign: Option<String>,
}

#[inline]
pub fn text(text: impl Into<String>) -> Text {
    Text::default().text(text)
}

impl Text {
    /// The name of the font that should be used. This is not the name of the font file, but the logical font name (internally handled by the fontconfig library). This can also include a style parameter, see below. A list of installed fonts & styles can be obtained using the font list dialog (Help -> Font List).
    #[inline]
    pub fn font<T: Into<String>>(mut self, font: T) -> Self {
        self.font = Some(font.into());
        self
    }

    /// The horizontal alignment for the text. Possible values are "left", "center" and "right". Default is "left".
    #[inline]
    pub fn halign<T: Into<String>>(mut self, halign: T) -> Self {
        self.halign = Some(halign.into());
        self
    }

    /// The generated text has an ascent (height above the baseline) of approximately the given value. Default is 10. Different fonts can vary somewhat and may not fill the size specified exactly, typically they render slightly smaller. On a metric system a size of 25.4 (1" imperial) will correspond to 100pt ⇒ a 12pt font size would be 12×0.254 for metric conversion or 0.12 in imperial.
    #[inline]
    pub fn size<T: Into<crate::types::Scalar>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }

    /// The text to generate.
    #[inline]
    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = Some(text.into());
        self
    }

    /// The vertical alignment for the text. Possible values are "top", "center", "baseline" and "bottom". Default is "baseline".
    #[inline]
    pub fn valign<T: Into<String>>(mut self, valign: T) -> Self {
        self.valign = Some(valign.into());
        self
    }
}

impl ::core::fmt::Debug for Text {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("text");
        if let Some(value) = self.font.as_ref() {
            s.field("font", value);
        }
        if let Some(value) = self.halign.as_ref() {
            s.field("halign", value);
        }
        if let Some(value) = self.size.as_ref() {
            s.field("size", value);
        }
        if let Some(value) = self.text.as_ref() {
            s.field("text", value);
        }
        if let Some(value) = self.valign.as_ref() {
            s.field("valign", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Text {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "text";
        let args = [
            (
                "font",
                self.font
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "halign",
                self.halign
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "size",
                self.size
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "text",
                self.text
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "valign",
                self.valign
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, false)
    }
}

impl ::core::fmt::Display for Text {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Add<T> for Text {
    type Output = crate::Object<2>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::Sub<T> for Text {
    type Output = crate::Object<2>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<2>> ::core::ops::BitOr<T> for Text {
    type Output = crate::Object<2>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<2>> ::core::ops::Shr<F> for Text {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Text> for crate::Object<2> {
    #[inline]
    fn from(value: Text) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<2> for Text {
    #[inline]
    fn into_object(self) -> crate::Object<2> {
        crate::Object::new(self)
    }
}
