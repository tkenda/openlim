use chrono::{DateTime, FixedOffset, NaiveDate};
use std::str::FromStr;

use crate::{ASTMError, Result};

/* Address */

#[derive(Debug, Default, PartialEq)]
pub struct Address {
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

impl FromStr for Address {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        let dst: Vec<&str> = src.split("^").collect();

        Ok(Address {
            street_address: dst.get(0).map(|t| t.to_string()),
            city: dst.get(1).map(|t| t.to_string()),
            state: dst.get(2).map(|t| t.to_string()),
            postal_code: dst.get(3).map(|t| t.to_string()),
            country_code: dst.get(4).map(|t| t.to_string()),
        })
    }
}

/* Dates and Times */

#[derive(Debug, PartialEq)]
pub struct ASTMDateTime(pub(crate) DateTime<FixedOffset>);

impl FromStr for ASTMDateTime {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match DateTime::parse_from_str(&src, "%Y%m%d%H%M%S%z") {
            Ok(t) => Ok(ASTMDateTime(t)),
            Err(_) => {
                let dst = format!("{}+0000", src);
                DateTime::parse_from_str(&dst, "%Y%m%d%H%M%S%z")
                    .map_err(ASTMError::InvalidDateAndTimeValue)
                    .map(ASTMDateTime)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTMDate(pub(crate) NaiveDate);

impl FromStr for ASTMDate {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        NaiveDate::parse_from_str(&src, "%Y%m%d")
            .map_err(ASTMError::InvalidDateAndTimeValue)
            .map(ASTMDate)
    }
}

/* Processing ID */

#[derive(Debug, PartialEq)]
pub enum ProcessingID {
    Production,
    Training,
    Debugging,
    QualityControl,
}

impl Default for ProcessingID {
    fn default() -> Self {
        Self::Production
    }
}

impl FromStr for ProcessingID {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src.trim() {
            "P" => Ok(ProcessingID::Production),
            "T" => Ok(ProcessingID::Training),
            "D" => Ok(ProcessingID::Debugging),
            "Q" => Ok(ProcessingID::QualityControl),
            _ => Err(ASTMError::InvalidProcessingIDValue),
        }
    }
}

/* Patient Name */

#[derive(Debug, Default, PartialEq)]
pub struct PatientName {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub suffix: Option<String>,
    pub title: Option<String>,
}

impl FromStr for PatientName {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        let dst: Vec<&str> = src.split("^").collect();

        Ok(PatientName {
            last_name: dst.get(0).map(|t| t.to_string()),
            first_name: dst.get(1).map(|t| t.to_string()),
            middle_name: dst.get(2).map(|t| t.to_string()),
            suffix: dst.get(3).map(|t| t.to_string()),
            title: dst.get(4).map(|t| t.to_string()),
        })
    }
}

/* Patient Sex */

#[derive(Debug, PartialEq)]
pub enum PatientSex {
    Male,
    Female,
    Unknown,
}

impl Default for PatientSex {
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for PatientSex {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src.trim() {
            "M" => Ok(PatientSex::Male),
            "F" => Ok(PatientSex::Female),
            "U" => Ok(PatientSex::Unknown),
            _ => Err(ASTMError::InvalidPatientSexValue),
        }
    }
}

impl std::fmt::Display for PatientSex {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Male => fmt.write_str("M")?,
            Self::Female => fmt.write_str("F")?,
            Self::Unknown => fmt.write_str("U")?,
        }
        Ok(())
    }
}

/* Patient Race */

#[derive(Debug, PartialEq)]
pub enum PatientRace {
    White,
    Black,
    AsianPacificIslander,
    NativeAmericanAlaskanNative,
    Hispanic,
    Other(String),
}

impl FromStr for PatientRace {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src.trim() {
            "W" => Ok(PatientRace::White),
            "B" => Ok(PatientRace::Black),
            "O" => Ok(PatientRace::AsianPacificIslander),
            "NA" => Ok(PatientRace::NativeAmericanAlaskanNative),
            "H" => Ok(PatientRace::Hispanic),            
            _ => Ok(PatientRace::Other(src.to_string())),
        }
    }
}

impl std::fmt::Display for PatientRace {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::White => fmt.write_str("W")?,
            Self::Black => fmt.write_str("B")?,
            Self::AsianPacificIslander => fmt.write_str("O")?,
            Self::NativeAmericanAlaskanNative => fmt.write_str("NA")?,
            Self::Hispanic => fmt.write_str("H")?,
            Self::Other(t) => fmt.write_str(t)?,
        }
        Ok(())
    }
}

/* Measurement */

#[derive(Debug, Default, PartialEq)]
pub struct Measurement {
    pub(crate) measure: f64,
    pub(crate) unit: Option<String>,
}

impl FromStr for Measurement {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        let dst: Vec<&str> = src.split("^").collect();
        let value = dst.get(0).ok_or(ASTMError::MissingMeasurementValue)?;

        Ok(Self {
            measure: value.parse::<f64>().map_err(ASTMError::ParseFloatNumber)?,
            unit: dst.get(1).map(|t| t.to_string()),
        })
    }
}

