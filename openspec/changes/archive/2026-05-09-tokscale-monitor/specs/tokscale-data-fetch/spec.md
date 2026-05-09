## ADDED Requirements

### Requirement: Execute tokscale command and return raw JSON
The system SHALL provide a Tauri command `get_tokscale_raw_data` that executes `tokscale --json --no-spinner` and returns the stdout as a string.

#### Scenario: Successful execution
- **WHEN** the frontend invokes `get_tokscale_raw_data`
- **THEN** the backend executes `tokscale --json --no-spinner`
- **AND** returns the stdout string to the frontend

#### Scenario: Command failure
- **WHEN** the backend fails to execute `tokscale` (e.g., command not found or non-zero exit code)
- **THEN** the backend returns an error message describing the failure

### Requirement: Command execution safety
The system SHALL execute the tokscale command without shell interpolation and with a reasonable timeout.

#### Scenario: Safe argument passing
- **WHEN** the backend runs the command
- **THEN** it passes `--json` and `--no-spinner` as discrete arguments
- **AND** it does not invoke a shell to parse the command line
