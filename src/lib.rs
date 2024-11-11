/*
 * dl-srt-rust
 * Copyright (C) 2024 DragN Life LLC (Adam B)
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

// Optional Tracing
#[cfg(feature = "tracing")]
use tracing::{warn};

use crate::errors::SrtError;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;

pub mod errors;

// Declare test module
#[cfg(test)]
mod test;

#[link(name = "srt")]
extern "C" {
  fn srt_startup() -> c_int;
  fn srt_cleanup() -> c_int;

  fn srt_create_socket() -> SRTSOCKET;
  fn srt_bind(sock: SRTSOCKET, addr: *const SockAddrIn, addrlen: c_int) -> c_int;

  fn srt_listen(sock: SRTSOCKET, backlog: c_int) -> c_int;
  fn srt_accept(sock: SRTSOCKET, addr: *mut libc::sockaddr, addrlen: *mut c_int) -> SRTSOCKET;

  fn srt_close(sock: SRTSOCKET) -> SRTSOCKET;
  fn srt_setsockopt(
    sock: SRTSOCKET,
    level: c_int,
    optname: SrtSocketOptions,
    optval: *const c_char,
    optlen: c_int,
  ) -> c_int;
  fn srt_getsockflag(
    sock: SRTSOCKET,
    optname: SrtSocketOptions,
    optval: *mut c_char,
    optlen: *mut c_int,
  ) -> c_int;

  fn srt_send(sock: SRTSOCKET, buf: *const c_char, len: c_int, flags: c_int) -> c_int;
  fn srt_recv(sock: SRTSOCKET, buf: *mut c_char, len: c_int, flags: c_int) -> c_int;

  fn srt_getlasterror_str() -> *const c_char;

  fn srt_getsockstate(sock: SRTSOCKET) -> SrtSocketStatus;
}

pub type SRTSOCKET = c_int;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum SrtSocketStatus {
  SrtStatusInit = 1,
  SrtStatusOpened,
  SrtStatusListening,
  SrtStatusConnecting,
  SrtStatusConnected,
  SrtStatusBroken,
  SrtStatusClosing,
  SrtStatusClosed,
  SrtStatusNonExist,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum SrtSocketOptions {
  SrtOptRCVSYN = 2,
  SrtOptReuseAddr = 15,
  SrtOptLatency = 23,
  SrtOptRCVLatency = 43,
  SrtOptPeerLatency = 44,
  SrtOptStreamID = 46,
}

#[derive(Debug)]
pub enum SrtOptionValue {
  Bool(bool),
  Int(i32),
  String(String),
}

#[derive(Debug)]
pub struct SrtSocketConnection {
  sock: SRTSOCKET,
}
// Display for SrtSocketConnection
impl std::fmt::Display for SrtSocketConnection {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SRTSOCKET: {}", self.sock)
  }
}

#[repr(C)]
pub struct SockAddrIn {
  pub sin_family: u16,
  pub sin_port: u16,
  pub sin_addr: u32,
  pub sin_zero: [u8; 8],
}

fn get_last_srt_error() -> String {
  let err_ptr = unsafe { srt_getlasterror_str() };

  if err_ptr.is_null() {
    return String::from("Unknown SRT error (null error)");
  }

  let c_string = unsafe { CStr::from_ptr(err_ptr) };

  match c_string.to_str() {
    Ok(s) => s.to_string(),
    Err(_) => String::from("Invalid UTF-8 in SRT error message"),
  }
}

pub fn startup_srt() -> Result<(), SrtError> {
  let ret = unsafe { srt_startup() };
  if ret == -1 {
    return Err(SrtError::SrtError(format!(
      "Failed to start SRT: {}",
      get_last_srt_error()
    )));
  }
  Ok(())
}

pub fn cleanup_srt() -> Result<(), SrtError> {
  let ret = unsafe { srt_cleanup() };
  if ret == -1 {
    return Err(SrtError::SrtError(format!(
      "Failed to cleanup SRT: {}",
      get_last_srt_error()
    )));
  }
  Ok(())
}

impl SrtSocketConnection {
  pub fn new() -> Result<Self, SrtError> {
    let sock = unsafe { srt_create_socket() };
    if sock == -1 {
      return Err(SrtError::SrtError(format!(
        "Failed to create SRT socket: {}",
        get_last_srt_error()
      )));
    }
    Ok(Self { sock })
  }

  pub fn bind(&self, port: u16) -> Result<(), SrtError> {
    let sockaddr = SockAddrIn {
      sin_family: 2,
      sin_port: port.to_be(),
      sin_addr: 0,
      sin_zero: [0; 8],
    };
    let ret = unsafe { srt_bind(self.sock, &sockaddr, size_of::<SockAddrIn>() as i32) };
    if ret == -1 {
      return Err(SrtError::BindError(port));
    }
    Ok(())
  }

  pub fn listen(&self, backlog: i32) -> Result<(), SrtError> {
    let ret = unsafe { srt_listen(self.sock, backlog) };
    if ret == -1 {
      return Err(SrtError::ListenError(get_last_srt_error()));
    }
    Ok(())
  }

  pub fn accept(&self) -> Result<Self, SrtError> {
    let sock = unsafe { srt_accept(self.sock, ptr::null_mut(), ptr::null_mut()) };
    if sock == -1 {
      return Err(SrtError::AcceptError(get_last_srt_error()));
    }
    Ok(Self { sock })
  }

  pub fn close(&self) -> Result<(), SrtError> {
    let ret = unsafe { srt_close(self.sock) };
    if ret == -1 {
      return Err(SrtError::SrtError(format!(
        "Failed to close SRT socket: {}",
        get_last_srt_error()
      )));
    }
    Ok(())
  }

  pub fn set_sock_opt(
    &self,
    level: c_int,
    opt_name: SrtSocketOptions,
    opt_value: SrtOptionValue,
  ) -> Result<(), SrtError> {
    match opt_value {
      SrtOptionValue::Bool(bool_value) => {
        let val: i32 = if bool_value { 1 } else { 0 };
        let bytes = val.to_ne_bytes();
        let ret = unsafe {
          srt_setsockopt(
            self.sock,
            level,
            opt_name,
            bytes.as_ptr() as *const c_char,
            size_of::<i32>() as i32,
          )
        };
        if ret == -1 {
          return Err(SrtError::SetSocketOptionError(get_last_srt_error()));
        }
      }
      SrtOptionValue::Int(int_value) => {
        let bytes = int_value.to_ne_bytes();
        let ret = unsafe {
          srt_setsockopt(
            self.sock,
            level,
            opt_name,
            bytes.as_ptr() as *const c_char,
            size_of::<i32>() as i32,
          )
        };
        if ret == -1 {
          return Err(SrtError::SetSocketOptionError(get_last_srt_error()));
        }
      }
      SrtOptionValue::String(string_value) => {
        // Convert the string to bytes directly
        let bytes = string_value.as_bytes();

        let ret = unsafe {
          srt_setsockopt(
            self.sock,
            level,
            opt_name,
            bytes.as_ptr() as *const c_char,
            bytes.len() as i32,
          )
        };
        if ret == -1 {
          return Err(SrtError::SetSocketOptionError(get_last_srt_error()));
        }
      }
    }
    Ok(())
  }

  pub fn get_sock_flag(&self, opt_name: SrtSocketOptions) -> Result<SrtOptionValue, SrtError> {
    // Start with a large buffer size
    let mut buffer = vec![0u8; 512];
    let mut buffer_size = buffer.len() as c_int;

    let ret = unsafe {
      srt_getsockflag(
        self.sock,
        opt_name,
        buffer.as_mut_ptr() as *mut c_char,
        &mut buffer_size as *mut c_int,
      )
    };
    if ret == -1 {
      return Err(SrtError::GetSocketOptionError(get_last_srt_error()));
    }

    // Different options have different types
    match opt_name {
      // Boolean options
      SrtSocketOptions::SrtOptRCVSYN | SrtSocketOptions::SrtOptReuseAddr => {
        if buffer_size != 1 {
          return Err(SrtError::GetSocketOptionError(String::from(
            "Invalid boolean value",
          )));
        }
        let value = i32::from_ne_bytes(
          buffer[..std::mem::size_of::<i32>()]
            .try_into()
            .map_err(|_| SrtError::GetSocketOptionError(String::from("Invalid boolean value")))?,
        );
        Ok(SrtOptionValue::Bool(value != 0))
      }
      // Integer options
      SrtSocketOptions::SrtOptLatency
      | SrtSocketOptions::SrtOptRCVLatency
      | SrtSocketOptions::SrtOptPeerLatency => {
        let value = i32::from_ne_bytes(
          buffer[..buffer_size as usize]
            .try_into()
            .map_err(|_| SrtError::GetSocketOptionError(String::from("Invalid integer value")))?,
        );
        Ok(SrtOptionValue::Int(value))
      }
      // String Options
      SrtSocketOptions::SrtOptStreamID => {
        // Trim Buffer
        buffer.truncate(buffer_size as usize);
        if buffer.is_empty() {
          return Ok(SrtOptionValue::String(String::new()));
        }
        let string_length = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());

        // Create a string from the buffer
        let value = match String::from_utf8(buffer[..string_length].to_vec()) {
          Ok(s) => s,
          Err(_) => {
            return Err(SrtError::GetSocketOptionError(
              "Invalid UTF-8 in String Flag".into(),
            ))
          }
        };
        Ok(SrtOptionValue::String(value))
      }
    }
  }

  pub fn send(&self, data: &[u8]) -> Result<(), SrtError> {
    let ret = unsafe {
      srt_send(
        self.sock,
        data.as_ptr() as *const c_char,
        data.len() as i32,
        0,
      )
    };
    if ret == -1 {
      return Err(SrtError::SendError(get_last_srt_error()));
    }
    Ok(())
  }

  pub fn recv(&self, len: i32) -> Result<Vec<u8>, SrtError> {
    // Create buffer to store received data
    let mut buf = vec![0u8; len as usize];
    let bytes_received = unsafe { srt_recv(self.sock, buf.as_mut_ptr() as *mut c_char, len, 0) };
    if bytes_received == -1 {
      return Err(SrtError::ReceiveError(get_last_srt_error()));
    }
    // Truncate buffer to actual received bytes
    buf.truncate(bytes_received as usize);
    Ok(buf)
  }

  pub fn get_socket_state(&self) -> SrtSocketStatus {
    unsafe { srt_getsockstate(self.sock) }
  }
}

impl Drop for SrtSocketConnection {
  fn drop(&mut self) {
    match self.close() {
      Ok(_) => (),
      Err(_e) => {
        #[cfg(feature = "tracing")]
        warn!("Failed to close SRT socket: {}", _e)
      }
    };
  }
}
