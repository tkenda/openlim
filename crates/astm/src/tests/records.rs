use chrono::{FixedOffset, TimeZone};

use crate::records::*;
use crate::{Message, Records};

#[test]
fn message_header() {
    let src =
        "H|\\^&|||Alinity ci-series^2.5^SCM01246|||||||P|LIS2-A2|20190821102030-0300\r".to_string();
    let message: Message = src.parse().unwrap();
    let records: Records = message.try_into().unwrap();
    let first = records.into_iter().next().unwrap();

    let dt = FixedOffset::west(10800)
        .ymd(2019, 8, 21)
        .and_hms(10, 20, 30);
    let date_time = ASTMDateTime(dt);

    assert_eq!(
        first,
        Record::MessageHeader(MessageHeaderRecord {
            delimiter_definition: Some("\\^&".to_string()),
            message_control_id: None,
            access_password: None,
            sender_name_or_id: Some("Alinity ci-series^2.5^SCM01246".to_string()),
            sender_street_address: None,
            reserver_field: None,
            sender_telephone_number: None,
            characteristics_of_sender: None,
            receiver_id: None,
            comment_or_special_instructions: None,
            processing_id: Some(ProcessingID::Production),
            version_number: Some("LIS2-A2".to_string()),
            date_and_time_of_message: Some(date_time),
        })
    );
}
