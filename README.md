# ZKP Chaum-Pedersen Protocol Implementation

A complete implementation of the Chaum-Pedersen Zero-Knowledge Proof protocol using Rust and Tonic.

## 📋 Overview

This project implements an authentication system using the Chaum-Pedersen protocol. Zero-knowledge proofs allow proving knowledge of secret information without revealing the information itself.

## 🚀 Features

- **Chaum-Pedersen Protocol**: Complete implementation of discrete logarithm-based zero-knowledge proof
- **gRPC Server**: Asynchronous communication server using Tonic
- **Protocol Buffers**: Type-safe message definitions
- **Random Number Generation**: Cryptographically secure random number generation
- **1024-bit Constants**: Practical security level (RFC 5114 compliant)
- **User Management**: HashMap-based user information management
- **Authentication Flow**: Complete 3-stage authentication process (Registration → Challenge → Verification)
- **Error Handling**: Proper error handling and logging
- **Comprehensive Testing**: Verification through 11 unit tests
- **Complete Client Implementation**: Full interactive client with user input and authentication flow

## 🛠️ Tech Stack

- **Rust**: Systems programming language
- **Tonic**: gRPC framework
- **Prost**: Protocol Buffers implementation
- **Tokio**: Async runtime
- **num-bigint**: Arbitrary precision integer arithmetic

## 📦 Dependencies

```toml
[dependencies]
rand = "0.8"
num-bigint = { version = "0.4", features = ["rand"] }
hex = "0.4.3"
tonic = "0.14.2"
tonic-prost = "0.14.2"
prost = "0.14.1"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

[build-dependencies]
tonic-build = "0.14.2"
tonic-prost-build = "0.14.2"
```

## 🏗️ Project Structure

```
zkp-chaum-pedersen/
├── src/
│   ├── lib.rs          # ZKP implementation and tests (11 tests, complete)
│   ├── server.rs       # gRPC server (3/3 endpoints fully implemented)
│   ├── client.rs       # gRPC client (complete implementation with full auth flow)
│   └── zkp_auth.rs     # Generated protobuf code
├── examples/
│   └── test_zero_values.rs  # Zero-value vulnerability demo
├── proto/
│   └── zkp_auth.proto  # Protocol Buffers definition
├── build.rs            # Build script
└── Cargo.toml          # Project configuration
```

## 🔧 Setup

### Prerequisites

- Rust 1.75 or higher
- Cargo

### Installation

```bash
git clone <repository-url>
cd zkp-chaum-pedersen
cargo build
```

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run zero-value vulnerability verification test
cargo test test_zero_values_with_nonzero_challenge -- --nocapture

# Run zero-value vulnerability demo
cargo run --example test_zero_values
```

### Test Coverage

- **11 Unit Tests**: Verification of ZKP protocol mathematical correctness
- **Zero-Value Vulnerability Test**: Confirmation of authentication bypass existence
- **Toy Example Tests**: Operation verification with small values
- **1024-bit Constants Test**: Verification at practical security level

## 🚀 Usage

### Starting the Server

```bash
cargo run --bin server
```

The server will display the following message when started:
```
🚀 Starting server on 127.0.0.1:50051...
📡 Server is ready to accept connections
```

### Running the Client

```bash
cargo run --bin client
```

The client will prompt you for:
1. **Username** for registration
2. **Password** for registration (used to generate y1, y2)
3. **Password** for authentication (used to solve the challenge)

**Example Output**:
```
✅ Client connected to server
Please enter username:
jiro
Please enter password:
123
✅ User registered successfully: Response { ... }
✅ Authentication challenge created successfully: AuthenticationChallengeResponse { auth_id: "k7UqwUlr8Ggj", c: [...] }
========== verify authentication ==========
Please enter password to login:
123
✅ Authentication verified successfully. Session ID: abc123def456
```

### Stopping the Server

To stop the server, press `Ctrl+C` in the terminal or run:

```bash
# Check process status
ss -tulpn | grep 50051

# Stop process
kill <PID>
```

### gRPC Client Tools

You can test using VS Code extensions (grpc-clicker) or grpcurl:

```bash
# Example with grpcurl
echo '{"user":"test","y1":"","y2":""}' | grpcurl -plaintext -d @ 127.0.0.1:50051 zkp_auth.Auth/Register
```

## 📚 Chaum-Pedersen Protocol

### Overview

The Chaum-Pedersen protocol is a zero-knowledge proof protocol based on the discrete logarithm problem.

### Parameters

- **p**: Large prime number (1024 bits)
- **q**: Prime factor of p-1
- **g**: Generator
- **h**: g^α mod p (α is secret)

### Protocol Steps

1. **Registration**: Prover sends y1 = g^x mod p, y2 = h^x mod p
2. **Challenge**: Prover sends r1 = g^k mod p, r2 = h^k mod p
3. **Response**: Verifier sends random challenge c
4. **Proof**: Prover sends s = k - c*x mod q
5. **Verification**: Verifier verifies r1 = g^s * y1^c mod p and r2 = h^s * y2^c mod p

## 🔒 Security

- **Discrete Logarithm Problem**: Security based on computational difficulty
- **Randomness**: Different random values used for each session
- **Zero-Knowledge**: No leakage of secret information

### ⚠️ Known Vulnerabilities

#### Zero-Value Authentication Bypass
**Discovery Date**: October 14, 2025
**Impact**: Critical - Complete authentication system bypass possible

**Details**:
- Sending empty strings (`""`) for `y1`, `y2`, `r1`, `r2`, `s` converts them to `BigUint::from(0u32)`
- In verification equations `r1 == (g^s * y1^c) mod p` and `r2 == (h^s * y2^c) mod p`:
  - `g^0 mod p = 1`, `h^0 mod p = 1`
  - `0^c mod p = 0` (when c > 0)
  - `1 * 0 mod p = 0`
  - Result: `0 == 0`, causing authentication to succeed

**Verification Method**:
```bash
# Verify with test
cargo test test_zero_values_with_nonzero_challenge -- --nocapture

