use safe_regex_macro::regex;

#[test]
fn test() {
    // Literal { kind: Str, symbol: "abc€\\x20\\u{00A2}", suffix: None, span: #0 bytes(61..81) }
    // regex!("abc€\x20\u{00A2}");

    // Literal { kind: StrRaw(0), symbol: "abc€\\x20\\u{00A2}", suffix: None, span: #0 bytes(61..82)
    // regex!(r"abc€\x20\u{00A2}");

    // Literal { kind: ByteStr, symbol: "abc\\x20", suffix: None, span: #0 bytes(338..348) }
    // regex!(b"abc\x20");

    // Literal { kind: ByteStrRaw(0), symbol: "abc\\x20", suffix: None, span: #0 bytes(461..472) }
    // regex!(br"abc\x20");

    // regex!(b"abc\x20\n").match_all("abc").unwrap();
    // regex!(br#"\n\r\t\\\0\'\""#).match_all("abc").unwrap();
    // regex!(br"\n\?[abc]\x01");
    //regex!(br"abc\x20");
}
