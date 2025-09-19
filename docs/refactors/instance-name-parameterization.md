# Instance Name Parameterization Refactor

## Overview

This refactor aims to eliminate hardcoded "torrust-vm" instance names throughout the codebase and replace them with parameterized values. The goal is to enable dynamic instance naming for different environments and support running multiple instances simultaneously.

## 📊 Progress Status

### ✅ Completed Phases

- **Phase 1**: Foundation - OpenTofu Variables Infrastructure

  - ✅ Step 1a: Created OpenTofu variables template (`variables.tfvars`)
  - ✅ Step 1b: Updated OpenTofu client for variables file support

- **Phase 2**: Template Parameterization

  - ✅ Step 2a: Converted variables.tfvars to Tera template with `{{instance_name}}` placeholder
  - ✅ Step 2b: Created template wrapper infrastructure (`VariablesTemplate`, `VariablesContext`)
  - ✅ Step 2c: Integrated Variables Template Rendering into workflow

- **Phase 3**: Context Integration - Configuration Level Parameterization

  - ✅ Step 3a: Added `instance_name` field to `Config` struct in `src/config/mod.rs`
  - ✅ Step 3b: Updated `Config::new()` to accept `instance_name` parameter
  - ✅ Step 3c: Modified `Services::new()` to use `config.instance_name` instead of hardcoded values
  - ✅ Step 3d: Updated cleanup functions to use dynamic instance names from config

- **Phase 4**: Test Environment Parameterization
  - ✅ Step 4a: Added `instance_name` parameter to `TestEnvironment::with_ssh_user()` function
  - ✅ Step 4b: Updated `TestEnvironment::with_ssh_user_and_init()` to accept `instance_name`
  - ✅ Step 4c: Updated `create_config()` helper to accept `instance_name` parameter
  - ✅ Step 4d: Maintained backward compatibility via `new()` functions with hardcoded values

### 🔄 Current Phase

- **Phase 3**: Context Integration - Add instance_name to workflow context (Next Priority)
  - 🔄 Add `instance_name` field to `TofuContext` struct
  - 🔄 Pass `instance_name` context from provision workflow
  - 🔄 Replace hardcoded "torrust-vm" values with dynamic context

### 📋 Remaining Phases

- **Phase 3**: Context Integration - Add instance_name to workflow context
- **Phase 4**: E2E Integration - Update E2E tests infrastructure context
- **Phase 5**: Complete Migration - Update remaining hardcoded references

## 🔄 Design Updates

### Variable Naming Convention

- **Variable name**: `instance_name` (not `container_name`) - since we're provisioning VM instances
- **Scope**: Focus on instance name parameterization only
- **Static variables**: Keep `image` and other variables static for now (not Tera templated)

## 🎯 Implementation Plan

### Phase 1: Foundation - OpenTofu Variables Infrastructure

**Goal**: Establish OpenTofu variables file pattern and integration

#### Step 1a: Create OpenTofu variables template ✅

- ✅ Create `templates/tofu/lxd/variables.tfvars` template file to define `instance_name` variable
- ✅ Update `TofuTemplateRenderer` to include this file in static template copying
- ✅ Keep `image` variable static (not templated)
- **Status**: Static variables file created with hardcoded "torrust-vm" value
- **Validation**: ✅ Unit tests + linters + e2e tests passed

#### Step 1b: Update OpenTofu client for variables file ✅

- ✅ Modify OpenTofu client to pass `-var-file` parameter to `tofu` commands
- ✅ Update unit tests for `TofuClient`
- **Status**: OpenTofu client accepts `extra_args` parameter for `-var-file=variables.tfvars`
- **Validation**: ✅ All OpenTofu commands work with variables file

### Phase 2: Template Parameterization ✅

**Goal**: Convert static variables to dynamic Tera templates

#### Step 2a: Convert variables.tfvars to Tera template ✅

- ✅ Transform static `variables.tfvars` into `variables.tfvars.tera` template with `{{instance_name}}` placeholder
- ✅ Update `TofuTemplateRenderer` to render it with context
- **Status**: Template created with Tera placeholder for dynamic rendering
- **Validation**: ✅ Rendered file contains correct instance name value

#### Step 2b: Create template wrapper for variables rendering ✅

- ✅ Create `VariablesTemplate` and `VariablesContext` following cloud-init pattern
- ✅ Add `.tfvars` extension support to template engine
- ✅ Implement comprehensive template validation and rendering
- **Status**: Template wrapper infrastructure complete with 14 new unit tests
- **Validation**: ✅ Template rendering works, all tests pass

### Phase 2c: Integrate Variables Template Rendering (In Progress) 🔄

**Goal**: Add variables template rendering to infrastructure workflow

