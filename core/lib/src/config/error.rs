use std::{io, fmt};
use std::path::PathBuf;
use std::error::Error;

use super::Environment;
use self::ConfigError::*;

/// The type of a configuration error.
#[derive(Debug)]
pub enum ConfigError {
    /// The configuration file was not found.
    NotFound,
    /// There was an I/O error while reading the configuration file.
    IoError,
    /// There was an I/O error while setting a configuration parameter.
    ///
    /// Parameters: (io_error, config_param_name)
    Io(io::Error, &'static str),
    /// The path at which the configuration file was found was invalid.
    ///
    /// Parameters: (path, reason)
    BadFilePath(PathBuf, &'static str),
    /// An environment specified in `ROCKET_ENV` is invalid.
    ///
    /// Parameters: (environment_name)
    BadEnv(String),
    /// An environment specified as a table `[environment]` is invalid.
    ///
    /// Parameters: (environment_name, filename)
    BadEntry(String, PathBuf),
    /// A config key was specified with a value of the wrong type.
    ///
    /// Parameters: (entry_name, expected_type, actual_type, filename)
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    /// There was a TOML parsing error.
    ///
    /// Parameters: (toml_source_string, filename, error_description, line/col)
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
    /// There was a TOML parsing error in a config environment variable.
    ///
    /// Parameters: (env_key, env_value, error)
    BadEnvVal(String, String, String),
    /// The entry (key) is unknown.
    ///
    /// Parameters: (key)
    UnknownKey(String),
    /// The entry (key) was expected but was missing.
    ///
    /// Parameters: (key)
    Missing(String),
}

impl ConfigError {
    /// Prints this configuration error with Rocket formatting.
    pub fn pretty_print(&self) {
        let valid_envs = Environment::VALID;
        match *self {
            NotFound => warn!("config file was not found"),
            IoError => warn!("failed reading the config file: IO error"),
            Io(ref error, param) => {
                warn!("I/O error while setting {}:", param);
                info_!("{}", error);
            }
            BadFilePath(ref path, reason) => {
                warn!("configuration file path {} is invalid", path.display());
                info_!("{}", reason);
            }
            BadEntry(ref name, ref filename) => {
                let valid_entries = format!("{}, global", valid_envs);
                warn!("[{}] is not a known configuration environment", name);
                info_!("in {}", filename.display());
                info_!("valid environments are: {}", valid_entries);
            }
            BadEnv(ref name) => {
                warn!("{} is not a valid ROCKET_ENV value", name);
                info_!("valid environments are: {}", valid_envs);
            }
            BadType(ref name, expected, actual, ref filename) => {
                warn!("{} key could not be parsed", name);
                if let Some(filename) = filename {
                    info_!("in {}", filename.display());
                }

                info_!("expected value to be {}, but found {}", expected, actual);
            }
            ParseError(_, ref filename, ref desc, line_col) => {
                warn!("config file failed to parse due to invalid TOML");
                info_!("{}", desc);
                info_!("in {}", filename.display());
                if let Some((line, col)) = line_col {
                    info_!("at line {}, column {}", line + 1, col + 1);
                }
            }
            BadEnvVal(ref key, ref value, ref error) => {
                warn!("environment variable ROCKET_{}={} could not be parsed", key.to_uppercase(), value);
                info_!("{}", error);
            }
            UnknownKey(ref key) => {
                warn!("the configuration key {} is unknown and disallowed in this position", key);
            }
            Missing(ref key) => {
                warn!("missing configuration key: {}", key);
            }
        }
    }

    /// Returns `true` if `self` is of `NotFound` variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::config::ConfigError;
    ///
    /// let error = ConfigError::NotFound;
    /// assert!(error.is_not_found());
    /// ```
    #[inline(always)]
    pub fn is_not_found(&self) -> bool {
        match *self {
            NotFound => true,
            _ => false
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f, "config file was not found"),
            IoError => write!(f, "I/O error while reading the config file"),
            Io(ref e, p) => write!(f, "I/O error while setting '{}': {}", p, e),
            BadFilePath(ref p, _) => write!(f, "{:?} is not a valid config path", p),
            BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` value", e),
            ParseError(..) => write!(f, "the config file contains invalid TOML"),
            UnknownKey(ref k) => write!(f, "'{}' is an unknown key", k),
            Missing(ref k) => write!(f, "missing key: '{}'", k),
            BadEntry(ref e, _) => {
                write!(f, "{:?} is not a valid `[environment]` entry", e)
            }
            BadType(ref n, e, a, _) => {
                write!(f, "type mismatch for '{}'. expected {}, found {}", n, e, a)
            }
            BadEnvVal(ref k, ref v, _) => {
                write!(f, "environment variable '{}={}' could not be parsed", k, v)
            }
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            NotFound => "config file was not found",
            IoError => "there was an I/O error while reading the config file",
            Io(..) => "an I/O error occured while setting a configuration parameter",
            BadFilePath(..) => "the config file path is invalid",
            BadEntry(..) => "an environment specified as `[environment]` is invalid",
            BadEnv(..) => "the environment specified in `ROCKET_ENV` is invalid",
            ParseError(..) => "the config file contains invalid TOML",
            BadType(..) => "a key was specified with a value of the wrong type",
            BadEnvVal(..) => "an environment variable could not be parsed",
            UnknownKey(..) => "an unknown key was used in a disallowed position",
            Missing(..) => "an expected key was not found",
        }
    }
}

impl PartialEq for ConfigError {
    fn eq(&self, other: &ConfigError) -> bool {
        match (self, other) {
            (&NotFound, &NotFound) => true,
            (&IoError, &IoError) => true,
            (&Io(_, p1), &Io(_, p2)) => p1 == p2,
            (&BadFilePath(ref p1, _), &BadFilePath(ref p2, _)) => p1 == p2,
            (&BadEnv(ref e1), &BadEnv(ref e2)) => e1 == e2,
            (&ParseError(..), &ParseError(..)) => true,
            (&UnknownKey(ref k1), &UnknownKey(ref k2)) => k1 == k2,
            (&BadEntry(ref e1, _), &BadEntry(ref e2, _)) => e1 == e2,
            (&BadType(ref n1, e1, a1, _), &BadType(ref n2, e2, a2, _)) => {
                n1 == n2 && e1 == e2 && a1 == a2
            }
            (&BadEnvVal(ref k1, ref v1, _), &BadEnvVal(ref k2, ref v2, _)) => {
                k1 == k2 && v1 == v2
            }
            (&Missing(ref k1), &Missing(ref k2)) => k1 == k2,
            (&NotFound, _) | (&IoError, _) | (&Io(..), _)
                | (&BadFilePath(..), _) | (&BadEnv(..), _) | (&ParseError(..), _)
                | (&UnknownKey(..), _) | (&BadEntry(..), _) | (&BadType(..), _)
                | (&BadEnvVal(..), _) | (&Missing(..), _) => false
        }
    }
}