# Run as example
cargo run --example test_zero_values
```

**Mitigation**:
- Validate that `y1`, `y2` are non-zero during registration
- Validate that `r1`, `r2` are non-zero during authentication
- Planned implementation: Input validation functionality

## 📖 API Specification

### gRPC Service

```protobuf
service Auth {
    rpc Register(RegisterRequest) returns (RegisterResponse);
    rpc CreateAuthenticationChallenge(AuthenticationChallengeRequest) returns (AuthenticationChallengeResponse);
    rpc VerifyAuthentication(AuthenticationAnswerRequest) returns (AuthenticationAnswerResponse);
}
```

### Message Types

- `RegisterRequest`: User registration (user, y1, y2)
- `RegisterResponse`: Registration response
- `AuthenticationChallengeRequest`: Authentication challenge request (user, r1, r2)
- `AuthenticationChallengeResponse`: Challenge response (auth_id, c)
- `AuthenticationAnswerRequest`: Authentication answer (auth_id, s)
- `AuthenticationAnswerResponse`: Authentication result (session_id)

### API Implementation Status

| Endpoint | Status | Description |
|---|---|---|
| `Register` | ✅ Complete | User registration functionality (y1, y2 storage) |
| `CreateAuthenticationChallenge` | ✅ Complete | Authentication challenge generation (r1, r2 storage, c generation) |
| `VerifyAuthentication` | ✅ Complete | Authentication verification functionality (ZKP verification and session management) |

## 🏗️ Implementation Status

### ✅ Completed

- **Project Setup**: Cargo.toml, build.rs, protocol definitions
- **Tonic Integration**: Complete gRPC server/client implementation
- **Version Compatibility**: Tonic 0.14.2 support
- **User Management**: HashMap-based user information management
- **Register Endpoint**: Complete user registration functionality
- **CreateAuthenticationChallenge Endpoint**: Authentication challenge functionality
- **VerifyAuthentication Endpoint**: Complete authentication verification functionality
- **Chaum-Pedersen Protocol**: Complete ZKP library implementation
- **Error Handling**: Proper error handling and logging
- **Testing**: 11 unit tests (all passing, including zero-value vulnerability test)
- **1024-bit Constants**: Implementation at practical security level
- **Session Management**: Session ID generation upon successful authentication
- **Complete Client Implementation**: Full interactive client with complete authentication flow

### 🚧 In Development

- **None** - All core functionality is complete

### 📋 Future Plans

- **Security Enhancement**: Zero-value vulnerability fix (input validation implementation)
- **Session Management Extension**: Session expiration, session invalidation functionality
- **Performance Optimization**: Large-scale user support
- **Documentation**: Detailed API specification documentation
- **Logging Functionality**: Detailed authentication logs and audit functionality

## 📄 License

This project is released under the MIT License. See the `LICENSE` file for details.

## 🐛 Troubleshooting

### Common Issues

#### Server Won't Start
```bash
# Check port usage
ss -tulpn | grep 50051

# Stop existing process
kill <PID>
```

#### gRPC Client Tool Errors
```bash
# If grpcurl is not installed
wget https://github.com/fullstorydev/grpcurl/releases/download/v1.8.7/grpcurl_1.8.7_linux_x86_64.tar.gz
tar -xzf grpcurl_1.8.7_linux_x86_64.tar.gz
sudo mv grpcurl /usr/local/bin/
```

#### Postman gRPC Testing Errors
```bash
# Error: "Message violates its Protobuf type definition"
# Cause: Sending string "0" to bytes type field
# Solution: Send empty string "" or Base64 encoded value

# Correct input example
{
  "user": "jirok",
  "y1": "",     # Empty string (empty byte array)
  "y2": ""      # Empty string (empty byte array)
}

# Or
{
  "user": "jirok",
  "y1": "AA==",  # Base64 for empty bytes
  "y2": "AA=="   # Base64 for empty bytes
}
```

#### Build Errors
```bash
# Update dependencies
cargo update

# Clean build
cargo clean
cargo build
```

## 🔗 References

- [Chaum-Pedersen Protocol](https://crypto.stackexchange.com/questions/99262/chaum-pedersen-protocol)
- [Cryptography: An Introduction (3rd Edition)](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf)
- [Tonic Documentation](https://github.com/hyperium/tonic)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)

## 🌐 Internationalization

- **English**: This README.md
- **Japanese**: [README.jp.md](README.jp.md)
