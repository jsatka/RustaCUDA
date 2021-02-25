//! Types for error handling
//!
//! # Error handling in CUDA:
//!
//! RustaCUDA uses the [`CudaError`](enum.CudaError.html) enum to represent the errors returned by
//! the CUDA API. It is important to note that nearly every function in CUDA (and therefore
//! RustaCUDA) can fail. Even those functions which have no normal failure conditions can return
//! errors related to previous asynchronous launches.

use cuda_driver_sys::{cuGetErrorString, CUresult};
use std::error::Error;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use std::result::Result;

/// Error enum which represents all the potential errors returned by the CUDA driver API.
#[repr(u32)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CudaError {
    // CUDA errors
    InvalidValue = 1,
    OutOfMemory = 2,
    NotInitialized = 3,
    Deinitialized = 4,
    ProfilerDisabled = 5,
    ProfilerNotInitialized = 6,
    ProfilerAlreadyStarted = 7,
    ProfilerAlreadyStopped = 8,
    NoDevice = 100,
    InvalidDevice = 101,
    InvalidImage = 200,
    InvalidContext = 201,
    ContextAlreadyCurrent = 202,
    MapFailed = 205,
    UnmapFailed = 206,
    ArrayIsMapped = 207,
    AlreadyMapped = 208,
    NoBinaryForGpu = 209,
    AlreadyAcquired = 210,
    NotMapped = 211,
    NotMappedAsArray = 212,
    NotMappedAsPointer = 213,
    EccUncorrectable = 214,
    UnsupportedLimit = 215,
    ContextAlreadyInUse = 216,
    PeerAccessUnsupported = 217,
    InvalidPtx = 218,
    InvalidGraphicsContext = 219,
    NvlinkUncorrectable = 220,
    InvalidSouce = 300,
    FileNotFound = 301,
    SharedObjectSymbolNotFound = 302,
    SharedObjectInitFailed = 303,
    OperatingSystemError = 304,
    InvalidHandle = 400,
    NotFound = 500,
    NotReady = 600,
    IllegalAddress = 700,
    LaunchOutOfResources = 701,
    LaunchTimeout = 702,
    LaunchIncompatibleTexturing = 703,
    PeerAccessAlreadyEnabled = 704,
    PeerAccessNotEnabled = 705,
    PrimaryContextActive = 708,
    ContextIsDestroyed = 709,
    AssertError = 710,
    TooManyPeers = 711,
    HostMemoryAlreadyRegistered = 712,
    HostMemoryNotRegistered = 713,
    HardwareStackError = 714,
    IllegalInstruction = 715,
    MisalignedAddress = 716,
    InvalidAddressSpace = 717,
    InvalidProgramCounter = 718,
    LaunchFailed = 719,
    NotPermitted = 800,
    NotSupported = 801,
    UnknownError = 999,

    // RustaCUDA errors
    InvalidMemoryAllocation = 100_100,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for CudaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CudaError::InvalidMemoryAllocation => write!(f, "Invalid memory allocation"),
            CudaError::__Nonexhaustive => write!(f, "__Nonexhaustive"),
            other if (other as u32) <= 999 => {
                let value = other as u32;
                let mut ptr: *const c_char = ptr::null();
                unsafe {
                    cuGetErrorString(mem::transmute(value), &mut ptr as *mut *const c_char)
                        .to_result()
                        .map_err(|_| fmt::Error)?;
                    let cstr = CStr::from_ptr(ptr);
                    write!(f, "{:?}", cstr)
                }
            }
            // This shouldn't happen
            _ => write!(f, "Unknown error"),
        }
    }
}

impl Error for CudaError {}

/// Result type for most CUDA functions.
pub type CudaResult<T> = Result<T, CudaError>;

/// Special result type for `drop` functions which includes the un-dropped value with the error.
pub type DropResult<T> = Result<(), (CudaError, T)>;

pub(crate) trait ToResult {
    fn to_result(self) -> CudaResult<()>;
}

