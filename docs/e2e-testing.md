# E2E Testing Guide

This guide explains how to run and understand the End-to-End (E2E) tests for the Torrust Tracker Deploy project.

## 🧪 What are E2E Tests?

The E2E tests validate the complete deployment process using two independent test suites:

1. **E2E Provision Tests** - Test infrastructure provisioning using LXD VMs
2. **E2E Configuration Tests** - Test software installation and configuration using Docker containers

This split approach ensures reliable testing in CI environments while maintaining comprehensive coverage.

## 🚀 Running E2E Tests

### Independent Test Suites

#### Provision Tests

Test infrastructure provisioning only (VM creation and cloud-init):

```bash
cargo run --bin e2e-provision-tests
```

#### Configuration Tests

Test software installation and configuration (Ansible playbooks):

```bash
cargo run --bin e2e-config-tests
```

#### Full Local Testing

For local development, you can run the complete end-to-end test:

```bash
cargo run --bin e2e-tests-full
```

⚠️ **Note**: The `e2e-tests-full` binary cannot run on GitHub Actions due to network connectivity issues, but is useful for local validation.

### Command Line Options

All test binaries support these options:

- `--keep` - Keep the test environment after completion (useful for debugging)
- `--templates-dir` - Specify custom templates directory path
- `--help` - Show help information

### Examples

```bash
# Run provision tests only
cargo run --bin e2e-provision-tests

# Run configuration tests with debugging
cargo run --bin e2e-config-tests -- --keep

# Run full local tests with custom templates
cargo run --bin e2e-tests-full -- --templates-dir ./custom/templates
```

## 📋 Test Sequences

### E2E Provision Tests (`e2e-provision-tests`)

Tests infrastructure provisioning using LXD VMs:

1. **Infrastructure Provisioning**

   - Uses OpenTofu configuration from `templates/tofu/lxd/`
   - Creates LXD container with Ubuntu and cloud-init configuration

2. **Cloud-init Completion**
   - Waits for cloud-init to finish system initialization
   - Validates user accounts and SSH key setup
   - Verifies basic network interface setup

**Validation**:

- ✅ VM is created and running
- ✅ Cloud-init status is "done"
- ✅ Boot completion marker file exists (`/var/lib/cloud/instance/boot-finished`)

### E2E Configuration Tests (`e2e-config-tests`)

Tests software installation and configuration using Docker containers:

1. **Container Setup**

   - Creates Docker container from `docker/provisioned-instance/`
   - Configures SSH connectivity for Ansible

2. **Software Installation** (`install-docker.yml`)

   - Installs Docker Community Edition
   - Configures Docker service
   - Validates Docker daemon is running

3. **Docker Compose Installation** (`install-docker-compose.yml`)
   - Installs Docker Compose binary
   - Validates installation with test configuration

**Validation**:

- ✅ Container is accessible via SSH
- ✅ Docker version command works
- ✅ Docker daemon service is active
- ✅ Docker Compose version command works
- ✅ Can parse and validate a test docker-compose.yml file

### Full Local Tests (`e2e-tests-full`)

Combines both provision and configuration phases in a single LXD VM for comprehensive local testing.

## 🛠️ Prerequisites

### For E2E Provision Tests

1. **LXD installed and configured**

   ```bash
   sudo snap install lxd
   sudo lxd init  # Follow the setup prompts
   ```

2. **OpenTofu installed**

   ```bash
   # Installation instructions in docs/tech-stack/opentofu.md
   ```

### For E2E Configuration Tests

1. **Docker installed**

   ```bash
   # Docker is available on most systems or in CI environments
   docker --version
   ```

2. **Ansible installed**

   ```bash
   # Installation instructions in docs/tech-stack/ansible.md
   ```

### For Full Local Tests (`e2e-tests-full`)

Requires **all** of the above: LXD, OpenTofu, Docker, and Ansible.

## 🐛 Troubleshooting

### Test Environment Cleanup

