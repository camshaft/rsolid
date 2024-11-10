/// Makes a very simple screw model, useful for making screwholes.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Screw {
    countersunk: Option<bool>,
    head_len: Option<crate::types::Length>,
    head_size: Option<crate::types::Length>,
    len: Option<crate::types::Length>,
    pitch: Option<crate::types::Scalar>,
    size: Option<crate::types::Length>,
}

/// Makes a very simple screw model, useful for making screwholes.
#[inline]
pub fn screw() -> Screw {
    Screw::default()
}

impl Screw {
    /// If true, center from cap's top instead of it's bottom.
    #[inline]
    pub fn countersunk<T: Into<bool>>(mut self, countersunk: T) -> Self {
        self.countersunk = Some(countersunk.into());
        self
    }

    /// length of the screw head.
    #[inline]
    pub fn head_len<T: Into<crate::types::Length>>(mut self, head_len: T) -> Self {
        self.head_len = Some(head_len.into());
        self
    }

    /// diameter of the screw head.
    #[inline]
    pub fn head_size<T: Into<crate::types::Length>>(mut self, head_size: T) -> Self {
        self.head_size = Some(head_size.into());
        self
    }

    /// length of threaded part of screw.
    #[inline]
    pub fn len<T: Into<crate::types::Length>>(mut self, len: T) -> Self {
        self.len = Some(len.into());
        self
    }

    #[inline]
    pub fn pitch<T: Into<crate::types::Scalar>>(mut self, pitch: T) -> Self {
        self.pitch = Some(pitch.into());
        self
    }

    /// diameter of threaded part of screw.
    #[inline]
    pub fn size<T: Into<crate::types::Length>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl ::core::fmt::Debug for Screw {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("screw");
        if let Some(value) = self.countersunk.as_ref() {
            s.field("countersunk", value);
        }
        if let Some(value) = self.head_len.as_ref() {
            s.field("headlen", value);
        }
        if let Some(value) = self.head_size.as_ref() {
            s.field("headsize", value);
        }
        if let Some(value) = self.len.as_ref() {
            s.field("screwlen", value);
        }
        if let Some(value) = self.pitch.as_ref() {
            s.field("pitch", value);
        }
        if let Some(value) = self.size.as_ref() {
            s.field("screwsize", value);
        }
        s.finish()
    }
}

impl crate::scad::Scad for Screw {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        f.includes("<BOSL/constants.scad>");
        f.uses("<BOSL/metric_screws.scad>");
        let name = "screw";
        let args = [
            (
                "countersunk",
                self.countersunk
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "headlen",
                self.head_len
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "headsize",
                self.head_size
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "screwlen",
                self.len
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "pitch",
                self.pitch
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
            (
                "screwsize",
                self.size
                    .as_ref()
                    .map(|value| crate::scad::Scad::assign(value, f)),
            ),
        ];
        f.call(name, args, false)
    }
}

impl ::core::fmt::Display for Screw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Add<T> for Screw {
    type Output = crate::Object<3>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::Sub<T> for Screw {
    type Output = crate::Object<3>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<3>> ::core::ops::BitOr<T> for Screw {
    type Output = crate::Object<3>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<3>> ::core::ops::Shr<F> for Screw {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl From<Screw> for crate::Object<3> {
    #[inline]
    fn from(value: Screw) -> Self {
        crate::Object::new(value)
    }
}

impl crate::IntoObject<3> for Screw {
    #[inline]
    fn into_object(self) -> crate::Object<3> {
        crate::Object::new(self)
    }
}
