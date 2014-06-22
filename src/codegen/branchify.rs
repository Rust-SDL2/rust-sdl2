#![macro_escape]

use std::io::BufferedWriter;
use std::io::{File, Writer};
use std::str::Chars;
use std::vec::Vec;

struct ParseBranch {
    matches: Vec<u8>,
    result: Option<String>,
    children: Vec<ParseBranch>,
}

impl ParseBranch {
    fn new() -> ParseBranch {
        ParseBranch {
            matches: Vec::new(),
            result: None,
            children: Vec::new(),
        }
    }
}

#[allow(visible_private_types)]
pub fn branchify(options: &[(&str, &str)], case_sensitive: bool) -> Vec<ParseBranch> {
    let mut root = ParseBranch::new();

    fn go_down_moses(branch: &mut ParseBranch, mut chariter: Chars, result: &str, case_sensitive: bool) {
        match chariter.next() {
            Some(c) => {
                let first_case = if case_sensitive { c as u8 } else { c.to_ascii().to_uppercase().to_byte() };
                for next_branch in branch.children.mut_iter() {
                    if next_branch.matches.as_slice()[0] == first_case {
                        go_down_moses(next_branch, chariter, result, case_sensitive);
                        return;
                    }
                }
                let mut subbranch = ParseBranch::new();
                subbranch.matches.push(first_case);
                if !case_sensitive {
                    let second_case = c.to_ascii().to_lowercase().to_byte();
                    if first_case != second_case {
                        subbranch.matches.push(second_case);
                    }
                }
                branch.children.push(subbranch);
                let index = branch.children.len() -1;
                go_down_moses(&mut branch.children.as_mut_slice()[index], chariter, result, case_sensitive);
            },
            None => {
                assert!(branch.result.is_none());
                branch.result = Some(result.to_string());
            },
        }
    };

    for &(key, result) in options.iter() {
        go_down_moses(&mut root, key.chars(), result, case_sensitive);
    }

    root.children
}

macro_rules! branchify(
    (case sensitive, $($key:expr => $value:ident),*) => (
        ::branchify::branchify([$(($key, stringify!($value))),*], true)
    );
    (case insensitive, $($key:expr => $value:ident),*) => (
        branchify([$(($key, stringify!($value))),*], false)
    );
)

/// Prints the contents to stdout.
///
/// :param branches: the branches to search through
/// :param indent: the level of indentation (each level representing four leading spaces)
/// :param read_call: the function call to read a byte
/// :param end: the byte which marks the end of the sequence
/// :param max_len: the maximum length a value may be before giving up and returning ``None``
/// :param valid: the function call to if a byte ``b`` is valid
/// :param unknown: the expression to call for an unknown value; in this string, ``{}`` will be
///         replaced with an expression (literal or non-literal) evaluating to a ``~str`` (it is
///         ``{}`` only, not arbitrary format strings)
#[allow(visible_private_types)]
pub fn generate_branchified_method(
        writer: &mut BufferedWriter<File>,
        branches: &[ParseBranch],
        indent: uint,
        read_call: &str,
        end: &str,
        max_len: &str,
        valid: &str,
        unknown: &str) {

    // Write Formatted
    macro_rules! wf(($($x:tt)*) => ({
        let indentstr = " ".repeat(indent * 4);
        let s = format!($($x)*);
        let result = writer.write(indentstr.as_bytes())
        .and(writer.write(s.as_bytes()))
        .and(writer.write(b"\n"));

        match result {
            Ok(_)  => {},
            Err(e) => fail!("write error: {:s}", e.desc),
        }
    }))

    fn r(writer: &mut BufferedWriter<File>, branch: &ParseBranch, prefix: &str, indent: uint, read_call: &str,
            end: &str, max_len: &str, valid: &str, unknown: &str) {
        for &c in branch.matches.iter() {
            let next_prefix = format!("{}{}", prefix, c as char);
            wf!("Some(b) if b == '{}' as u8 => match {} {{", c as char, read_call);
            for b in branch.children.iter() {
                r(writer, b, next_prefix.as_slice(), indent + 1, read_call, end, max_len, valid, unknown);
            }
            match branch.result {
                Some(ref result) => wf!("    Some(b) if b == SP => return Some({}),", *result),
                None => wf!("    Some(b) if b == SP => return Some({}),",
                                unknown.replace("{}", format!("~\"{}\"", next_prefix.as_slice()).as_slice())),
            }
            wf!("    Some(b) if {} => (\"{}\", b),", valid, next_prefix.as_slice());
            wf!("    _ => return None,");
            wf!("}},");
        }
    }
    wf!("let (s, next_byte) = match {} {{", read_call);
    for b in branches.iter() {
        r(writer, b, "", indent + 1, read_call, end, max_len, valid, unknown);
    }
    wf!("    Some(b) if {} => (\"\", b),", valid);
    wf!("    _ => return None,");
    wf!("}};");
    wf!("// OK, that didn't pan out. Let's read the rest and see what we get.");
    wf!("let mut s = s.to_string();");
    wf!("s.push_char(next_byte as char);");
    wf!("loop {{");
    wf!("    match {} {{", read_call);
    wf!("        Some(b) if b == {} => return Some({}),", end, unknown.replace("{}", "s"));
    wf!("        Some(b) if {} => {{", valid);
    wf!("            if s.len() == {} {{", max_len);
    wf!("                // Too long; bad request");
    wf!("                return None;");
    wf!("            }}");
    wf!("            s.push_char(b as char);");
    wf!("        }},");
    wf!("        _ => return None,");
    wf!("    }}");
    wf!("}}");
}