#### Provision Tests Cleanup

If provision tests fail and leave LXD resources behind:

```bash
# Check running containers
lxc list

# Stop and delete the test container
lxc stop torrust-tracker-vm
lxc delete torrust-tracker-vm

# Or use OpenTofu to clean up
cd build/tofu/lxd
tofu destroy -auto-approve
```

#### Configuration Tests Cleanup

If configuration tests fail and leave Docker resources behind:

```bash
# Check running containers
docker ps -a

# Stop and remove test containers
docker stop $(docker ps -q --filter "ancestor=torrust-provisioned-instance")
docker rm $(docker ps -aq --filter "ancestor=torrust-provisioned-instance")

# Remove test images if needed
docker rmi torrust-provisioned-instance
```

### Common Issues by Test Suite

#### Provision Tests Issues

- **LXD daemon not running**: `sudo systemctl start lxd`
- **Insufficient privileges**: Ensure your user is in the `lxd` group
- **OpenTofu state corruption**: Delete `build/tofu/lxd/terraform.tfstate` and retry
- **Cloud-init timeout**: VM may need more time; check `lxc exec torrust-tracker-vm -- cloud-init status`

#### Configuration Tests Issues

- **Docker daemon not running**: `sudo systemctl start docker`
- **Container build failures**: Check Docker image build logs
- **SSH connectivity to container**: Verify container networking and SSH service
- **Ansible connection errors**: Check container SSH configuration and key permissions

#### Full Local Tests Issues

- **Network connectivity in VMs**: Known limitation - use split test suites for reliable testing
- **SSH connectivity failures**: Usually means cloud-init is still running or SSH configuration failed
- **Mixed infrastructure issues**: Combines all provision and configuration issues above

### Test Suite Selection Guide

**Use Provision Tests (`e2e-provision-tests`) when**:

- Testing infrastructure changes (OpenTofu, LXD configuration)
- Validating VM creation and cloud-init setup
- Working on provisioning-related features

**Use Configuration Tests (`e2e-config-tests`) when**:

- Testing Ansible playbooks and software installation
- Validating configuration management changes
- Working on application deployment features

**Use Full Local Tests (`e2e-tests-full`) when**:

- Comprehensive local validation before CI
- Integration testing of provision + configuration
- Debugging end-to-end deployment issues

### CI Network Issues

**Problem**: GitHub Actions runners experience intermittent network connectivity problems within LXD VMs that cause:

- Docker GPG key downloads to fail (`Network is unreachable` errors)
- Package repository access timeouts
- Generally flaky network behavior

**Root Cause**: This is a known issue with GitHub-hosted runners:

