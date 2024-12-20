use crate::types as t;

#[derive(Clone, Debug, Default)]
pub struct Points {
    points: Vec<t::Length2>,
}

impl Points {
    #[inline]
    pub fn extend(&mut self, points: impl IntoIterator<Item = t::Length2>) {
        let mut points = points.into_iter();
        let (lower, upper) = points.size_hint();
        let mut remaining = upper.unwrap_or(lower);
        self.points.reserve(remaining);

        // force unwinding
        while remaining >= 4 {
            let a = points.next().unwrap();
            let b = points.next().unwrap();
            let c = points.next().unwrap();
            let d = points.next().unwrap();
            self.push(a);
            self.push(b);
            self.push(c);
            self.push(d);
            remaining -= 4;
        }

        for point in points {
            self.push(point);
        }
    }

    #[inline]
    pub fn push(&mut self, target: t::Length2) {
        // dedup points
        if let Some(last) = self.points.last() {
            if *last == target {
                return;
            }
        }
        self.points.push(target);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl crate::scad::Scad for Points {
    #[inline]
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        self.points.assign(f)
    }
}
