function _v0() = 100;
function _v1() = 200;
function _v2() = [_v0(), _v1()];
module _v3() { square(size=_v2()); }
function _v4() = 400;
function _v5() = [_v4(), _v4()];
module _v6() { square(size=_v5()); }
module _v7 (a=0) { rotate([0, 0, a]) children(); }
function _v8() = 63.43494882292201;
module _v9() { _v7(a=_v8()) children(); }
module _v10() { _v9() _v6(); }
module _v11() { difference() { _v3(); _v10(); }; }
module _v12 (y=undef) { translate([0, -y, 0]) children(); }
module _v13() { _v12(y=_v0()) children(); }
module _v14() { _v13() _v11(); }
module _v15 (x=undef) { translate([-x, 0, 0]) children(); }
function _v16() = 50;
module _v17() { _v15(x=_v16()) children(); }
module _v18() { _v17() _v14(); }
function _v19() = 1;
function _v20() = 0;
function _v21() = [_v19(), _v20(), _v20()];
module _v22() { mirror(v=_v21()) children(); }
module _v23() { _v22() _v18(); }

_v23();