- [GitHub Issue #13003](https://github.com/actions/runner-images/issues/13003) - Network connectivity issues with LXD VMs
- [GitHub Issue #1187](https://github.com/actions/runner-images/issues/1187) - Original networking issue
- [GitHub Issue #2890](https://github.com/actions/runner-images/issues/2890) - Specific apt repository timeout issues

**Solution**: We split E2E tests into two suites:

- **Provision Tests**: Use LXD VMs for infrastructure testing only (no network-heavy operations inside VM)
- **Configuration Tests**: Use Docker containers which have reliable network connectivity on GitHub Actions
- **Full Local Tests**: Available for comprehensive local testing where network connectivity works

**Implementation**: Configuration tests use Docker containers with:

- Direct internet access for package downloads
- Reliable networking for Ansible connectivity
- No nested virtualization issues

### Debug Mode

Use the `--keep` flag to inspect the environment after test completion:

#### Provision Tests Debugging

```bash
cargo run --bin e2e-provision-tests -- --keep

# After test completion, connect to the LXD container:
lxc exec torrust-tracker-vm -- /bin/bash
```

#### Configuration Tests Debugging

```bash
cargo run --bin e2e-config-tests -- --keep

# After test completion, find and connect to the Docker container:
docker ps
docker exec -it <container-id> /bin/bash
```

#### Full Local Tests Debugging

```bash
cargo run --bin e2e-tests-full -- --keep

# Connect to the LXD VM as above
lxc exec torrust-tracker-vm -- /bin/bash
```

## 🏗️ Architecture

The split E2E testing architecture ensures reliable CI while maintaining comprehensive coverage:

```text
┌──────────────────────────────────────────────────────────────────┐
│                        E2E Test Suites                          │
└─────┬────────────────┬────────────────┬─────────────────────────┘
      │                │                │
      │                │                │
┌─────▼──────┐   ┌─────▼────────┐   ┌───▼──────────────────┐
│ Provision  │   │Configuration │   │    Full Local        │
│   Tests    │   │    Tests     │   │      Tests           │
│            │   │              │   │                      │
│ LXD VMs    │   │   Docker     │   │ LXD VMs + Docker     │
│ (CI Safe)  │   │ Containers   │   │ (Local Only)         │
│            │   │ (CI Safe)    │   │                      │
└─────┬──────┘   └─────┬────────┘   └───┬──────────────────┘
      │                │                │
┌─────▼──────┐   ┌─────▼────────┐   ┌───▼──────────────────┐
│ OpenTofu/  │   │  Testcon-    │   │ OpenTofu + Ansible   │
│    LXD     │   │   tainers    │   │    (Full Stack)      │
│Infrastructure│  │   Docker     │   │                      │
│   Layer    │   │ Management   │   │                      │
└────────────┘   └──────────────┘   └──────────────────────┘
       │                │                        │
┌──────▼──────┐  ┌──────▼────────┐    ┌─────────▼─────────┐
│ VM Creation │  │Ansible Playbooks│    │  Complete Stack │
│ Cloud-init  │  │ Configuration   │    │    Validation   │
│ Validation  │  │   Validation    │    │                 │
└─────────────┘  └─────────────────┘    └───────────────────┘
```

### Test Suite Responsibilities

- **Provision Tests**: Infrastructure creation and basic VM setup validation
- **Configuration Tests**: Software installation and application deployment
- **Full Local Tests**: End-to-end integration validation for comprehensive testing

This architecture provides:

1. **Reliability**: Each test suite works independently in CI environments
2. **Speed**: Focused testing reduces execution time
3. **Coverage**: Combined suites provide complete deployment validation
4. **Debugging**: Clear separation makes issue identification easier

## 📝 Contributing to E2E Tests

When adding new features or making changes:

### Infrastructure Changes

For OpenTofu, LXD, or cloud-init modifications:

1. **Update provision tests** in `src/bin/e2e_provision_tests.rs`
2. **Add validation methods** for new infrastructure components
3. **Test locally**: `cargo run --bin e2e-provision-tests`
4. **Verify CI passes** on `.github/workflows/test-e2e-provision.yml`

### Configuration Changes

For Ansible playbooks or software installation modifications:

1. **Update configuration tests** in `src/bin/e2e_config_tests.rs`
2. **Add validation methods** for new software components
3. **Update Docker image** in `docker/provisioned-instance/` if needed
4. **Test locally**: `cargo run --bin e2e-config-tests`
5. **Verify CI passes** on `.github/workflows/test-e2e-config.yml`

### End-to-End Integration

For comprehensive changes affecting multiple components:

1. **Test with full local suite**: `cargo run --bin e2e-tests-full`
2. **Verify both provision and configuration suites pass independently**
3. **Update this documentation** to reflect changes
4. **Consider split approach**: Can the change be tested in isolated suites?

### Test Design Principles

- **Provision tests**: Focus on infrastructure readiness, minimal network dependencies
- **Configuration tests**: Focus on software functionality, reliable network access via containers
- **Full local tests**: Comprehensive validation for development workflows
- **Independence**: Each suite should be runnable independently without conflicts

The split E2E testing approach ensures reliable CI while maintaining comprehensive coverage of the entire deployment pipeline.
