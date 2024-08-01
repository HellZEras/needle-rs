use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappedErrors {
    #[error("Memory allocation failed")]
    MemoryAllocationFailure,
    #[error("Writing process memory failed")]
    MemoryWritingFailure,
    #[error("Getting module handle failed")]
    ModuleHandleFailure,
    #[error("Getting function address failed")]
    ProcAddressFailure,
    #[error("Thread creation failed")]
    ThreadCreationFailure,
    #[error("Waiting for single object was abandoned")]
    WaitForSingleObjectAbandoned,
    #[error("Waiting for single object failed")]
    WaitForSingleObjectFailure,
    #[error("Waiting for single object timed out")]
    WaitForSingleObjectTimeOut,
    #[error("Unexpected behavior")]
    WaitForSingleObjectUnknown,
    #[error("Opening processing Failed")]
    ProcessOpeningFailure,
    #[error("Getting module basename failed")]
    ModuleBaseNameFailure,
    #[error("Creating snapshot failed")]
    SnapshotCreationFailure,
    #[error("Getting next process failed")]
    NextProcessFailure,
    #[error("Specified process was not found")]
    ProcessNotFound,
    #[error("Failed to check process architecture")]
    ProcessCheckFailure,
    #[error("DLL and Process must be of the same architecture")]
    ArchitectureMismatch,
}
