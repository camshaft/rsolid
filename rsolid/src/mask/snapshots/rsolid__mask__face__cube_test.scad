function _v0() = true;
function _v1() = [10,10];
module _v2() { square(center=_v0(), size=_v1()); }
function _v3() = 10;
module _v4() { circle(r=_v3()); }
module _v5 (y=undef) { translate([0, -y, 0]) children(); }
function _v6() = 5;
module _v7() { _v5(y=_v6()) children(); }
module _v8() { _v7() _v4(); }
module _v9 (x=undef) { translate([-x, 0, 0]) children(); }
module _v10() { _v9(x=_v6()) children(); }
module _v11() { _v10() _v8(); }
module _v12() { difference() { _v2(); _v11(); }; }
function _v13() = [10,0.01];
module _v14() { square(center=_v0(), size=_v13()); }
module _v15 (y=undef) { translate([0, y, 0]) children(); }
module _v16() { _v15(y=_v6()) children(); }
module _v17() { _v16() _v14(); }
function _v18() = [0.01,10];
module _v19() { square(center=_v0(), size=_v18()); }
module _v20 (x=undef) { translate([x, 0, 0]) children(); }
module _v21() { _v20(x=_v6()) children(); }
module _v22() { _v21() _v19(); }
module _v23() { union() { _v17(); _v22(); }; }
module _v24 () { if($preview) { children(); } }
module _v25() { _v24() _v23(); }
module _v26() { union() { _v12(); _v25(); }; }
module _v27() { _v7() _v26(); }
module _v28() { _v10() _v27(); }
function _v29() = 100;
module _v30() { linear_extrude(center=_v0(), height=_v29()) children(); }
module _v31() { _v30() _v28(); }
module _v32 (a=0) { rotate([a, 0, 0]) children(); }
function _v33() = 90;
module _v34() { _v32(a=_v33()) children(); }
module _v35() { _v34() _v31(); }
function _v36() = 25;
module _v37() { _v20(x=_v36()) children(); }
module _v38() { _v37() _v35(); }
function _v39() = [1,0,0];
module _v40() { mirror(v=_v39()) children(); }
module _v41() { _v40() _v38(); }
module _v42() { union() { _v38(); _v41(); }; }
function _v43() = 50;
module _v44() { linear_extrude(center=_v0(), height=_v43()) children(); }
module _v45() { _v44() _v28(); }
function _v46() = [90,0,90];
module _v47() { rotate(a=_v46()) children(); }
module _v48() { _v47() _v45(); }
module _v49() { _v15(y=_v43()) children(); }
module _v50() { _v49() _v48(); }
module _v51() { union() { _v42(); _v50(); }; }
function _v52() = [0,1,0];
module _v53() { mirror(v=_v52()) children(); }
module _v54() { _v53() _v50(); }
module _v55() { union() { _v51(); _v54(); }; }

_v55();