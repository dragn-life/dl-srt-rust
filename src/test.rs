/*
 * dl-srt-rust
 * Copyright (C) 2024 DragN Life LLC (Adam B)
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */
use super::*;

const TEST_PORT: u16 = 9000;
const TEST_STREAM_ID: &str = "test_stream";

#[test]
fn test_srt_lifecycle() {
  assert!(startup_srt().is_ok(), "SRT startup failed");
  assert!(cleanup_srt().is_ok(), "SRT cleanup failed");
}

#[test]
fn test_socket_creation() {
  setup();

  let socket = SrtSocketConnection::new();
  assert!(socket.is_ok(), "Socket creation failed");

  teardown();
}

#[test]
fn test_socket_binding() {
  setup();

  let socket = SrtSocketConnection::new().expect("Socket creation failed");
  assert!(socket.bind(TEST_PORT).is_ok(), "Socket binding failed: {}", SrtSocketConnection::get_last_srt_error());
  teardown();
}

#[test]
fn test_socket_state() {
  setup();

  let socket = SrtSocketConnection::new().expect("Socket creation failed");
  assert!(matches!(socket.get_socket_state(),
  SrtSocketStatus::SrtStatusOpened | SrtSocketStatus::SrtStatusInit));

  teardown();
}

#[test]
fn test_socket_options() {
  setup();

  let socket = SrtSocketConnection::new().expect("Socket creation failed");

  // Test setting stream ID
  assert!(socket.set_sock_opt(0, SrtSocketOptions::SrtOptStreamID, TEST_STREAM_ID).is_ok(), "Failed to set stream ID: {}", SrtSocketConnection::get_last_srt_error());

  // Test getting stream ID
  let stream_id = socket.get_sock_flag(SrtSocketOptions::SrtOptStreamID).expect("Failed to get stream ID");
  assert_eq!(stream_id, TEST_STREAM_ID, "Stream ID mismatch");

  teardown();
}

#[test]
fn test_error_handling() {
  startup_srt().expect("SRT startup failed");

  // Test invalid socket operations
  let invalid_socket = SrtSocketConnection { sock: -1 };
  let test_data: &[u8] = b"test";
  assert!(invalid_socket.send(test_data).is_err(), "Send should fail on invalid socket");
  assert!(invalid_socket.recv(256).is_err(), "Recv should fail on invalid socket");

  // Test error string retrieval
  let error_str = SrtSocketConnection::get_last_srt_error();
  assert!(!error_str.is_empty(), "Error string should not be empty");

  cleanup_srt().expect("SRT cleanup failed");
}

// Test Setup and Teardown
fn setup() {
  startup_srt().expect("SRT startup failed");
}

fn teardown() {
  cleanup_srt().expect("SRT cleanup failed");
}