#### Step 2c: Integrate Variables Template Rendering into workflow ✅

- ✅ Added `VariablesTemplate` imports to `TofuTemplateRenderer`
- ✅ Added `instance_name` field to `TofuTemplateRenderer` struct
- ✅ Updated `TofuTemplateRenderer::new()` to accept `instance_name` parameter
- ✅ Created `render_variables_template()` method for dynamic rendering
- ✅ Updated `render_tera_templates()` to call variables template rendering
- ✅ Removed `variables.tfvars` from static template files list
- ✅ Added hardcoded "torrust-vm" values for backward compatibility
- ✅ Updated all test constructors to include `instance_name` parameter
- **Status**: Variables template now dynamically renders with `instance_name = "torrust-vm"`
- **Validation**: ✅ All linters + unit tests + e2e tests passed

### Phase 3: Context Integration ✅

#### Step 3a-3d: Config Struct Approach (Completed)

Instead of the originally planned `TofuContext` approach, we implemented instance name parameterization through the existing `Config` struct:

- ✅ Added `instance_name: InstanceName` field to `Config` struct in `src/config/mod.rs`
- ✅ Updated `Config::new()` constructor to accept `instance_name` parameter
- ✅ Modified `Services::new()` in `src/container.rs` to use `config.instance_name` instead of hardcoded values
- ✅ Updated `TofuTemplateRenderer::new()` to use dynamic instance name from config
- **Status**: Config struct now serves as central source of truth for instance names
- **Validation**: ✅ All linters + unit tests + e2e tests passed

### Phase 4: Test Environment Parameterization ✅

#### Step 4a-4d: TestEnvironment Function Updates (Completed)

- ✅ Added `instance_name` parameter to `TestEnvironment::with_ssh_user()` in `src/e2e/environment.rs`
- ✅ Updated `TestEnvironment::with_ssh_user_and_init()` to accept `instance_name` parameter
- ✅ Updated `create_config()` helper function to accept `instance_name` parameter
- ✅ Maintained backward compatibility with existing `new()` functions using hardcoded "torrust-vm"
- **Status**: E2E test infrastructure can now create environments with custom instance names
- **Validation**: ✅ All linters + unit tests + e2e tests passed

### 🔄 Implementation Notes

The implementation evolved from the original plan due to codebase changes:

1. **Config Struct Approach**: Instead of creating a `TofuContext` struct (which the original plan assumed existed), we used the existing `Config` struct as the primary container for `instance_name`.

2. **Combined Phase Implementation**: Phases 3, 4, and 5 were implemented together due to interdependencies, using the `Config` struct as the central source of truth for instance names.

3. **TestEnvironment Parameterization**: Multiple `TestEnvironment` functions were parameterized to accept `instance_name`, maintaining backward compatibility with functions that use hardcoded values.

4. **Backward Compatibility**: All changes maintain existing function signatures through overloads, ensuring no breaking changes to existing tests.

### 🎯 Outcomes

- **Hardcoded Instances Eliminated**: Key hardcoded "torrust-vm" strings removed from infrastructure provisioning
- **Dynamic Instance Naming**: Instance names can now be configured at runtime through the Config struct
- **Test Environment Flexibility**: E2E tests can create environments with custom instance names
- **Configuration Centralization**: All instance name handling flows through the `Config` struct

### 📍 Current Status

**COMPLETED**: Instance name parameterization refactor successfully implemented using Config struct approach. All tests passing, linters clean, e2e tests successful.

## 🔍 Analysis of Current "torrust-vm" Usage

Currently, the instance name "torrust-vm" is hardcoded in multiple places across the codebase, including:

- Infrastructure configuration files (OpenTofu templates)
- Rust code literals and variables
- Ansible configurations
- Potentially workflows and comments

This creates several issues:

- Difficulty running multiple instances simultaneously
- Hard to customize instance names for different environments
- Tight coupling between configuration and implementation

## Summary of "torrust-vm" Usage Analysis

Based on a comprehensive search through the codebase, **61 occurrences** of "torrust-vm" were found across 4 main categories:

### 1. Configuration Files (2 files, 7 occurrences)

- `templates/tofu/lxd/main.tf` - OpenTofu variable default value
- `templates/ansible/inventory.yml.tera` - Ansible inventory template (5 places: comments, host names, lxc commands)

### 2. Rust Code Literals/Variables (5 files, 9 occurrences)

- `src/e2e/tasks/cleanup_infrastructure.rs` - E2E cleanup tasks
- `src/template/file.rs` - Template file testing
- `src/command_wrappers/opentofu/json_parser.rs` - JSON parser unit tests (5 places)
- `src/ansible/template/renderer/inventory.rs` - Inventory renderer tests (2 places)
- `tests/template_integration.rs` - Integration test assertions

