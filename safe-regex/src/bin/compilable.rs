use safe_regex::internal::escape_ascii;

/// This compiles in a few seconds.
///
/// ```
/// $ cargo clean
/// $ time cargo run --package safe-regex --bin compilable --release
///    Compiling safe-proc-macro2 v1.0.24 (safe-regex-rs/safe-proc-macro2)
///    Compiling unicode-xid v0.2.1
///    Compiling safe-quote v1.0.9 (safe-regex-rs/safe-quote)
///    Compiling safe-regex-compiler v0.1.1 (safe-regex-rs/safe-regex-compiler)
///    Compiling safe-regex-macro v0.1.1 (safe-regex-rs/safe-regex-macro)
///    Compiling safe-regex v0.1.1 (safe-regex-rs/safe-regex)
///     Finished release [optimized] target(s) in 5.56s
///      Running `safe-regex-rs/target/release/compilable`
/// Matcher(br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa").match_all("aaaaaaaaaaaaaaaaaaaa") -> Some(Groups { ranges: [], data: [97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97] })
///
/// real	0m5.856s
/// user	0m7.660s
/// sys	0m0.772s
/// ```
fn main() {
    let re = {
        use safe_regex::internal::InputByte;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct Ranges_;
        impl Ranges_ {
            pub fn new() -> Self {
                Self
            }
            pub fn inner(&self) -> &[core::ops::Range<u32>; 0usize] {
                &[]
            }
        }
        type States_ =
            std::collections::HashSet<CompiledRegex_, std::collections::hash_map::RandomState>;
        #[doc = "br\"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa\""]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum CompiledRegex_ {
            Byte59(Ranges_),
            Byte58(Ranges_),
            Byte57(Ranges_),
            Byte56(Ranges_),
            Byte55(Ranges_),
            Byte54(Ranges_),
            Byte53(Ranges_),
            Byte52(Ranges_),
            Byte51(Ranges_),
            Byte50(Ranges_),
            Byte49(Ranges_),
            Byte48(Ranges_),
            Byte47(Ranges_),
            Byte46(Ranges_),
            Byte45(Ranges_),
            Byte44(Ranges_),
            Byte43(Ranges_),
            Byte42(Ranges_),
            Byte41(Ranges_),
            Byte40(Ranges_),
            Byte39(Ranges_),
            Byte37(Ranges_),
            Byte35(Ranges_),
            Byte33(Ranges_),
            Byte31(Ranges_),
            Byte29(Ranges_),
            Byte27(Ranges_),
            Byte25(Ranges_),
            Byte23(Ranges_),
            Byte21(Ranges_),
            Byte19(Ranges_),
            Byte17(Ranges_),
            Byte15(Ranges_),
            Byte13(Ranges_),
            Byte11(Ranges_),
            Byte9(Ranges_),
            Byte7(Ranges_),
            Byte5(Ranges_),
            Byte3(Ranges_),
            Byte1(Ranges_),
            Accept(Ranges_),
        }
        impl CompiledRegex_ {
            fn byte59(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::accept(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte59(ranges.clone()));
                    }
                }
            }
            fn byte58(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte59(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte58(ranges.clone()));
                    }
                }
            }
            fn byte57(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte58(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte57(ranges.clone()));
                    }
                }
            }
            fn byte56(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte57(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte56(ranges.clone()));
                    }
                }
            }
            fn byte55(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte56(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte55(ranges.clone()));
                    }
                }
            }
            fn byte54(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte55(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte54(ranges.clone()));
                    }
                }
            }
            fn byte53(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte54(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte53(ranges.clone()));
                    }
                }
            }
            fn byte52(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte53(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte52(ranges.clone()));
                    }
                }
            }
            fn byte51(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte52(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte51(ranges.clone()));
                    }
                }
            }
            fn byte50(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte51(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte50(ranges.clone()));
                    }
                }
            }
            fn byte49(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte50(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte49(ranges.clone()));
                    }
                }
            }
            fn byte48(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte49(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte48(ranges.clone()));
                    }
                }
            }
            fn byte47(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte48(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte47(ranges.clone()));
                    }
                }
            }
            fn byte46(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte47(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte46(ranges.clone()));
                    }
                }
            }
            fn byte45(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte46(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte45(ranges.clone()));
                    }
                }
            }
            fn byte44(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte45(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte44(ranges.clone()));
                    }
                }
            }
            fn byte43(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte44(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte43(ranges.clone()));
                    }
                }
            }
            fn byte42(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte43(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte42(ranges.clone()));
                    }
                }
            }
            fn byte41(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte42(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte41(ranges.clone()));
                    }
                }
            }
            fn byte40(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte41(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte40(ranges.clone()));
                    }
                }
            }
            fn byte39(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::byte40(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte39(ranges.clone()));
                    }
                }
            }
            fn optional38(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional38b(ranges, ib, next_states)
            }
            fn optional38b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional38c(ranges, ib, next_states)
            }
            fn optional38c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte39(ranges, ib, next_states);
                Self::byte40(ranges, ib, next_states);
            }
            fn byte37(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional38(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte37(ranges.clone()));
                    }
                }
            }
            fn optional36(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional36b(ranges, ib, next_states)
            }
            fn optional36b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional36c(ranges, ib, next_states)
            }
            fn optional34(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional34b(ranges, ib, next_states)
            }
            fn optional34b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional34c(ranges, ib, next_states)
            }
            fn optional32(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional32b(ranges, ib, next_states)
            }
            fn optional32b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional32c(ranges, ib, next_states)
            }
            fn optional30(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional30b(ranges, ib, next_states)
            }
            fn optional30b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional30c(ranges, ib, next_states)
            }
            fn optional28(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional28b(ranges, ib, next_states)
            }
            fn optional28b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional28c(ranges, ib, next_states)
            }
            fn optional26(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional26b(ranges, ib, next_states)
            }
            fn optional26b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional26c(ranges, ib, next_states)
            }
            fn optional24(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional24b(ranges, ib, next_states)
            }
            fn optional24b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional24c(ranges, ib, next_states)
            }
            fn optional22(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional22b(ranges, ib, next_states)
            }
            fn optional22b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional22c(ranges, ib, next_states)
            }
            fn optional20(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional20b(ranges, ib, next_states)
            }
            fn optional20b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional20c(ranges, ib, next_states)
            }
            fn optional18(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional18b(ranges, ib, next_states)
            }
            fn optional18b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional18c(ranges, ib, next_states)
            }
            fn optional16(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional16b(ranges, ib, next_states)
            }
            fn optional16b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional16c(ranges, ib, next_states)
            }
            fn optional14(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional14b(ranges, ib, next_states)
            }
            fn optional14b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional14c(ranges, ib, next_states)
            }
            fn optional12(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional12b(ranges, ib, next_states)
            }
            fn optional12b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional12c(ranges, ib, next_states)
            }
            fn optional10(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional10b(ranges, ib, next_states)
            }
            fn optional10b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional10c(ranges, ib, next_states)
            }
            fn optional8(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional8b(ranges, ib, next_states)
            }
            fn optional8b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional8c(ranges, ib, next_states)
            }
            fn optional6(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional6b(ranges, ib, next_states)
            }
            fn optional6b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional6c(ranges, ib, next_states)
            }
            fn optional4(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional4b(ranges, ib, next_states)
            }
            fn optional4b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional4c(ranges, ib, next_states)
            }
            fn optional2(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional2b(ranges, ib, next_states)
            }
            fn optional2b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional2c(ranges, ib, next_states)
            }
            fn optional0(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional0b(ranges, ib, next_states)
            }
            fn optional0b(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::optional0c(ranges, ib, next_states)
            }
            fn optional36c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte37(ranges, ib, next_states);
                Self::optional38(ranges, ib, next_states);
            }
            fn byte35(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional36(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte35(ranges.clone()));
                    }
                }
            }
            fn optional34c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte35(ranges, ib, next_states);
                Self::optional36(ranges, ib, next_states);
            }
            fn byte33(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional34(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte33(ranges.clone()));
                    }
                }
            }
            fn optional32c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte33(ranges, ib, next_states);
                Self::optional34(ranges, ib, next_states);
            }
            fn byte31(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional32(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte31(ranges.clone()));
                    }
                }
            }
            fn optional30c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte31(ranges, ib, next_states);
                Self::optional32(ranges, ib, next_states);
            }
            fn byte29(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional30(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte29(ranges.clone()));
                    }
                }
            }
            fn optional28c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte29(ranges, ib, next_states);
                Self::optional30(ranges, ib, next_states);
            }
            fn byte27(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional28(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte27(ranges.clone()));
                    }
                }
            }
            fn optional26c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte27(ranges, ib, next_states);
                Self::optional28(ranges, ib, next_states);
            }
            fn byte25(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional26(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte25(ranges.clone()));
                    }
                }
            }
            fn optional24c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte25(ranges, ib, next_states);
                Self::optional26(ranges, ib, next_states);
            }
            fn byte23(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional24(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte23(ranges.clone()));
                    }
                }
            }
            fn optional22c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte23(ranges, ib, next_states);
                Self::optional24(ranges, ib, next_states);
            }
            fn byte21(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional22(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte21(ranges.clone()));
                    }
                }
            }
            fn optional20c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte21(ranges, ib, next_states);
                Self::optional22(ranges, ib, next_states);
            }
            fn byte19(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional20(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte19(ranges.clone()));
                    }
                }
            }
            fn optional18c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte19(ranges, ib, next_states);
                Self::optional20(ranges, ib, next_states);
            }
            fn byte17(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional18(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte17(ranges.clone()));
                    }
                }
            }
            fn optional16c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte17(ranges, ib, next_states);
                Self::optional18(ranges, ib, next_states);
            }
            fn byte15(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional16(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte15(ranges.clone()));
                    }
                }
            }
            fn optional14c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte15(ranges, ib, next_states);
                Self::optional16(ranges, ib, next_states);
            }
            fn byte13(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional14(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte13(ranges.clone()));
                    }
                }
            }
            fn optional12c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte13(ranges, ib, next_states);
                Self::optional14(ranges, ib, next_states);
            }
            fn byte11(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional12(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte11(ranges.clone()));
                    }
                }
            }
            fn optional10c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte11(ranges, ib, next_states);
                Self::optional12(ranges, ib, next_states);
            }
            fn byte9(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional10(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte9(ranges.clone()));
                    }
                }
            }
            fn optional8c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte9(ranges, ib, next_states);
                Self::optional10(ranges, ib, next_states);
            }
            fn byte7(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional8(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte7(ranges.clone()));
                    }
                }
            }
            fn optional6c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte7(ranges, ib, next_states);
                Self::optional8(ranges, ib, next_states);
            }
            fn byte5(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional6(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte5(ranges.clone()));
                    }
                }
            }
            fn optional4c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte5(ranges, ib, next_states);
                Self::optional6(ranges, ib, next_states);
            }
            fn byte3(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional4(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte3(ranges.clone()));
                    }
                }
            }
            fn optional2c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte3(ranges, ib, next_states);
                Self::optional4(ranges, ib, next_states);
            }
            fn byte1(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(b) if b == 97u8 => Self::optional2(ranges, ib.consume(), next_states),
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Byte1(ranges.clone()));
                    }
                }
            }
            fn optional0c(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                Self::byte1(ranges, ib, next_states);
                Self::optional2(ranges, ib, next_states);
            }
            fn accept(ranges: &Ranges_, ib: InputByte, next_states: &mut States_) {
                match ib.byte() {
                    Some(_) => {}
                    None => {
                        next_states.insert(Self::Accept(ranges.clone()));
                    }
                }
            }
        }
        impl safe_regex::internal::Machine for CompiledRegex_ {
            type GroupRanges = [core::ops::Range<u32>; 0usize];
            fn expression() -> &'static [u8] {
                br"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa"
            }
            fn start(next_states: &mut States_) {
                Self::optional0(&Ranges_::new(), InputByte::Consumed(0), next_states);
            }
            fn try_accept(&self) -> Option<Self::GroupRanges> {
                match self {
                    Self::Accept(ranges) => Some(ranges.inner().clone()),
                    _ => None,
                }
            }
            fn make_next_states(&self, b: u8, n: u32, next_states: &mut States_) {
                let ib = InputByte::Available(b, n);
                match self {
                    Self::Byte59(ranges) => Self::byte59(ranges, ib, next_states),
                    Self::Byte58(ranges) => Self::byte58(ranges, ib, next_states),
                    Self::Byte57(ranges) => Self::byte57(ranges, ib, next_states),
                    Self::Byte56(ranges) => Self::byte56(ranges, ib, next_states),
                    Self::Byte55(ranges) => Self::byte55(ranges, ib, next_states),
                    Self::Byte54(ranges) => Self::byte54(ranges, ib, next_states),
                    Self::Byte53(ranges) => Self::byte53(ranges, ib, next_states),
                    Self::Byte52(ranges) => Self::byte52(ranges, ib, next_states),
                    Self::Byte51(ranges) => Self::byte51(ranges, ib, next_states),
                    Self::Byte50(ranges) => Self::byte50(ranges, ib, next_states),
                    Self::Byte49(ranges) => Self::byte49(ranges, ib, next_states),
                    Self::Byte48(ranges) => Self::byte48(ranges, ib, next_states),
                    Self::Byte47(ranges) => Self::byte47(ranges, ib, next_states),
                    Self::Byte46(ranges) => Self::byte46(ranges, ib, next_states),
                    Self::Byte45(ranges) => Self::byte45(ranges, ib, next_states),
                    Self::Byte44(ranges) => Self::byte44(ranges, ib, next_states),
                    Self::Byte43(ranges) => Self::byte43(ranges, ib, next_states),
                    Self::Byte42(ranges) => Self::byte42(ranges, ib, next_states),
                    Self::Byte41(ranges) => Self::byte41(ranges, ib, next_states),
                    Self::Byte40(ranges) => Self::byte40(ranges, ib, next_states),
                    Self::Byte39(ranges) => Self::byte39(ranges, ib, next_states),
                    Self::Byte37(ranges) => Self::byte37(ranges, ib, next_states),
                    Self::Byte35(ranges) => Self::byte35(ranges, ib, next_states),
                    Self::Byte33(ranges) => Self::byte33(ranges, ib, next_states),
                    Self::Byte31(ranges) => Self::byte31(ranges, ib, next_states),
                    Self::Byte29(ranges) => Self::byte29(ranges, ib, next_states),
                    Self::Byte27(ranges) => Self::byte27(ranges, ib, next_states),
                    Self::Byte25(ranges) => Self::byte25(ranges, ib, next_states),
                    Self::Byte23(ranges) => Self::byte23(ranges, ib, next_states),
                    Self::Byte21(ranges) => Self::byte21(ranges, ib, next_states),
                    Self::Byte19(ranges) => Self::byte19(ranges, ib, next_states),
                    Self::Byte17(ranges) => Self::byte17(ranges, ib, next_states),
                    Self::Byte15(ranges) => Self::byte15(ranges, ib, next_states),
                    Self::Byte13(ranges) => Self::byte13(ranges, ib, next_states),
                    Self::Byte11(ranges) => Self::byte11(ranges, ib, next_states),
                    Self::Byte9(ranges) => Self::byte9(ranges, ib, next_states),
                    Self::Byte7(ranges) => Self::byte7(ranges, ib, next_states),
                    Self::Byte5(ranges) => Self::byte5(ranges, ib, next_states),
                    Self::Byte3(ranges) => Self::byte3(ranges, ib, next_states),
                    Self::Byte1(ranges) => Self::byte1(ranges, ib, next_states),
                    Self::Accept(ranges) => Self::accept(ranges, ib, next_states),
                }
            }
        }
        <safe_regex::Matcher<CompiledRegex_>>::new()
    };
    let data = b"aaaaaaaaaaaaaaaaaaaa";
    println!(
        "{:?}.match_all({:?}) -> {:?}",
        re,
        escape_ascii(data),
        re.match_all(data)
    );
}
