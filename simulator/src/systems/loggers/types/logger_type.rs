use serde::Serialize;

/// used to create
/// specific Logger Implementation
pub trait LoggerType: Send + Sync {
    fn open(filename: &str) -> Self;
    fn write<S: Serialize>(&mut self, record: S);
}