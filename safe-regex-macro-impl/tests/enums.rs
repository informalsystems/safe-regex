#![forbid(unsafe_code)]
use safe_regex_parser::{ClassItem, FinalNode, Node, NonFinalNode};

#[test]
fn node() {
    assert_eq!(
        FinalNode::AnyByte,
        Node::Final(FinalNode::AnyByte).unwrap_final()
    );
    assert_eq!(
        NonFinalNode::OpenGroup,
        Node::NonFinal(NonFinalNode::OpenGroup).unwrap_non_final()
    );
}

#[test]
#[should_panic]
fn unwrap_final() {
    Node::NonFinal(NonFinalNode::OpenGroup).unwrap_final();
}

#[test]
#[should_panic]
fn unwrap_non_final() {
    Node::Final(FinalNode::AnyByte).unwrap_non_final();
}

#[test]
fn class_item() {
    assert_eq!("Byte(a)", format!("{:?}", ClassItem::Byte(b'a')));
    assert_eq!(
        "ByteRange(0-9)",
        format!("{:?}", ClassItem::ByteRange(b'0', b'9'))
    );
}

#[test]
fn non_final_node() {
    assert_eq!("Escape", format!("{:?}", NonFinalNode::Escape));
    assert_eq!("HexEscape0", format!("{:?}", NonFinalNode::HexEscape0));
    assert_eq!(
        "HexEscape1(a)",
        format!("{:?}", NonFinalNode::HexEscape1(b'a'))
    );
    assert_eq!("OpenClass0", format!("{:?}", NonFinalNode::OpenClass0));
    assert_eq!("OpenClassNeg", format!("{:?}", NonFinalNode::OpenClassNeg));
    assert_eq!(
        "OpenClass[Byte(a)]",
        format!(
            "{:?}",
            NonFinalNode::OpenClass(true, vec![ClassItem::Byte(b'a')])
        )
    );
    assert_eq!(
        "OpenClass^[Byte(a)]",
        format!(
            "{:?}",
            NonFinalNode::OpenClass(false, vec![ClassItem::Byte(b'a')])
        )
    );
    assert_eq!(
        "OpenByteRange(a)",
        format!("{:?}", NonFinalNode::OpenByteRange(b'a'))
    );
    assert_eq!(
        "ByteRange(0-9)",
        format!("{:?}", NonFinalNode::ByteRange(b'0', b'9'))
    );
    assert_eq!("OpenGroup", format!("{:?}", NonFinalNode::OpenGroup));
    assert_eq!(
        "OpenOr[AnyByte]",
        format!("{:?}", NonFinalNode::OpenOr(vec![FinalNode::AnyByte]))
    );
    assert_eq!(
        "RepeatMin(123)",
        format!("{:?}", NonFinalNode::RepeatMin("123".to_string()))
    );
    assert_eq!(
        "RepeatMax(123,456)",
        format!(
            "{:?}",
            NonFinalNode::RepeatMax("123".to_string(), "456".to_string())
        )
    );
    assert_eq!(
        "RepeatToken(\"printable0\",5,Some(7))",
        format!(
            "{:?}",
            NonFinalNode::RepeatToken("printable0".to_string(), 5, Some(7))
        )
    );

    assert_eq!(
        (true, vec![ClassItem::Byte(b'a')]),
        NonFinalNode::OpenClass(true, vec![ClassItem::Byte(b'a')]).unwrap_open_class()
    );
    assert_eq!(
        vec![FinalNode::AnyByte],
        NonFinalNode::OpenOr(vec![FinalNode::AnyByte]).unwrap_open_or()
    );
    assert_eq!(
        "123".to_string(),
        NonFinalNode::RepeatMin("123".to_string()).unwrap_repeat_min()
    );
    assert_eq!(
        ("123".to_string(), "456".to_string()),
        NonFinalNode::RepeatMax("123".to_string(), "456".to_string()).unwrap_repeat_max()
    );
}

#[test]
#[should_panic]
fn unwrap_open_class() {
    NonFinalNode::OpenGroup.unwrap_open_class();
}

#[test]
#[should_panic]
fn unwrap_open_or() {
    NonFinalNode::OpenGroup.unwrap_open_or();
}

#[test]
#[should_panic]
fn unwrap_repeat_min() {
    NonFinalNode::OpenGroup.unwrap_repeat_min();
}

#[test]
#[should_panic]
fn unwrap_repeat_max() {
    NonFinalNode::OpenGroup.unwrap_repeat_max();
}

#[test]
fn final_node() {
    assert_eq!("Byte(a)", format!("{:?}", FinalNode::Byte(b'a')));
    assert_eq!("AnyByte", format!("{:?}", FinalNode::AnyByte));
    assert_eq!(
        "Seq[AnyByte]",
        format!("{:?}", FinalNode::Seq(vec![FinalNode::AnyByte]))
    );
    assert_eq!(
        "Class[Byte(a)]",
        format!("{:?}", FinalNode::Class(true, vec![ClassItem::Byte(b'a')]))
    );
    assert_eq!(
        "Class^[Byte(a)]",
        format!("{:?}", FinalNode::Class(false, vec![ClassItem::Byte(b'a')]))
    );
    assert_eq!(
        "Group(AnyByte)",
        format!("{:?}", FinalNode::Group(Box::new(FinalNode::AnyByte)))
    );
    assert_eq!(
        "Or[AnyByte]",
        format!("{:?}", FinalNode::Or(vec![FinalNode::AnyByte]))
    );
    assert_eq!(
        "Repeat(AnyByte,5-Some(7))",
        format!(
            "{:?}",
            FinalNode::Repeat(Box::new(FinalNode::AnyByte), 5, Some(7))
        )
    );

    assert_eq!(
        vec![FinalNode::AnyByte],
        FinalNode::Or(vec![FinalNode::AnyByte]).unwrap_or()
    );
}

#[test]
#[should_panic]
fn unwrap_or() {
    FinalNode::AnyByte.unwrap_or();
}
