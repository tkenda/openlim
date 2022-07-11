use crate::message::*;
use crate::CharEncoding;

// <STX>5R|2|^^^1.0000+950+1.0|15|||^5^||V||34001637|20080516153540|20080516153602|34001637<CR><ETX>3D<CR><LF>
fn build_raw_frame() -> Vec<u8> {
    // <STX>
    let mut dst = vec![0x02];

    // 5R|2|^^^1.0000+950+1.0|15|||^5^||V||34001637|20080516153540|20080516153602|34001637<CR>
    let mut content = vec![
        53, 82, 124, 50, 124, 94, 94, 94, 49, 46, 48, 48, 48, 48, 43, 57, 53, 48, 43, 49, 46, 48,
        124, 49, 53, 124, 124, 124, 94, 53, 94, 124, 124, 86, 124, 124, 51, 52, 48, 48, 49, 54, 51,
        55, 124, 50, 48, 48, 56, 48, 53, 49, 54, 49, 53, 51, 53, 52, 48, 124, 50, 48, 48, 56, 48,
        53, 49, 54, 49, 53, 51, 54, 48, 50, 124, 51, 52, 48, 48, 49, 54, 51, 55, 0x0D,
    ];
    dst.append(&mut content);

    // <ETX>3D<CR><LF>
    let mut tail = vec![0x03, 0x33, 0x44, 0x0D, 0x0A];
    dst.append(&mut tail);

    dst
}

#[test]
fn deserialize() {
    let src = build_raw_frame();
    let frame = Frame::deserialize(&src, CharEncoding::UTF8).unwrap();

    assert_eq!(frame.number(), 5);
    assert_eq!(
        frame.data(),
        "R|2|^^^1.0000+950+1.0|15|||^5^||V||34001637|20080516153540|20080516153602|34001637\r"
    );
    assert!(frame.is_last());
}

#[test]
fn serialize() {
    let frame = Frame {
        number: 5,
        data:
            "R|2|^^^1.0000+950+1.0|15|||^5^||V||34001637|20080516153540|20080516153602|34001637\r"
                .to_string(),
        last: true,
    };
    let raw = frame.serialize(CharEncoding::UTF8).unwrap();
    let dst = build_raw_frame();

    assert_eq!(raw, dst);
}

fn build_raw_message() -> String {
    let mut src =
        "H|\\^&|||Alinity ci-series^2.5^SCM01246|||||||P|LIS2-A2|20190821102030-3000\r".to_string();
    src.push_str("P|1|||9750230|TEST||19921018|F\r");
    src.push_str("O|1|9750230|9750230^L5777^6^1^5|^^^248^TSH^UNDILUTED|R||||||||||||||||||||F\r");
    src
}

fn build_message() -> Message {
    Message {
        frames: vec![
            Frame {
                number: 1,
                data:
                    "H|\\^&|||Alinity ci-series^2.5^SCM01246|||||||P|LIS2-A2|20190821102030-3000\r"
                        .to_string(),
                last: true,
            },
            Frame {
                number: 2,
                data: "P|1|||9750230|TEST||19921018|F\r".to_string(),
                last: true,
            },
            Frame {
                number: 3,
                data:
                    "O|1|9750230|9750230^L5777^6^1^5|^^^248^TSH^UNDILUTED|R||||||||||||||||||||F\r"
                        .to_string(),
                last: true,
            },
        ],
        ..Default::default()
    }
}

#[test]
fn to_message() {
    let message: Message = build_raw_message().parse().unwrap();
    assert_eq!(message, build_message());
}

#[test]
fn from_message() {
    assert_eq!(build_message().to_string(), build_raw_message());
}
