use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
use crate::v17::ast::*;
use pretty::RcDoc;

impl PrettyPrint for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Void => RcDoc::text("void"),
            Ty::Int => RcDoc::text("int"),
            Ty::Width(w) => match w {
                0 => panic!("Error: width must be greater than zero"),
                1 => RcDoc::nil(),
                n => RcDoc::text("[")
                    .append(RcDoc::as_string(n - 1))
                    .append(RcDoc::text(":"))
                    .append(RcDoc::text("0"))
                    .append(RcDoc::text("]")),
            },
        }
    }
}

impl PrettyPrint for Function {
    fn to_doc(&self) -> RcDoc<()> {
        let input_doc = if self.inputs().is_empty() {
            RcDoc::nil()
        } else {
            let mut doc = RcDoc::nil();
            for port in self.inputs().iter() {
                doc = doc
                    .append(RcDoc::hardline())
                    .append(port.to_doc())
                    .append(RcDoc::text(";"));
            }
            doc = doc.append(RcDoc::hardline()).nest(PRETTY_INDENT);
            doc
        };
        let body_doc = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            let mut doc = RcDoc::nil();
            for seq in self.body().iter() {
                doc = doc
                    .append(RcDoc::hardline())
                    .append(seq.to_doc())
                    .append(RcDoc::text(";"));
            }
            doc = doc.append(RcDoc::hardline()).nest(PRETTY_INDENT);
            RcDoc::text("begin").append(doc).append(RcDoc::text("end"))
        };
        RcDoc::text("function")
            .append(RcDoc::space())
            .append(self.ret.to_doc())
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::text(";"))
            .append(input_doc)
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("endfunction"))
    }
}

impl PrettyPrint for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Int(name, ty) => ty
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::as_string(name)),
            Decl::Logic(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("logic")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
            Decl::Func(function) => function.to_doc(),
            Decl::Param(name, ty, expr) => RcDoc::text("parameter")
                .append(RcDoc::space())
                .append(ty.to_doc())
                .append(RcDoc::space())
                .append(RcDoc::as_string(name))
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(expr.to_doc()),
        }
    }
}

impl PrettyPrint for Sequential {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Sequential::Error(msg) => RcDoc::text("$")
                .append(RcDoc::text("error"))
                .append(RcDoc::text("("))
                .append(RcDoc::text(r#"""#))
                .append(RcDoc::as_string(msg))
                .append(RcDoc::text(r#"""#))
                .append(RcDoc::text(")")),
            Sequential::Assign(id, expr, ty) => RcDoc::as_string(id)
                .append(RcDoc::space())
                .append(ty.to_doc())
                .append(RcDoc::space())
                .append(expr.to_doc()),
            Sequential::Event(ty, expr) => ty.to_doc().append(RcDoc::space()).append(expr.to_doc()),
            Sequential::Assert(expr, branch) => {
                let cond = RcDoc::text("assert")
                    .append(RcDoc::text("("))
                    .append(expr.to_doc())
                    .append(RcDoc::text(")"));
                if let Some(block) = branch {
                    cond.append(RcDoc::space())
                        .append(RcDoc::text("else"))
                        .append(RcDoc::space())
                        .append(block.to_doc())
                } else {
                    cond
                }
            }
            Sequential::If(expr, _, _) => RcDoc::text("if")
                .append(RcDoc::space())
                .append(RcDoc::text("("))
                .append(expr.to_doc())
                .append(RcDoc::text(")")),
        }
    }
}

impl PrettyPrint for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Inst(ty) => ty.to_doc(),
            Parallel::Assign => RcDoc::text("assign"),
            Parallel::AlwaysComb(_) => RcDoc::text("always_comb"),
            Parallel::AlwaysFF(_, _) => RcDoc::text("always_ff"),
        }
    }
}

impl PrettyPrint for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Decl(decl) => decl.to_doc(),
            Stmt::Parallel(par) => par.to_doc(),
        }
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input(decl) => RcDoc::text("input")
                .append(RcDoc::space())
                .append(decl.to_doc()),
            Port::Output(decl) => RcDoc::text("output")
                .append(RcDoc::space())
                .append(decl.to_doc()),
        }
    }
}

impl PrettyPrint for Module {
    fn to_doc(&self) -> RcDoc<()> {
        let body_doc = if self.body().is_empty() {
            RcDoc::nil()
        } else {
            let mut doc = RcDoc::nil();
            for stmt in self.body().iter() {
                doc = doc
                    .append(RcDoc::hardline())
                    .append(RcDoc::hardline())
                    .append(stmt.to_doc());
                if let Stmt::Decl(Decl::Func(_)) = stmt {
                    doc = doc;
                } else {
                    doc = doc.append(RcDoc::text(";"));
                }
            }
            doc = doc.append(RcDoc::hardline()).nest(PRETTY_INDENT);
            doc
        };
        let mut ports_doc = if self.ports().is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::hardline().append(RcDoc::intersperse(
                self.ports().iter().map(|p| p.to_doc()),
                RcDoc::text(",").append(RcDoc::hardline()),
            ))
        };
        ports_doc = ports_doc.nest(PRETTY_INDENT);
        RcDoc::text("module")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name()))
            .append(RcDoc::space())
            .append(RcDoc::text("("))
            .append(ports_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::text(";"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("endmodule"))
            .append(RcDoc::hardline())
    }
}
