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
use serial_test::serial;

const TEST_PORT: u16 = 9000;

#[test]
#[serial]
fn test_srt_lifecycle() {
  assert!(startup_srt().is_ok(), "SRT startup failed");
  assert!(cleanup_srt().is_ok(), "SRT cleanup failed");
}

#[test]
#[serial]
fn test_socket_creation() {
  setup();

  let socket = SrtSocketConnection::new();
  assert!(socket.is_ok(), "Socket creation failed");

  teardown();
}

#[test]
#[serial]
fn test_socket_binding() {
  setup();

  let socket = SrtSocketConnection::new().expect("Socket creation failed");
  match socket.bind(TEST_PORT) {
    Ok(_) => assert!(true),
    Err(e) => panic!("Socket binding failed: {}", e),
  }

  teardown();
}

#[test]
#[serial]
fn test_socket_state() {
  setup();

  let socket = SrtSocketConnection::new().expect("Socket creation failed");
  assert!(matches!(
    socket.get_socket_state(),
    SrtSocketStatus::SrtStatusOpened | SrtSocketStatus::SrtStatusInit
  ));

  teardown();
}

#[test]
#[serial]
fn test_socket_options() {
  setup();

  let socket = SrtSocketConnection::new().expect("Socket creation failed");

  // Test Setting/Getting Boolean Options
  let value = true;
  match socket.set_sock_opt(
    0,
    SrtSocketOptions::SrtOptReuseAddr,
    SrtOptionValue::Bool(value),
  ) {
    Ok(_) => assert!(true),
    Err(e) => panic!("Failed to set SrtOptReuseAddr: {}", e),
  }

  let ret = match socket.get_sock_flag(SrtSocketOptions::SrtOptReuseAddr) {
    Ok(SrtOptionValue::Bool(val)) => val,
    Err(e) => panic!("Failed to get SrtOptReuseAddr: {}", e),
    _ => panic!("Expected boolean value for SrtOptReuseAddr"),
  };
  assert_eq!(ret, value, "SrtOptReuseAddr mismatch");

  let value = false;
  match socket.set_sock_opt(
    0,
    SrtSocketOptions::SrtOptRCVSYN,
    SrtOptionValue::Bool(value),
  ) {
    Ok(_) => assert!(true),
    Err(e) => panic!("Failed to set SrtOptRCVSYN: {}", e),
  }

  let ret = match socket.get_sock_flag(SrtSocketOptions::SrtOptRCVSYN) {
    Ok(SrtOptionValue::Bool(val)) => val,
    Err(e) => panic!("Failed to get SrtOptRCVSYN: {}", e),
    _ => panic!("Expected boolean value for SrtOptRCVSYN"),
  };
  assert_eq!(ret, value, "SrtOptRCVSYN mismatch");

  // Test Setting/Getting Integer Options
  let value = 256;
  match socket.set_sock_opt(
    0,
    SrtSocketOptions::SrtOptLatency,
    SrtOptionValue::Int(value),
  ) {
    Ok(_) => assert!(true),
    Err(e) => panic!("Failed to set SrtOptLatency: {}", e),
  }

  let ret = match socket.get_sock_flag(SrtSocketOptions::SrtOptLatency) {
    Ok(SrtOptionValue::Int(val)) => val,
    Err(e) => panic!("Failed to get SrtOptLatency: {}", e),
    _ => panic!("Expected integer value for SrtOptLatency"),
  };
  assert_eq!(ret, value, "SrtOptLatency mismatch");

  let value = 123;
  match socket.set_sock_opt(
    0,
    SrtSocketOptions::SrtOptRCVLatency,
    SrtOptionValue::Int(value),
  ) {
    Ok(_) => assert!(true),
    Err(e) => panic!("Failed to set SrtOptRCVLatency: {}", e),
  }

  let ret = match socket.get_sock_flag(SrtSocketOptions::SrtOptRCVLatency) {
    Ok(SrtOptionValue::Int(val)) => val,
    Err(e) => panic!("Failed to get SrtOptRCVLatency: {}", e),
    _ => panic!("Expected integer value for SrtOptRCVLatency"),
  };
  assert_eq!(ret, value, "SrtOptRCVLatency mismatch");

  let value = 1_000_000_000;
  match socket.set_sock_opt(
    0,
    SrtSocketOptions::SrtOptPeerLatency,
    SrtOptionValue::Int(value),
  ) {
    Ok(_) => assert!(true),
    Err(e) => panic!("Failed to set SrtOptPeerLatency: {}", e),
  }

  let ret = match socket.get_sock_flag(SrtSocketOptions::SrtOptPeerLatency) {
    Ok(SrtOptionValue::Int(val)) => val,
    Err(e) => panic!("Failed to get SrtOptPeerLatency: {}", e),
    _ => panic!("Expected integer value for SrtOptPeerLatency"),
  };
  assert_eq!(ret, value, "SrtOptPeerLatency mismatch");

  teardown();
}

#[test]
#[serial]
fn test_stream_id_flag() {
  let very_long_string = "a".repeat(511);
  let test_cases = vec![
    "test",
    "space test",
    "",
    "あいうえお", // Non-ASCII Japanese characters
    very_long_string.as_str(),
  ];

  for test_value in test_cases {
    setup();

    let socket = SrtSocketConnection::new().expect("Socket creation failed");

    let value = String::from(test_value);

    // Test setting the Stream ID
    match socket.set_sock_opt(
      0,
      SrtSocketOptions::SrtOptStreamID,
      SrtOptionValue::String(value.clone()),
    ) {
      Ok(_) => assert!(true),
      Err(e) => panic!("Failed to set SrtOptStreamID with '{}': {}", value, e),
    }

    // Test getting the Stream ID
    let ret = match socket.get_sock_flag(SrtSocketOptions::SrtOptStreamID) {
      Ok(SrtOptionValue::String(val)) => val,
      Ok(other) => panic!("Expected string value for SrtOptStreamID, got {:?}", other),
      Err(e) => panic!("Failed to get SrtOptStreamID: {}", e),
    };
    assert_eq!(
      ret, value,
      "Stream ID mismatch for test case '{}'\nExpected: {}\nGot: {}",
      test_value, value, ret
    );
    teardown();
  }
}

#[test]
#[serial]
fn test_error_handling() {
  startup_srt().expect("SRT startup failed");

  // Test invalid socket operations
  let invalid_socket = SrtSocketConnection { sock: -1 };
  let test_data: &[u8] = b"test";

  match invalid_socket.send(test_data) {
    Ok(_) => panic!("Send should fail on invalid socket"),
    Err(e) => assert!(matches!(e, SrtError::SendError(_))),
  }

  match invalid_socket.recv(256) {
    Ok(_) => panic!("Recv should fail on invalid socket"),
    Err(e) => assert!(matches!(e, SrtError::ReceiveError(_))),
  }

  // Test error string retrieval
  let error_str = get_last_srt_error();
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
