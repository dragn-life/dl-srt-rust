/*
 * dl-srt-rust
 * Copyright (C) 2024 DragN Life LLC (Adam B)
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */
use std::os::raw::{c_char, c_int};
use std::ffi::{CString, CStr};
use std::io::{Error, Result};

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
  fn srt_accept(sock: SRTSOCKET) -> SRTSOCKET;

  fn srt_close(sock: SRTSOCKET) -> SRTSOCKET;
  fn srt_setsockopt(sock: SRTSOCKET, level: c_int, optname: SrtSocketOptions, optval: *const c_char, optlen: c_int) -> c_int;
  fn srt_getsockflag(sock: SRTSOCKET, optname: SrtSocketOptions, optval: *mut c_char, optlen: *mut c_int) -> c_int;

  fn srt_send(sock: SRTSOCKET, buf: *const c_char, len: c_int, flags: c_int) -> c_int;
  fn srt_recv(sock: SRTSOCKET, buf: *mut c_char, len: c_int, flags: c_int) -> c_int;

  fn srt_getlasterror_str() -> *const c_char;

  fn srt_getsockstate(sock: SRTSOCKET) -> SrtSocketStatus;
}

pub type SRTSOCKET = c_int;

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

#[repr(C)]
pub enum SrtSocketOptions {
  SrtOptRCVSYN = 2,
  SrtOptReuseAddr = 15,
  SrtOptStreamID = 46,
}

pub struct SrtSocketConnection {
  sock: SRTSOCKET,
}

#[repr(C)]
pub struct SockAddrIn {
  pub sin_family: u16,
  pub sin_port: u16,
  pub sin_addr: u32,
  pub sin_zero: [u8; 8],
}

pub fn startup_srt() -> Result<()> {
  let ret = unsafe { srt_startup() };
  if ret == -1 {
    return Err(Error::last_os_error());
  }
  Ok(())
}

pub fn cleanup_srt() -> Result<()> {
  let ret = unsafe { srt_cleanup() };
  if ret == -1 {
    return Err(Error::last_os_error());
  }
  Ok(())
}

impl SrtSocketConnection {
  pub fn new() -> Result<Self> {
    let sock = unsafe { srt_create_socket() };
    if sock == -1 {
      return Err(Error::last_os_error());
    }
    Ok(Self { sock })
  }

  pub fn bind(&self, port: u16) -> Result<()> {
    let sockaddr = SockAddrIn {
      sin_family: 2,
      sin_port: port.to_be(),
      sin_addr: 0,
      sin_zero: [0; 8],
    };
    let ret = unsafe { srt_bind(self.sock, &sockaddr, size_of::<SockAddrIn>() as i32) };
    if ret == -1 {
      return Err(Error::last_os_error());
    }
    Ok(())
  }

  pub fn listen(&self, backlog: i32) -> Result<()> {
    let ret = unsafe { srt_listen(self.sock, backlog) };
    if ret == -1 {
      return Err(Error::last_os_error());
    }
    Ok(())
  }

  pub fn accept(&self) -> Result<Self> {
    let sock = unsafe { srt_accept(self.sock) };
    if sock == -1 {
      return Err(Error::last_os_error());
    }
    Ok(Self { sock })
  }

  pub fn set_sock_opt(&self, level: c_int, optname: SrtSocketOptions, optval: &str) -> Result<()> {
    let optval = CString::new(optval).unwrap();
    let ret = unsafe { srt_setsockopt(self.sock, level, optname, optval.as_ptr(), optval.as_bytes().len() as i32) };
    if ret == -1 {
      return Err(Error::last_os_error());
    }
    Ok(())
  }

  pub fn get_sock_flag(&self, optname: SrtSocketOptions) -> Result<String> {
    let mut optval = [0 as c_char; 256];
    let mut optlen = 256;
    let ret = unsafe { srt_getsockflag(self.sock, optname, optval.as_mut_ptr(), &mut optlen) };
    if ret == -1 {
      return Err(Error::last_os_error());
    }
    let optval = unsafe { CStr::from_ptr(optval.as_ptr()) };
    Ok(optval.to_str().unwrap().to_string())
  }

  pub fn send(&self, buf: &str) -> Result<()> {
    let buf = CString::new(buf).unwrap();
    let ret = unsafe { srt_send(self.sock, buf.as_ptr(), buf.as_bytes().len() as i32, 0) };
    if ret == -1 {
      return Err(Error::last_os_error());
    }
    Ok(())
  }

  pub fn recv(&self, len: i32) -> Result<String> {
    let mut buf = vec![0 as c_char; len as usize];
    let ret = unsafe { srt_recv(self.sock, buf.as_mut_ptr(), len, 0) };
    if ret == -1 {
      return Err(Error::last_os_error());
    }
    let buf = unsafe { CStr::from_ptr(buf.as_ptr()) };
    Ok(buf.to_str().unwrap().to_string())
  }

  pub fn get_last_srt_error() -> String {
    let err = unsafe { srt_getlasterror_str() };
    let err = unsafe { CStr::from_ptr(err) };
    err.to_str().unwrap().to_string()
  }

  pub fn get_socket_state(&self) -> SrtSocketStatus {
    unsafe { srt_getsockstate(self.sock) }
  }
}

impl Drop for SrtSocketConnection {
  fn drop(&mut self) {
    unsafe { srt_close(self.sock) };
  }
}
