function _v0() = [50,100];
module _v1() { square(size=_v0()); }
function _v2() = [200,200];
module _v3() { square(size=_v2()); }
module _v4 (a=0) { rotate([0, 0, a]) children(); }
function _v5() = 63.43494882292201;
module _v6() { _v4(a=_v5()) children(); }
module _v7() { _v6() _v3(); }
module _v8() { difference() { _v1(); _v7(); }; }
module _v9 (y=undef) { translate([0, -y, 0]) children(); }
function _v10() = 50;
module _v11() { _v9(y=_v10()) children(); }
module _v12() { _v11() _v8(); }
module _v13 (x=undef) { translate([-x, 0, 0]) children(); }
function _v14() = 25;
module _v15() { _v13(x=_v14()) children(); }
module _v16() { _v15() _v12(); }
function _v17() = [1,0,0];
module _v18() { mirror(v=_v17()) children(); }
module _v19() { _v18() _v16(); }
function _v20() = 180;
module _v21() { _v4(a=_v20()) children(); }
module _v22() { _v21() _v19(); }
function _v23() = true;
function _v24() = [50,0.01];
module _v25() { square(center=_v23(), size=_v24()); }
module _v26 (y=undef) { translate([0, y, 0]) children(); }
module _v27() { _v26(y=_v10()) children(); }
module _v28() { _v27() _v25(); }
function _v29() = [0.01,100];
module _v30() { square(center=_v23(), size=_v29()); }
module _v31 (x=undef) { translate([x, 0, 0]) children(); }
module _v32() { _v31(x=_v14()) children(); }
module _v33() { _v32() _v30(); }
module _v34() { union() { _v28(); _v33(); }; }
module _v35 () { if($preview) { children(); } }
module _v36() { _v35() _v34(); }
module _v37() { union() { _v22(); _v36(); }; }
module _v38() { _v11() _v37(); }
module _v39() { _v15() _v38(); }

_v39();