/* Admission Status */

#[derive(Debug, PartialEq)]
pub enum AdmissionStatus {
    Outpatient,
    Preadmit,
    Inpatient,
    EmergencyRoom,
    Other(String),
}

impl FromStr for AdmissionStatus {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src {
            "OP" => Ok(Self::Outpatient),
            "PA" => Ok(Self::Preadmit),
            "IP" => Ok(Self::Inpatient),
            "ER" => Ok(Self::EmergencyRoom),
            _ => Ok(Self::Other(src.to_string()))
        }
    }
}

impl std::fmt::Display for AdmissionStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Outpatient => fmt.write_str("OP")?,
            Self::Preadmit => fmt.write_str("PA")?,
            Self::Inpatient => fmt.write_str("IP")?,
            Self::EmergencyRoom => fmt.write_str("ER")?,
            Self::Other(t) => fmt.write_str(t)?,
        }
        Ok(())
    }
}

/* Patient Religion */

#[derive(Debug, PartialEq)]
pub enum PatientReligion {
    Protestant,
    Catholic,
    Mormon,
    Jewish,
    Lutheran,
    Hindu,
    Other(String),
}

impl FromStr for PatientReligion {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src {
            "P" => Ok(Self::Protestant),
            "C" => Ok(Self::Catholic),
            "M" => Ok(Self::Mormon),
            "J" => Ok(Self::Jewish),
            "L" => Ok(Self::Lutheran),
            "H" => Ok(Self::Hindu),
            _ => Ok(Self::Other(src.to_string()))
        }
    }
}

impl std::fmt::Display for PatientReligion {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Protestant => fmt.write_str("P")?,
            Self::Catholic => fmt.write_str("C")?,
            Self::Mormon => fmt.write_str("M")?,
            Self::Jewish => fmt.write_str("J")?,
            Self::Lutheran => fmt.write_str("L")?,
            Self::Hindu => fmt.write_str("H")?,
            Self::Other(t) => fmt.write_str(t)?,
        }
        Ok(())
    }
}

/* Marital Status */

#[derive(Debug, PartialEq)]
pub enum MaritalStatus {
    Married,
    Single,
    Divorced,
    Widowed,
    Separated,
}

impl FromStr for MaritalStatus {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src {
            "M" => Ok(Self::Married),
            "S" => Ok(Self::Single),
            "D" => Ok(Self::Divorced),
            "W" => Ok(Self::Widowed),
            "A" => Ok(Self::Separated),
            _ => Err(ASTMError::InvalidMaritalStatusValue)
        }
    }
}

impl std::fmt::Display for MaritalStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Married => fmt.write_str("M")?,
            Self::Single => fmt.write_str("S")?,
            Self::Divorced => fmt.write_str("D")?,
            Self::Widowed => fmt.write_str("W")?,
            Self::Separated => fmt.write_str("A")?,
        }
        Ok(())
    }
}

/* Isolation Status */

#[derive(Debug, PartialEq)]
pub enum IsolationStatus {
    AntibioticResistancePrecautions,
    BloodAndNeedlePrecautions,
    EntericPrecautions,
    PrecautionsForNeutropenicPatient,
    PrecautionsForPregnantWomen,
    RespiratoryIsolation,
    SecretionExcretionPrecautions,
    StrictIsolation,
    WoundAndSkinPrecautions,
    Other(String),
}

impl FromStr for IsolationStatus {
    type Err = ASTMError;

    fn from_str(src: &str) -> Result<Self> {
        match src {
            "ARP" => Ok(Self::AntibioticResistancePrecautions),
            "BP" => Ok(Self::BloodAndNeedlePrecautions),
            "ENP" => Ok(Self::EntericPrecautions),
            "NP" => Ok(Self::PrecautionsForNeutropenicPatient),
            "PWP" => Ok(Self::PrecautionsForPregnantWomen),
            "RI" => Ok(Self::RespiratoryIsolation),
            "SE" => Ok(Self::SecretionExcretionPrecautions),
            "SI" => Ok(Self::StrictIsolation),
            "WSP" => Ok(Self::WoundAndSkinPrecautions),
            _ => Ok(Self::Other(src.to_string()))
        }
    }
}

impl std::fmt::Display for IsolationStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AntibioticResistancePrecautions => fmt.write_str("ARP")?,
            Self::BloodAndNeedlePrecautions => fmt.write_str("BP")?,
            Self::EntericPrecautions => fmt.write_str("ENP")?,
            Self::PrecautionsForNeutropenicPatient => fmt.write_str("NP")?,
            Self::PrecautionsForPregnantWomen => fmt.write_str("PWP")?,
            Self::RespiratoryIsolation => fmt.write_str("RI")?,
            Self::SecretionExcretionPrecautions => fmt.write_str("SE")?,
            Self::StrictIsolation => fmt.write_str("SI")?,
            Self::WoundAndSkinPrecautions => fmt.write_str("WSP")?,
            Self::Other(t) => fmt.write_str(&t)?,
        }
        Ok(())
    }
}