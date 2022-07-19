use serde::Deserialize;

use super::{
    AddCandidateResponse, Candidates, GetCandidatesResponse, GetOfferResponse, HangUpResponse,
    Offer, SetAnswerResponse, WebRtcError,
};

/// A type that deserializes from either `Result`s representation or a boolean,
/// for backwards compatibility of deserializing SetAnswersResponse,
/// AddCandidateResponse and HangUpResponse
#[derive(Deserialize)]
#[serde(untagged)]
pub enum BoolResponseCompat {
    Result(Result<(), WebRtcError>),
    Bool(bool),
}

impl BoolResponseCompat {
    fn into_result(self) -> Result<(), WebRtcError> {
        match self {
            Self::Result(r) => r,
            Self::Bool(true) => Ok(()),
            Self::Bool(false) => Err(WebRtcError(None)),
        }
    }
}

impl From<BoolResponseCompat> for SetAnswerResponse {
    fn from(val: BoolResponseCompat) -> Self {
        Self(val.into_result())
    }
}

impl From<BoolResponseCompat> for AddCandidateResponse {
    fn from(val: BoolResponseCompat) -> Self {
        Self(val.into_result())
    }
}

impl From<BoolResponseCompat> for HangUpResponse {
    fn from(val: BoolResponseCompat) -> Self {
        Self(val.into_result())
    }
}

/// A type that deserializes from either a `Result`s representation or from the
/// inner `Ok` variant's representation, for backwards compatibility of
/// deserializing GetOfferResponse and GetCandidatesResponse
#[derive(Deserialize)]
#[serde(untagged)]
pub enum StructResponseCompat<T> {
    Result(Result<T, WebRtcError>),
    Struct(T),
}

impl<T> StructResponseCompat<T> {
    fn into_result(self) -> Result<T, WebRtcError> {
        match self {
            StructResponseCompat::Result(r) => r,
            StructResponseCompat::Struct(s) => Ok(s),
        }
    }
}

impl From<StructResponseCompat<Offer>> for GetOfferResponse {
    fn from(val: StructResponseCompat<Offer>) -> Self {
        Self(val.into_result())
    }
}

impl From<StructResponseCompat<Candidates>> for GetCandidatesResponse {
    fn from(val: StructResponseCompat<Candidates>) -> Self {
        Self(val.into_result())
    }
}
