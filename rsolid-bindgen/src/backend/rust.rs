use crate::definition::*;
use heck::ToPascalCase;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    /// Is this being generated in the main crate
    pub main_crate: bool,
}

impl Options {
    fn rsolid(&self) -> &'static str {
        if self.main_crate {
            "crate"
        } else {
            "rsolid"
        }
    }

    fn translate_type(&self, ty: Type) -> String {
        let ty = match ty {
            Type::Bool => return "bool".into(),
            Type::String => return "String".into(),
            Type::Length => "Length",
            Type::Length2 => "Length2",
            Type::Length3 => "Length3",
            Type::Scalar => "Scalar",
            Type::Scalar2 => "Scalar2",
            Type::Scalar3 => "Scalar3",
            Type::Angle => "Angle",
            Type::Angle2 => "Angle2",
            Type::Angle3 => "Angle3",
            Type::FragmentResolution => "FragmentResolution",
        };
        let rsolid = self.rsolid();
        format!("{rsolid}::types::{ty}")
    }
}

pub fn generate<O: Write>(options: &Options, defs: &[Definitions], out: &mut O) -> io::Result<()> {
    macro_rules! w {
        ($($tt:tt)*) => {
            writeln!(out, $($tt)*)?;
        }
    }

    let rsolid = options.rsolid();

    for def in defs {
        for (name, m) in def.modules.iter() {
            let upper = name.to_pascal_case();
            let (dim_gen, dim_gen_constraint, dim_gen_arg) = if m.dimensions == 0 {
                (
                    "const DIMENSIONS: usize, ",
                    "<const DIMENSIONS: usize>",
                    "<DIMENSIONS>",
                )
            } else {
                ("", "", "")
            };

            let is_copy = m.parameters.values().all(|p| !matches!(p.ty, Type::String));

            for line in m.docs.lines() {
                w!("/// {line}");
            }

            if is_copy {
                w!("{}", "#[derive(Clone, Copy, Default)]");
            } else {
                w!("{}", "#[derive(Clone, Default)]");
            }
            w!("#[must_use = \"Objects must be returned in order to be rendered\"]");
            w!("pub struct {upper} {dim_gen_constraint} {{");
            for (p_name, param) in m.parameters.iter() {
                let ty = options.translate_type(param.ty);
                w!("    {p_name}: Option<{ty}>,");
            }
            w!("}}");
            w!();

            static DEFAULT_CONSTRUCTOR: &str = "_default";

            if !m.constructors.contains_key(DEFAULT_CONSTRUCTOR) {
                for line in m.docs.lines() {
                    w!("/// {line}");
                }
                w!("#[inline]");
                w!("pub fn {name} {dim_gen_constraint} (");

                // if we only have one argument then just make it the default
                if m.parameters.len() == 1 {
                    for (arg, param) in &m.parameters {
                        let ty = options.translate_type(param.ty);
                        w!("    {arg}: impl Into<{ty}>,");
                    }
                }

                w!(") -> {upper} {dim_gen_arg} {{");
                w!("    {upper}::default()");
                if m.parameters.len() == 1 {
                    for arg in m.parameters.keys() {
                        w!("        .{arg}({arg})");
                    }
                }
                w!("}}");
                w!();
            }

            for (cname, c) in &m.constructors {
                let cname = if cname == DEFAULT_CONSTRUCTOR {
                    name
                } else {
                    cname
                };
                for line in c.docs.lines() {
                    w!("/// {line}");
                }
                w!("#[inline]");
                w!("pub fn {cname}{dim_gen_constraint}(");
                for arg in &c.arguments {
                    let param = m
                        .parameters
                        .get(arg)
                        .unwrap_or_else(|| panic!("invalid argument {arg} in constructor {cname}"));
                    let ty = options.translate_type(param.ty);
                    w!("    {arg}: impl Into<{ty}>,");
                }
                w!(") -> {upper} {dim_gen_arg} {{");
                w!("    {upper}::default()");
                for arg in &c.arguments {
                    w!("        .{arg}({arg})");
                }
                w!("}}");
                w!();
            }

            w!("impl{dim_gen_constraint} {upper}{dim_gen_arg} {{");
            {
                let mut first = true;
                for (p_name, param) in m.parameters.iter() {
                    if !core::mem::take(&mut first) {
                        w!();
                    }
                    let ty = options.translate_type(param.ty);
                    for line in param.docs.lines() {
                        w!("    /// {line}");
                    }
                    w!("    #[inline]");
                    w!("    pub fn {p_name}<T: Into<{ty}>>(mut self, {p_name}: T) -> Self {{");
                    w!("        self.{p_name} = Some({p_name}.into());");
                    w!("        self");
                    w!("    }}");
                }
            }

            w!("}}");
            w!();

            w!("impl{dim_gen_constraint} ::core::fmt::Debug for {upper}{dim_gen_arg} {{");
            w!("    #[allow(clippy::write_literal)]");
            w!("    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{");
            w!("        let mut s = f.debug_struct({name:?});");
            for (p_name, param) in m.parameters.iter() {
                w!("        if let Some(value) = self.{p_name}.as_ref() {{");
                let scad_name = param.name.as_ref().unwrap_or(p_name);
                w!("             s.field({scad_name:?}, value);");
                w!("        }}");
            }
            w!("        s.finish()");
            w!("    }}");
            w!("}}");
            w!();

            w!("impl{dim_gen_constraint} {rsolid}::scad::Scad for {upper}{dim_gen_arg} {{");
            w!("    fn assign(&self, f: &mut {rsolid}::scad::Formatter) -> {rsolid}::scad::Assignment {{");

            for inc in &m.imports {
                if let Some(inc) = inc.strip_prefix("use ") {
                    w!("        f.uses({inc:?});");
                } else if let Some(inc) = inc.strip_prefix("include ") {
                    w!("        f.includes({inc:?});");
                } else {
                    w!("        f.uses({inc:?});");
                }
            }

            if let Some(code) = m.code.as_ref() {
                let mut out = "(".to_string();

                for (idx, (p_name, param)) in m.parameters.iter().enumerate() {
                    let p_name = param.name.as_deref().unwrap_or(p_name);

                    if idx > 0 {
                        out += ", ";
                    }

                    if let Some(default) = param.default.as_ref() {
                        out += &format!("{p_name}={default}");
                    } else {
                        out += &format!("{p_name}=undef");
                    }
                }

                out += ") { ";
                out += code.replace('\n', " ").trim();
                out += " }";

                w!("        let name = f.module({out:?});");
            } else {
                w!("        let name = {name:?};");
            };
            w!("        let args = [");
            for (p_name, param) in m.parameters.iter() {
                let scad_name = param.name.as_ref().unwrap_or(p_name);
                w!("        ({scad_name:?}, self.{p_name}.as_ref().map(|value| {{");
                w!("            {rsolid}::scad::Scad::assign(value, f)");
                w!("        }})),");
            }
            w!("        ];");
            w!("        f.call(name, args, {})", m.operator);
            w!("    }}");
            w!("}}");
            w!();

            w!("impl{dim_gen_constraint} ::core::fmt::Display for {upper}{dim_gen_arg} {{");
            w!("    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{");
            w!("        f.write_str(&{rsolid}::scad::Scad::to_scad(self))");
            w!("    }}");
            w!("}}");
            w!();

            let (dim_out, dim_in) = if m.dimensions == 0 {
                ("DIMENSIONS".to_string(), "DIMENSIONS".to_string())
            } else {
                let dim_in = m.dimensions_in.unwrap_or(m.dimensions).to_string();
                (m.dimensions.to_string(), dim_in)
            };

            if m.operator {
                w!("impl {dim_gen_constraint} {rsolid}::Operator<{dim_in}> for {upper} {dim_gen_arg} {{");
                w!("    type Output = {rsolid}::Object<{dim_out}>;");
                w!();
                w!("    fn apply(self, child: &{rsolid}::Object<{dim_in}>) -> Self::Output {{");
                w!("        let obj: {rsolid}::operator::Wrapped<{dim_in}, {dim_out}> = {rsolid}::operator::Wrapped {{ parent: self.into(), child: child.clone() }};");
                w!("        {rsolid}::Object::new(obj)");
                w!("    }}");
                w!("}}");
                w!();
            }

            for (op_upper, op_lower) in [("Add", "add"), ("Sub", "sub"), ("BitOr", "bitor")] {
                w!("impl<T: {rsolid}::IntoObject<{dim_out}>, {dim_gen}> ::core::ops::{op_upper}<T> for {upper} {dim_gen_arg} {{");
                w!("    type Output = {rsolid}::Object<{dim_out}>;");
                w!();
                w!("    fn {op_lower}(self, other: T) -> Self::Output {{");
                w!("        use {rsolid}::IntoObject as _;");
                w!("        self.into_object().{op_lower}(other.into_object())");
                w!("    }}");
                w!("}}");
                w!();
            }

            w!("impl<F: {rsolid}::Operator<{dim_out}>, {dim_gen}> ::core::ops::Shr<F> for {upper} {dim_gen_arg} {{");
            w!("    type Output = F::Output;");
            w!();
            w!("    fn shr(self, f: F) -> Self::Output {{");
            w!("        use {rsolid}::IntoObject as _;");
            w!("        self.into_object() >> f");
            w!("    }}");
            w!("}}");
            w!();

            w!("impl<{dim_gen}> From<{upper} {dim_gen_arg}> for {rsolid}::Object<{dim_out}> {{");
            w!("    #[inline]");
            w!("    fn from(value: {upper} {dim_gen_arg}) -> Self {{");
            w!("        {rsolid}::Object::new(value)");
            w!("    }}");
            w!("}}");
            w!();

            w!("impl<{dim_gen}> {rsolid}::IntoObject<{dim_out}> for {upper} {dim_gen_arg} {{");
            w!("    #[inline]");
            w!("    fn into_object(self) -> {rsolid}::Object<{dim_out}> {{");
            w!("        {rsolid}::Object::new(self)");
            w!("    }}");
            w!("}}");
            w!();
        }
    }

    Ok(())
}
