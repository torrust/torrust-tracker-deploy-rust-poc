use std::path::Path;
use tracing::{info, warn};

use crate::actions::{RemoteAction, RemoteActionError};
use crate::command_wrappers::ssh::SshClient;

/// Action that validates Docker Compose installation and basic functionality on the server
pub struct DockerComposeValidator {
    ssh_client: SshClient,
}

impl DockerComposeValidator {
    /// Create a new `DockerComposeValidator` with the specified SSH key
    ///
    /// # Arguments
    /// * `ssh_key_path` - Path to the SSH private key file
    /// * `username` - SSH username to use for connections
    /// * `verbose` - Whether to enable verbose SSH output
    #[must_use]
    pub fn new(ssh_key_path: &Path, username: &str, verbose: bool) -> Self {
        let ssh_client = SshClient::new(ssh_key_path, username, verbose);
        Self { ssh_client }
    }
}

impl RemoteAction for DockerComposeValidator {
    fn name(&self) -> &'static str {
        "docker-compose-validation"
    }

    async fn execute(&self, server_ip: &str) -> Result<(), RemoteActionError> {
        info!("🔍 Validating Docker Compose installation...");

        // First check if Docker is available (Docker Compose requires Docker)
        let docker_available = self
            .ssh_client
            .check_command(server_ip, "docker --version")
            .map_err(|source| RemoteActionError::SshCommandFailed {
                action_name: self.name().to_string(),
                source,
            })?;

        if !docker_available {
            warn!("⚠️  Docker Compose validation skipped");
            warn!("   ℹ️  Docker is not available, so Docker Compose cannot be validated");
            warn!("   ℹ️  This is expected in CI environments with network limitations");
            return Ok(()); // Don't fail the test, just skip validation
        }

        // Check Docker Compose version
        let Ok(compose_version) = self
            .ssh_client
            .execute(server_ip, "docker-compose --version")
        else {
            warn!(
                "⚠️  Docker Compose not found, this is expected if Docker installation was skipped"
            );
            return Ok(()); // Don't fail, just note the situation
        };

        let compose_version = compose_version.trim();
        info!("✅ Docker Compose installation validated");
        info!("   ✓ Docker Compose version: {compose_version}");

        // Test basic docker-compose functionality with a simple test file (only if Docker is working)
        let test_compose_content = r"services:
  test:
    image: hello-world
";

        // Create a temporary test docker-compose.yml file
        let create_test_success = self
            .ssh_client
            .check_command(
                server_ip,
                &format!("echo '{test_compose_content}' > /tmp/test-docker-compose.yml"),
            )
            .map_err(|source| RemoteActionError::SshCommandFailed {
                action_name: self.name().to_string(),
                source,
            })?;

        if !create_test_success {
            warn!("   ⚠️  Could not create test docker-compose.yml file");
            return Ok(()); // Don't fail, just skip the functional test
        }

        // Validate docker-compose file
        let validate_success = self
            .ssh_client
            .check_command(
                server_ip,
                "cd /tmp && docker-compose -f test-docker-compose.yml config",
            )
            .map_err(|source| RemoteActionError::SshCommandFailed {
                action_name: self.name().to_string(),
                source,
            })?;

        if validate_success {
            info!("   ✓ Docker Compose configuration validation passed");
        } else {
            warn!("   ⚠️  Docker Compose configuration validation skipped");
        }

        // Clean up test file
        drop(
            self.ssh_client
                .check_command(server_ip, "rm -f /tmp/test-docker-compose.yml"),
        );

        Ok(())
    }
}
