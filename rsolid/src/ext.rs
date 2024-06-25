use crate::{
    extension as extensions, primitive as primitives,
    types::{Angle3, Length, Length3, Scalar3},
    IntoObject, Object,
};

pub trait ObjectExt<const DIMENSIONS: usize>: IntoObject<DIMENSIONS> + Sized {
    #[inline]
    fn union<B: IntoObject<DIMENSIONS>>(self, b: B) -> Object<DIMENSIONS> {
        crate::block::Block(self.into_object(), b.into_object()).into_object()
            >> primitives::union()
    }

    #[inline]
    fn difference<B: IntoObject<DIMENSIONS>>(self, b: B) -> Object<DIMENSIONS> {
        crate::block::Block(self.into_object(), b.into_object()).into_object()
            >> primitives::difference()
    }

    #[inline]
    fn intersection<B: IntoObject<DIMENSIONS>>(self, b: B) -> Object<DIMENSIONS> {
        crate::block::Block(self.into_object(), b.into_object()).into_object()
            >> primitives::intersection()
    }

    #[inline]
    fn minkowski<B: IntoObject<DIMENSIONS>>(self, b: B) -> Object<DIMENSIONS> {
        crate::block::Block(self.into_object(), b.into_object()).into_object()
            >> primitives::minkowski()
    }

    #[inline]
    fn up<Z: Into<Length>>(self, z: Z) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::up(z)
    }

    #[inline]
    fn down<Z: Into<Length>>(self, z: Z) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::down(z)
    }

    #[inline]
    fn right<X: Into<Length>>(self, x: X) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::right(x)
    }

    #[inline]
    fn left<X: Into<Length>>(self, x: X) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::left(x)
    }

    #[inline]
    fn fwd<Y: Into<Length>>(self, y: Y) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::fwd(y)
    }

    #[inline]
    fn back<Y: Into<Length>>(self, y: Y) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::back(y)
    }

    #[inline]
    fn scale<V: Into<Scalar3>>(self, v: V) -> Object<DIMENSIONS> {
        self.into_object() >> primitives::scale(v)
    }

    #[inline]
    fn resize<V: Into<Length3>>(self, v: V) -> Object<DIMENSIONS> {
        self.into_object() >> primitives::resize(v)
    }

    #[inline]
    fn translate<V: Into<Length3>>(self, v: V) -> Object<DIMENSIONS> {
        self.into_object() >> primitives::translate(v)
    }

    #[inline]
    fn rotate<V: Into<Angle3>>(self, v: V) -> Object<DIMENSIONS> {
        self.into_object() >> primitives::rotate(v)
    }

    /// Use this subtree as usual in the rendering process but also draw it unmodified in transparent pink.
    #[inline]
    fn dbg(self) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::dbg()
    }

    /// Ignore this subtree for the normal rendering process and draw it in transparent gray (all transformations are still applied to the nodes in this tree).
    ///
    /// Because the marked subtree is completely ignored, it might have unexpected effects in case it's used, for example, with the first object in a difference(). In that case this object is rendered in transparent gray, but it is not used as the base for the difference()!
    #[inline]
    fn bg(self) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::bg()
    }

    /// Ignore the rest of the design and use this subtree as design root.
    #[inline]
    fn root(self) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::root()
    }

    /// Simply ignore this entire subtree.
    #[inline]
    fn disable(self) -> Object<DIMENSIONS> {
        self.into_object() >> extensions::disable()
    }
}