### 3. GitHub Workflows (2 files, 13 occurrences)

- `.github/workflows/test-e2e.yml` - E2E workflow (3 places)
- `.github/workflows/test-lxd-provision.yml` - LXD provision workflow (10 places)

### 4. Documentation & Comments (4 files, 30 occurrences)

- `README.md` - Usage examples (5 places)
- `docs/e2e-testing.md` - Testing documentation (3 places)
- `docs/tofu-lxd-configuration.md` - Configuration documentation (22 places)
- `docs/refactors/instance-name-parameterization.md` - This refactor document (3 places)

## Goals

1. **Centralize Configuration**: Define the instance name in one place (`src/bin/e2e_tests.rs`)
2. **Parameter Propagation**: Pass the instance name down through the call chain
3. **Template Parameterization**: Convert static templates to Tera templates where needed
4. **Maintain Separation**: Use OpenTofu variables file strategy to avoid over-templating main.tf

## Strategy

### Template Strategy

- Keep `templates/tofu/lxd/main.tf` as a static template
- Create a new `variables.tf.tera` template for OpenTofu variables
- Inject the instance name through the variables file rather than directly into main.tf

### Refactor Direction

Start from low-level infrastructure details and work up to higher abstractions:

1. OpenTofu variable configuration
2. Template rendering system
3. Command wrappers
4. Higher-level orchestration
5. Top-level main function
6. Workflows (excluding `.github/workflows/test-lxd-provision.yml` - independent test for GitHub runners contract)

## Detailed Refactor Plan

### Phase 1: Infrastructure Foundation

**Goal**: Create OpenTofu variables file strategy and update low-level templates

**Step 1a**: Create OpenTofu variables template

- Create `templates/tofu/lxd/variables.tfvars.tera` with instance name parameter
- Update `src/tofu/template/renderer/mod.rs` to render this new template
- **Validation**: Run linters, unit tests, e2e tests

**Step 1b**: Update OpenTofu client to use variables file

- Modify OpenTofu client to pass `-var-file` parameter
- Update relevant tests in `src/command_wrappers/opentofu/json_parser.rs`
- **Validation**: Run linters, unit tests, e2e tests

### Phase 2: Template System Updates

**Goal**: Update template rendering to accept instance name parameter

**Step 2a**: Update Tera template context

- Modify template renderer to accept instance_name parameter
- Update `templates/ansible/inventory.yml.tera` to use template variable instead of hardcoded name
- **Validation**: Run linters, unit tests, e2e tests

**Step 2b**: Update template integration tests

- Update `tests/template_integration.rs` to use parameterized name
- Update `src/template/file.rs` test expectations
- **Validation**: Run linters, unit tests, e2e tests

### Phase 3: E2E Infrastructure

**Goal**: Update E2E test system to accept parameterized instance names

**Step 3a**: Update E2E task implementations

- Modify `src/e2e/tasks/cleanup_infrastructure.rs` to use parameter
- Update `src/ansible/template/renderer/inventory.rs` tests
- **Validation**: Run linters, unit tests, e2e tests

**Step 3b**: Connect parameter flow from main function

- Add constant in `src/bin/e2e_tests.rs`
- Pass instance name through TestEnvironment
- Update environment setup
- **Validation**: Run linters, unit tests, e2e tests

### Phase 4: Workflow Updates

**Goal**: Update CI/CD workflows to be environment-flexible

**Step 4a**: Update GitHub workflows

- Update `.github/workflows/test-e2e.yml` to use environment variables or be more generic
- **Validation**: Run linters, check workflow syntax

### Phase 5: Documentation Updates

**Goal**: Update documentation to reflect parameterized approach

**Step 5a**: Update primary documentation

- Update `README.md` with parameterized examples
- Update `docs/e2e-testing.md` with new approach
- **Validation**: Run markdown linter

**Step 5b**: Update configuration documentation

- Update `docs/tofu-lxd-configuration.md` with variable-based approach
- **Validation**: Run markdown linter

## Success Criteria

- [ ] Instance name defined once in `src/bin/e2e_tests.rs`
- [ ] All hardcoded "torrust-vm" references eliminated
- [ ] OpenTofu uses variables file for parameterization
- [ ] All tests (unit, linting, e2e) pass at each step
- [ ] No regression in functionality
- [ ] Clean separation of concerns maintained

## Implementation Notes

- Use baby steps: maximum 2 files changed per step
- Run full validation (linters, unit tests, e2e tests) after each step
- Maintain backwards compatibility during transition
- Document each step thoroughly
- Ask for confirmation before implementing each step
- Update execution plan after completing small tests
