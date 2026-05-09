## ADDED Requirements

### Requirement: GitHub Copilot Chat provider
The system SHALL provide a `copilot` client that reads usage data from GitHub Copilot Chat.

#### Scenario: Copilot data read
- **WHEN** the frontend invokes `get_unified_messages` with `"copilot"`
- **THEN** the backend SHALL read Copilot's local data files
- **AND** return assistant interactions in UnifiedMessage format

## MODIFIED Requirements

### Requirement: Unified message endpoint expansion
The system SHALL support `"copilot"` as a valid client name in `get_unified_messages`.
