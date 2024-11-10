use core::fmt;
use std::collections::{BTreeMap, HashMap, HashSet};

pub trait Scad {
    fn assign(&self, f: &mut Formatter) -> Assignment;

    fn to_scad(&self) -> String {
        let mut formatter = Formatter::default();
        let assignment = self.assign(&mut formatter);
        format!("{formatter}\n{assignment};")
    }
}

pub trait Parameter: 'static + fmt::Display + fmt::Debug {}

#[derive(Debug, Default)]
pub struct Formatter {
    imports: Vec<Import>,
    // TODO
    #[allow(dead_code)]
    parameters: BTreeMap<String, Box<dyn Parameter>>,
    assignments: HashMap<String, Assignment>,
    assign_idx: usize,
    outputs: BTreeMap<String, Assignment>,
}

impl fmt::Display for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut seen_imports = HashSet::new();
        for i in &self.imports {
            if seen_imports.insert(i) {
                writeln!(f, "{}", i)?;
            }
        }

        let mut a: Vec<_> = self
            .assignments
            .iter()
            .map(|(value, var)| (var, value))
            .collect();

        a.sort_by_key(|(var, _value)| *var);

        for (var, value) in a {
            match var {
                Assignment::Var {
                    ty: AssignmentType::Call,
                    ..
                } => writeln!(f, "module {var} {{ {value}; }}")?,
                Assignment::Var {
                    ty: AssignmentType::Function,
                    ..
                } => writeln!(f, "function {var} = {value};")?,
                Assignment::Var {
                    ty: AssignmentType::Module,
                    ..
                } => writeln!(f, "module {var} {value}")?,
                Assignment::Inline { .. } => {}
            }
        }

        Ok(())
    }
}

impl Formatter {
    pub fn assign<V: Scad>(&mut self, v: V) -> Assignment {
        v.assign(self)
    }

    pub fn emit<V: fmt::Display>(&mut self, v: V, ty: AssignmentType) -> Assignment {
        self.assignments
            .entry(v.to_string())
            .or_insert_with(|| {
                let idx = self.assign_idx;
                self.assign_idx = idx + 1;
                Assignment::Var { idx, ty }
            })
            .clone()
    }

    pub fn value<V: fmt::Display>(&mut self, v: V) -> Assignment {
        self.emit(v, AssignmentType::Function)
    }

    pub fn module<V: fmt::Display>(&mut self, v: V) -> Assignment {
        self.emit(v, AssignmentType::Module)
    }

    pub fn uses<V: fmt::Display>(&mut self, v: V) {
        self.imports.push(Import::Use(v.to_string()))
    }

    pub fn includes<V: fmt::Display>(&mut self, v: V) {
        self.imports.push(Import::Include(v.to_string()))
    }

    pub fn call<'a, N: fmt::Display, A: IntoIterator<Item = (&'a str, Option<Assignment>)>>(
        &mut self,
        name: N,
        args: A,
        operator: bool,
    ) -> Assignment {
        let mut out = format!("{name}(");

        let mut first = true;
        for (name, value) in args {
            if let Some(value) = value {
                if !core::mem::take(&mut first) {
                    out.push_str(", ");
                }
                out.push_str(name);
                out.push('=');
                out.push_str(&value.to_string());
            }
        }
        out.push(')');

        if first {
            return Assignment::Inline { code: out };
        }

        if operator {
            out.push_str(" children()")
        }

        self.emit(out, AssignmentType::Call)
    }

    pub fn output(&mut self, name: impl Into<String>, assignment: Assignment) {
        self.outputs.insert(name.into(), assignment);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Assignment {
    Var { idx: usize, ty: AssignmentType },
    Inline { code: String },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssignmentType {
    Function,
    Call,
    Module,
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Assignment::Var {
                idx,
                ty: AssignmentType::Module,
            } => write!(f, "_v{idx}"),
            Assignment::Var { idx, .. } => write!(f, "_v{idx}()"),
            Assignment::Inline { code } => f.write_str(code),
        }
    }
}

impl Scad for Assignment {
    fn assign(&self, _f: &mut Formatter) -> Assignment {
        self.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
enum Import {
    Use(String),
    Include(String),
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Import::Use(path) => write!(f, "use {path};"),
            Import::Include(path) => write!(f, "include {path};"),
        }
    }
}
