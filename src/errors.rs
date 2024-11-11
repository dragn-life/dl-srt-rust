/*
 * dl-srt-rust
 * Copyright (C) 2024 DragN Life LLC (Adam B)
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SrtError {
  #[error("SRT error: {0}")]
  SrtError(String),
  #[error("SRT socket is disconnected")]
  Disconnected,
  #[error("SRT had an IO error: {0}")]
  ConnectionError(String),
  #[error("SRT failed to bind to port: {0}")]
  BindError(u16),
  #[error("SRT failed to listen on socket: {0}")]
  ListenError(String),
  #[error("SRT failed to accept connection on socket: {0}")]
  AcceptError(String),
  #[error("SRT failed to set socket option: {0}")]
  SetSocketOptionError(String),
  #[error("SRT failed to get socket option: {0}")]
  GetSocketOptionError(String),
  #[error("SRT failed to send data: {0}")]
  SendError(String),
  #[error("SRT failed to receive data: {0}")]
  ReceiveError(String),
}
