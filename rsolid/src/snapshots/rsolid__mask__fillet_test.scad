function _v0() = true;
function _v1() = 100;
function _v2() = [_v1(), _v1()];
module _v3() { square(center=_v0(), size=_v2()); }
module _v4() { circle(r=_v1()); }
module _v5 (y=undef) { translate([0, -y, 0]) children(); }
function _v6() = 50;
module _v7() { _v5(y=_v6()) children(); }
module _v8() { _v7() _v4(); }
module _v9 (x=undef) { translate([-x, 0, 0]) children(); }
module _v10() { _v9(x=_v6()) children(); }
module _v11() { _v10() _v8(); }
module _v12() { difference() { _v3(); _v11(); }; }
function _v13() = 0.01;
function _v14() = [_v1(), _v13()];
module _v15() { square(center=_v0(), size=_v14()); }
module _v16 (y=undef) { translate([0, y, 0]) children(); }
module _v17() { _v16(y=_v6()) children(); }
module _v18() { _v17() _v15(); }
function _v19() = [_v13(), _v1()];
module _v20() { square(center=_v0(), size=_v19()); }
module _v21 (x=undef) { translate([x, 0, 0]) children(); }
module _v22() { _v21(x=_v6()) children(); }
module _v23() { _v22() _v20(); }
module _v24() { union() { _v18(); _v23(); }; }
module _v25 () { if($preview) { children(); } }
module _v26() { _v25() _v24(); }
module _v27() { union() { _v12(); _v26(); }; }
module _v28() { _v7() _v27(); }
module _v29() { _v10() _v28(); }

_v29();