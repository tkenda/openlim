use chrono::{FixedOffset, TimeZone, NaiveDate};
use crate::values::*;

#[test]
fn address() {
    let address: Address = "52 Hilton Street #B42^Chicago^IL^60305^USA"
        .parse()
        .unwrap();
    assert_eq!(
        address,
        Address {
            street_address: Some("52 Hilton Street #B42".to_string()),
            city: Some("Chicago".to_string()),
            state: Some("IL".to_string()),
            postal_code: Some("60305".to_string()),
            country_code: Some("USA".to_string()),
        }
    );
}

#[test]
fn astm_date_time_with_offset() {
    let dt = FixedOffset::west(10800)
        .ymd(2019, 8, 21)
        .and_hms(10, 20, 30);
    let date_time_1 = ASTMDateTime(dt);
    let date_time_2: ASTMDateTime = "20190821102030-0300".parse().unwrap();
    assert_eq!(date_time_1, date_time_2);
}

#[test]
fn astm_date_time_without_offset() {
    let dt = FixedOffset::west(0).ymd(2019, 8, 21).and_hms(10, 20, 30);
    let date_time_1 = ASTMDateTime(dt);
    let date_time_2: ASTMDateTime = "20190821102030".parse().unwrap();
    assert_eq!(date_time_1, date_time_2);
}

#[test]
fn astm_date() {
    let date_1 = ASTMDate(NaiveDate::from_ymd(2019, 8, 21));
    let date_2: ASTMDate = "20190821".parse().unwrap();
    assert_eq!(date_1, date_2);
}

#[test]
fn processing_id() {
    assert_eq!(
        "P".parse::<ProcessingID>().unwrap(),
        ProcessingID::Production
    );
    assert_eq!("T".parse::<ProcessingID>().unwrap(), ProcessingID::Training);
    assert_eq!(
        "D".parse::<ProcessingID>().unwrap(),
        ProcessingID::Debugging
    );
    assert_eq!(
        "Q".parse::<ProcessingID>().unwrap(),
        ProcessingID::QualityControl
    );
}

#[test]
fn patient_name() {
    assert_eq!(
        "LAST^FIRST^MIDDLE^SUFFIX^TITLE"
            .parse::<PatientName>()
            .unwrap(),
        PatientName {
            last_name: Some("LAST".to_string()),
            first_name: Some("FIRST".to_string()),
            middle_name: Some("MIDDLE".to_string()),
            suffix: Some("SUFFIX".to_string()),
            title: Some("TITLE".to_string()),
        }
    );
}

#[test]
fn patient_sex() {
    assert_eq!("M".parse::<PatientSex>().unwrap(), PatientSex::Male);
    assert_eq!("F".parse::<PatientSex>().unwrap(), PatientSex::Female);
    assert_eq!("U".parse::<PatientSex>().unwrap(), PatientSex::Unknown);
    assert!("X".parse::<PatientSex>().is_err());
}

#[test]
fn mesaurement() {
    assert_eq!("20^kg".parse::<Measurement>().unwrap(), Measurement {
        measure: 20.0,
        unit: Some("kg".to_string()),
    })
}

#[test]
fn admission_status() {
    let outpatient = AdmissionStatus::Outpatient.to_string();
    let preadmit = AdmissionStatus::Preadmit.to_string();
    let inpatient = AdmissionStatus::Inpatient.to_string();
    let emergency_room = AdmissionStatus::EmergencyRoom.to_string();
    let other = AdmissionStatus::Other("OTHER".to_string()).to_string();

    assert_eq!(outpatient.parse::<AdmissionStatus>().unwrap(), AdmissionStatus::Outpatient);
    assert_eq!(preadmit.parse::<AdmissionStatus>().unwrap(), AdmissionStatus::Preadmit);
    assert_eq!(inpatient.parse::<AdmissionStatus>().unwrap(), AdmissionStatus::Inpatient);
    assert_eq!(emergency_room.parse::<AdmissionStatus>().unwrap(), AdmissionStatus::EmergencyRoom);
    assert_eq!(other.parse::<AdmissionStatus>().unwrap(), AdmissionStatus::Other("OTHER".to_string()));
}

#[test]
fn patient_religion() {
    let protestant = PatientReligion::Protestant.to_string();
    let catholic = PatientReligion::Catholic.to_string();
    let mormon = PatientReligion::Mormon.to_string();
    let jewish = PatientReligion::Jewish.to_string();
    let lutheran = PatientReligion::Lutheran.to_string();
    let hindu = PatientReligion::Hindu.to_string();
    let other = PatientReligion::Other("OTHER".to_string()).to_string();

    assert_eq!(protestant.parse::<PatientReligion>().unwrap(), PatientReligion::Protestant);
    assert_eq!(catholic.parse::<PatientReligion>().unwrap(), PatientReligion::Catholic);
    assert_eq!(mormon.parse::<PatientReligion>().unwrap(), PatientReligion::Mormon);
    assert_eq!(jewish.parse::<PatientReligion>().unwrap(), PatientReligion::Jewish);
    assert_eq!(lutheran.parse::<PatientReligion>().unwrap(), PatientReligion::Lutheran);
    assert_eq!(hindu.parse::<PatientReligion>().unwrap(), PatientReligion::Hindu);
    assert_eq!(other.parse::<AdmissionStatus>().unwrap(), AdmissionStatus::Other("OTHER".to_string()));
}

#[test]
fn marital_status() {
    let married = MaritalStatus::Married.to_string();
    let single = MaritalStatus::Single.to_string();
    let divorced = MaritalStatus::Divorced.to_string();
    let widowed = MaritalStatus::Widowed.to_string();
    let separated = MaritalStatus::Separated.to_string();

    assert_eq!(married.parse::<MaritalStatus>().unwrap(), MaritalStatus::Married);
    assert_eq!(single.parse::<MaritalStatus>().unwrap(), MaritalStatus::Single);
    assert_eq!(divorced.parse::<MaritalStatus>().unwrap(), MaritalStatus::Divorced);
    assert_eq!(widowed.parse::<MaritalStatus>().unwrap(), MaritalStatus::Widowed);
    assert_eq!(separated.parse::<MaritalStatus>().unwrap(), MaritalStatus::Separated);
}

#[test]
fn isolation_status() {
    
}