impl ToResult for CUresult {
    fn to_result(self) -> CudaResult<()> {
        match self {
            CUresult::CUDA_SUCCESS => Ok(()),
            CUresult::CUDA_ERROR_INVALID_VALUE => Err(CudaError::InvalidValue),
            CUresult::CUDA_ERROR_OUT_OF_MEMORY => Err(CudaError::OutOfMemory),
            CUresult::CUDA_ERROR_NOT_INITIALIZED => Err(CudaError::NotInitialized),
            CUresult::CUDA_ERROR_DEINITIALIZED => Err(CudaError::Deinitialized),
            CUresult::CUDA_ERROR_PROFILER_DISABLED => Err(CudaError::ProfilerDisabled),
            CUresult::CUDA_ERROR_PROFILER_NOT_INITIALIZED => Err(CudaError::ProfilerNotInitialized),
            CUresult::CUDA_ERROR_PROFILER_ALREADY_STARTED => Err(CudaError::ProfilerAlreadyStarted),
            CUresult::CUDA_ERROR_PROFILER_ALREADY_STOPPED => Err(CudaError::ProfilerAlreadyStopped),
            CUresult::CUDA_ERROR_NO_DEVICE => Err(CudaError::NoDevice),
            CUresult::CUDA_ERROR_INVALID_DEVICE => Err(CudaError::InvalidDevice),
            CUresult::CUDA_ERROR_INVALID_IMAGE => Err(CudaError::InvalidImage),
            CUresult::CUDA_ERROR_INVALID_CONTEXT => Err(CudaError::InvalidContext),
            CUresult::CUDA_ERROR_CONTEXT_ALREADY_CURRENT => Err(CudaError::ContextAlreadyCurrent),
            CUresult::CUDA_ERROR_MAP_FAILED => Err(CudaError::MapFailed),
            CUresult::CUDA_ERROR_UNMAP_FAILED => Err(CudaError::UnmapFailed),
            CUresult::CUDA_ERROR_ARRAY_IS_MAPPED => Err(CudaError::ArrayIsMapped),
            CUresult::CUDA_ERROR_ALREADY_MAPPED => Err(CudaError::AlreadyMapped),
            CUresult::CUDA_ERROR_NO_BINARY_FOR_GPU => Err(CudaError::NoBinaryForGpu),
            CUresult::CUDA_ERROR_ALREADY_ACQUIRED => Err(CudaError::AlreadyAcquired),
            CUresult::CUDA_ERROR_NOT_MAPPED => Err(CudaError::NotMapped),
            CUresult::CUDA_ERROR_NOT_MAPPED_AS_ARRAY => Err(CudaError::NotMappedAsArray),
            CUresult::CUDA_ERROR_NOT_MAPPED_AS_POINTER => Err(CudaError::NotMappedAsPointer),
            CUresult::CUDA_ERROR_ECC_UNCORRECTABLE => Err(CudaError::EccUncorrectable),
            CUresult::CUDA_ERROR_UNSUPPORTED_LIMIT => Err(CudaError::UnsupportedLimit),
            CUresult::CUDA_ERROR_CONTEXT_ALREADY_IN_USE => Err(CudaError::ContextAlreadyInUse),
            CUresult::CUDA_ERROR_PEER_ACCESS_UNSUPPORTED => Err(CudaError::PeerAccessUnsupported),
            CUresult::CUDA_ERROR_INVALID_PTX => Err(CudaError::InvalidPtx),
            CUresult::CUDA_ERROR_INVALID_GRAPHICS_CONTEXT => Err(CudaError::InvalidGraphicsContext),
            CUresult::CUDA_ERROR_NVLINK_UNCORRECTABLE => Err(CudaError::NvlinkUncorrectable),
            CUresult::CUDA_ERROR_INVALID_SOURCE => Err(CudaError::InvalidSouce),
            CUresult::CUDA_ERROR_FILE_NOT_FOUND => Err(CudaError::FileNotFound),
            CUresult::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND => {
                Err(CudaError::SharedObjectSymbolNotFound)
            }
            CUresult::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED => {
                Err(CudaError::SharedObjectInitFailed)
            }
            CUresult::CUDA_ERROR_OPERATING_SYSTEM => Err(CudaError::OperatingSystemError),
            CUresult::CUDA_ERROR_INVALID_HANDLE => Err(CudaError::InvalidHandle),
            CUresult::CUDA_ERROR_NOT_FOUND => Err(CudaError::NotFound),
            CUresult::CUDA_ERROR_NOT_READY => Err(CudaError::NotReady),
            CUresult::CUDA_ERROR_ILLEGAL_ADDRESS => Err(CudaError::IllegalAddress),
            CUresult::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES => Err(CudaError::LaunchOutOfResources),
            CUresult::CUDA_ERROR_LAUNCH_TIMEOUT => Err(CudaError::LaunchTimeout),
            CUresult::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING => {
                Err(CudaError::LaunchIncompatibleTexturing)
            }
            CUresult::CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED => {
                Err(CudaError::PeerAccessAlreadyEnabled)
            }
            CUresult::CUDA_ERROR_PEER_ACCESS_NOT_ENABLED => Err(CudaError::PeerAccessNotEnabled),
            CUresult::CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE => Err(CudaError::PrimaryContextActive),
            CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED => Err(CudaError::ContextIsDestroyed),
            CUresult::CUDA_ERROR_ASSERT => Err(CudaError::AssertError),
            CUresult::CUDA_ERROR_TOO_MANY_PEERS => Err(CudaError::TooManyPeers),
            CUresult::CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED => {
                Err(CudaError::HostMemoryAlreadyRegistered)
            }
            CUresult::CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED => {
                Err(CudaError::HostMemoryNotRegistered)
            }
            CUresult::CUDA_ERROR_HARDWARE_STACK_ERROR => Err(CudaError::HardwareStackError),
            CUresult::CUDA_ERROR_ILLEGAL_INSTRUCTION => Err(CudaError::IllegalInstruction),
            CUresult::CUDA_ERROR_MISALIGNED_ADDRESS => Err(CudaError::MisalignedAddress),
            CUresult::CUDA_ERROR_INVALID_ADDRESS_SPACE => Err(CudaError::InvalidAddressSpace),
            CUresult::CUDA_ERROR_INVALID_PC => Err(CudaError::InvalidProgramCounter),
            CUresult::CUDA_ERROR_LAUNCH_FAILED => Err(CudaError::LaunchFailed),
            CUresult::CUDA_ERROR_NOT_PERMITTED => Err(CudaError::NotPermitted),
            CUresult::CUDA_ERROR_NOT_SUPPORTED => Err(CudaError::NotSupported),
            _ => Err(CudaError::UnknownError),
        }
    }
}
