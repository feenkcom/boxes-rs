use crate::ValueBox;
use std::any::Any;
use thiserror::Error;
use user_error::{UserFacingError, UFE};

const SUMMARY_PREFIX: &str = "\u{001b}[97;41;22mError:\u{001b}[91;49;1m ";
const RESET: &str = "\u{001b}[0m";
const REASON_PREFIX: &str = "\u{001b}[93;49;1m - \u{001b}[97;49;1m";

#[derive(Error, Debug)]
pub enum BoxerError {
    #[error("The pointer to the box of type {0} is null")]
    NullPointer(String),
    #[error("There is no value of type {0} in the box")]
    NoValue(String),
    #[error("There was an error")]
    #[cfg(feature = "anyhow")]
    AnyhowError(#[from] anyhow::Error),
    #[error("There was an IO error")]
    IOError(#[from] std::io::Error),
    #[error("There was an error")]
    AnyError(#[from] Box<dyn std::error::Error>),
}

impl<T> From<BoxerError> for core::result::Result<T, BoxerError> {
    fn from(error: BoxerError) -> Self {
        Err(error)
    }
}

impl From<String> for BoxerError {
    fn from(value: String) -> Self {
        Self::AnyError(value.into())
    }
}

impl From<&str> for BoxerError {
    fn from(value: &str) -> Self {
        Self::AnyError(value.into())
    }
}

pub type Result<T> = core::result::Result<T, BoxerError>;

pub trait ReturnBoxerResult<Return: Any> {
    fn log(self);
    fn or_log(self, value: Return) -> Return;
    fn or_print(self, value: Return) -> Return;
}

pub trait ValueBoxIntoRaw<Return: Any> {
    fn into_raw(self) -> *mut ValueBox<Return>;
}

impl<Return: Any> ReturnBoxerResult<Return> for Result<Return> {
    fn log(self) {
        if let Err(error) = self {
            log_boxer_error(error);
        }
    }

    fn or_log(self, value: Return) -> Return {
        self.unwrap_or_else(|error| {
            log_boxer_error(error);
            value
        })
    }

    fn or_print(self, value: Return) -> Return {
        self.map_err(|error| {
            let error: Box<dyn std::error::Error> = Box::new(error);
            let user_facing_error: UserFacingError = error.into();
            user_facing_error
        })
        .unwrap_or_else(|error| {
            println!("{}", pretty_summary(error.summary().as_str()));
            if let Some(reasons) = pretty_reasons(error.reasons()) {
                println!("{}", reasons);
            }
            value
        })
    }
}

impl<Return: Any> ValueBoxIntoRaw<Return> for Result<ValueBox<Return>> {
    fn into_raw(self) -> *mut ValueBox<Return> {
        self.map(|value| value.into_raw())
            .or_log(std::ptr::null_mut())
    }
}

impl<Return: Any> ValueBoxIntoRaw<Return> for Result<Option<ValueBox<Return>>> {
    fn into_raw(self) -> *mut ValueBox<Return> {
        self.map(|option| {
            option
                .map(|value| value.into_raw())
                .unwrap_or_else(|| std::ptr::null_mut())
        })
        .or_log(std::ptr::null_mut())
    }
}

fn log_boxer_error(error: BoxerError) {
    match &error {
        BoxerError::NullPointer(_) => warn_user_facing_error(to_user_facing_error(error)),
        BoxerError::NoValue(_) => warn_user_facing_error(to_user_facing_error(error)),
        _ => error_user_facing_error(to_user_facing_error(error)),
    };
}

fn warn_user_facing_error(error: UserFacingError) {
    warn!("{}", pretty_summary(error.summary().as_str()));
    if let Some(reasons) = pretty_reasons(error.reasons()) {
        warn!("{}", reasons);
    }
}

fn error_user_facing_error(error: UserFacingError) {
    error!("{}", pretty_summary(error.summary().as_str()));
    if let Some(reasons) = pretty_reasons(error.reasons()) {
        error!("{}", reasons);
    }
}

fn to_user_facing_error(error: BoxerError) -> UserFacingError {
    let error: Box<dyn std::error::Error> = Box::new(error);
    let user_facing_error: UserFacingError = error.into();
    user_facing_error
}

fn pretty_summary(summary: &str) -> String {
    [SUMMARY_PREFIX, summary, RESET].concat()
}

fn pretty_reasons(reasons: Option<Vec<String>>) -> Option<String> {
    reasons.map(|reasons| {
        let mut reason_strings = Vec::with_capacity(reasons.len());
        for reason in reasons {
            let bullet_point = [REASON_PREFIX, &reason].concat();
            reason_strings.push(bullet_point);
        }
        [&reason_strings.join("\n"), RESET].concat()
    })
}
