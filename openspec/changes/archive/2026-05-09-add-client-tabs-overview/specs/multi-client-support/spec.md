## ADDED Requirements

### Requirement: Provide unified message endpoint per client
The system SHALL provide a Tauri command `get_unified_messages(client: String)` that reads raw session data for the specified client and returns messages in UnifiedMessage format.

#### Scenario: OpenCode client
- **WHEN** the frontend invokes `get_unified_messages` with `"opencode"`
- **THEN** the backend reads `~/.local/share/opencode/opencode.db`
- **AND** returns assistant messages with `model_id`, `cost`, and `timestamp`

#### Scenario: Claude Code client
- **WHEN** the frontend invokes `get_unified_messages` with `"claude"`
- **THEN** the backend reads `~/.claude/projects/` JSON files
- **AND** returns assistant messages in UnifiedMessage format

#### Scenario: Codex CLI client
- **WHEN** the frontend invokes `get_unified_messages` with `"codex"`
- **THEN** the backend reads `~/.codex/sessions/` JSON files
- **AND** returns assistant messages in UnifiedMessage format

#### Scenario: Unsupported client
- **WHEN** the frontend invokes `get_unified_messages` with an unsupported client name
- **THEN** the backend returns an empty messages array

### Requirement: Client selection tab UI
The system SHALL display a tab bar for switching between clients, with an Overview tab at the leftmost position.

#### Scenario: Tab bar display
- **WHEN** the application loads
- **THEN** the tab bar SHALL display tabs in order: Overview, OpenCode, Claude Code, Codex CLI
- **AND** the first tab (Overview) SHALL be active by default

#### Scenario: Tab switching
- **WHEN** the user clicks a client tab
- **THEN** the dashboard SHALL switch to show that client's window metrics
- **AND** auto-refresh SHALL reset to the newly selected client

### Requirement: Overview tab with progress bars
The system SHALL display an Overview tab that shows usage progress bars for all models across all supported clients.

#### Scenario: Overview display
- **WHEN** the Overview tab is active
- **THEN** the system SHALL display a progress bar for each model that has usage > 0
- **AND** each bar SHALL show the model name, client label, usage percentage, and cost/limit

#### Scenario: Progress bar color coding
- **WHEN** a model's usage rate is 80% or less
- **THEN** the progress bar SHALL use the primary color

#### Scenario: Progress bar warning color
- **WHEN** a model's usage rate exceeds 80%
- **THEN** the progress bar SHALL use the warning color

#### Scenario: Empty overview
- **WHEN** no messages exist for any client
- **THEN** the Overview SHALL display "No usage data available"
