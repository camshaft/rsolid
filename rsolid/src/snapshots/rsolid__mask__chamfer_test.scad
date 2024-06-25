function _v0() = 50;
function _v1() = 100;
function _v2() = [_v0(), _v1()];
module _v3() { square(size=_v2()); }
function _v4() = 200;
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
function _v16() = 25;
module _v17() { _v15(x=_v16()) children(); }
module _v18() { _v17() _v14(); }
function _v19() = 1;
function _v20() = 0;
function _v21() = [_v19(), _v20(), _v20()];
module _v22() { mirror(v=_v21()) children(); }
module _v23() { _v22() _v18(); }
function _v24() = 180;
module _v25() { _v7(a=_v24()) children(); }
module _v26() { _v25() _v23(); }
function _v27() = true;
function _v28() = 0.01;
function _v29() = [_v0(), _v28()];
module _v30() { square(center=_v27(), size=_v29()); }
module _v31 (y=undef) { translate([0, y, 0]) children(); }
module _v32() { _v31(y=_v0()) children(); }
module _v33() { _v32() _v30(); }
function _v34() = [_v28(), _v1()];
module _v35() { square(center=_v27(), size=_v34()); }
module _v36 (x=undef) { translate([x, 0, 0]) children(); }
module _v37() { _v36(x=_v16()) children(); }
module _v38() { _v37() _v35(); }
module _v39() { union() { _v33(); _v38(); }; }
module _v40 () { if($preview) { children(); } }
module _v41() { _v40() _v39(); }
module _v42() { union() { _v26(); _v41(); }; }
module _v43() { _v13() _v42(); }
module _v44() { _v17() _v43(); }

_v44();