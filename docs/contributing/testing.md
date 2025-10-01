# Testing Conventions

This document outlines the testing conventions for the Torrust Tracker Deploy project.

## 🧪 Unit Test Naming Style

Unit tests should use descriptive, behavior-driven naming with the `it_should_` prefix instead of the generic `test_` prefix.

### Naming Convention

- **Format**: `it_should_{describe_expected_behavior}`
- **Style**: Use lowercase with underscores, be descriptive and specific
- **Focus**: Describe what the test validates, not just what function it calls

### Examples

#### ✅ Good Test Names

```rust
#[test]
fn it_should_create_ansible_host_with_valid_ipv4() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let host = AnsibleHost::new(ip);
    assert_eq!(host.as_ip_addr(), &ip);
}

#[test]
fn it_should_fail_with_invalid_ip_address() {
    let result = AnsibleHost::from_str("invalid.ip.address");
    assert!(result.is_err());
}

#[test]
fn it_should_serialize_to_json() {
    let host = AnsibleHost::from_str("192.168.1.1").unwrap();
    let json = serde_json::to_string(&host).unwrap();
    assert_eq!(json, "\"192.168.1.1\"");
}
```

#### ❌ Avoid These Test Names

```rust
#[test]
fn test_new() { /* ... */ }

#[test]
fn test_from_str() { /* ... */ }

#[test]
fn test_serialization() { /* ... */ }
```

### Benefits

- **Clarity**: Test names clearly describe the expected behavior
- **Documentation**: Tests serve as living documentation of the code's behavior
- **BDD Style**: Follows Behavior-Driven Development naming conventions
- **Maintainability**: Easier to understand test failures and purpose

## 🧹 Resource Management

Tests should be isolated and clean up after themselves to avoid interfering with production data or leaving artifacts behind.

### Key Rules

- **Tests should clean the resources they create** - Use temporary directories or clean up generated files
- **They should not interfere with production data** - Never use real application directories like `./data` or `./build` in tests

### Example

#### ✅ Good: Using Temporary Directories

```rust
use tempfile::TempDir;

#[test]
fn it_should_create_environment_with_auto_generated_paths() {
    // Use temporary directory to avoid creating real directories
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let env_name = EnvironmentName::new("test-example".to_string()).unwrap();
    let ssh_credentials = create_test_ssh_credentials(&temp_path);
    let environment = Environment::new(env_name, ssh_credentials);

    assert!(environment.data_dir().starts_with(temp_path));
    assert!(environment.build_dir().starts_with(temp_path));

    // TempDir automatically cleans up when dropped
}
```

#### ❌ Avoid: Creating Real Directories

```rust
#[test]
fn it_should_create_environment() {
    // Don't do this - creates real directories that persist after tests
    let env_name = EnvironmentName::new("test".to_string()).unwrap();
    let environment = Environment::new(env_name, ssh_credentials);
    // Creates ./data/test and ./build/test directories
}
```

## 🎯 Principles

Test code should be held to the same quality standards as production code. Tests are not second-class citizens in the codebase.

### Core Principles

- **Maintainability**: Tests should be easy to update when requirements change
- **Readability**: Tests should be clear and understandable at first glance
- **Reliability**: Tests should be deterministic and not flaky
- **Isolation**: Each test should be independent and not affect other tests
- **Documentation**: Tests serve as living documentation of the system's behavior

Just like production code, tests should follow:

- **DRY (Don't Repeat Yourself)**: Extract common setup logic into helpers and builders
- **Single Responsibility**: Each test should verify one behavior
- **Clear Intent**: Test names and structure should make the purpose obvious
- **Clean Code**: Apply the same refactoring and quality standards as production code

Remember: **If the test code is hard to read or maintain, it will become a burden rather than an asset.**

## ✅ Good Practices

### AAA Pattern (Arrange-Act-Assert)

All tests should follow the AAA pattern, also known as Given-When-Then:

- **Arrange (Given)**: Set up the test data and preconditions
- **Act (When)**: Execute the behavior being tested
- **Assert (Then)**: Verify the expected outcome

This pattern makes tests:

- Easy to read and understand
- Clear about what is being tested
- Simple to maintain and modify

#### Example

```rust
#[test]
fn it_should_create_ansible_host_with_valid_ipv4() {
    // Arrange: Set up test data
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

    // Act: Execute the behavior
    let host = AnsibleHost::new(ip);

    // Assert: Verify the outcome
    assert_eq!(host.as_ip_addr(), &ip);
}
```

#### Benefits

- **Clarity**: Each section has a clear purpose
- **Structure**: Consistent test organization across the codebase
- **Debugging**: Easy to identify which phase is failing
- **Maintenance**: Simple to modify specific parts of the test

## 🚀 Getting Started

When writing new tests, always use the `it_should_` prefix and describe the specific behavior being validated. This makes the test suite more readable and maintainable for all contributors.
