#[cfg(feature = "apple-auth")]
pub use self::apple::apple_auth;
pub use self::des::encrypt as des;

#[cfg(feature = "apple-auth")]
mod apple;
mod des;
#[cfg(feature = "apple-auth")]
mod md5;
