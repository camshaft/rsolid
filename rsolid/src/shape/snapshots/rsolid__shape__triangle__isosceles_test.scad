function _v0() = 3;
function _v1() = 57.735026918962575;
module _v2() { circle($fn=_v0(), r=_v1()); }
module _v3 (a=0) { rotate([0, 0, a]) children(); }
function _v4() = -30;
module _v5() { _v3(a=_v4()) children(); }
module _v6() { _v5() _v2(); }
module _v7 (y=undef) { translate([0, y, 0]) children(); }
function _v8() = -56.69872981077807;
module _v9() { _v7(y=_v8()) children(); }
module _v10() { _v9() _v6(); }
function _v11() = 1;
function _v12() = 2.309401076758503;
function _v13() = [_v11(), _v12(), _v11()];
module _v14() { scale(v=_v13()) children(); }
module _v15() { _v14() _v10(); }
module _v16 (y=undef) { translate([0, -y, 0]) children(); }
function _v17() = 100;
module _v18() { _v16(y=_v17()) children(); }
module _v19() { _v18() _v15(); }

_v19();