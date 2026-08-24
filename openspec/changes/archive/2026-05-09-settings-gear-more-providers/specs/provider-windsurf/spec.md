## ADDED Requirements

### Requirement: Windsurf provider
The system SHALL provide a `windsurf` client that reads usage data from Windsurf IDE.

#### Scenario: Windsurf data read
- **WHEN** the frontend invokes `get_unified_messages` with `"windsurf"`
- **THEN** the backend SHALL read Windsurf's local data files
- **AND** return assistant interactions in UnifiedMessage format

## MODIFIED Requirements

### Requirement: Unified message endpoint expansion
The system SHALL support `"windsurf"` as a valid client name in `get_unified_messages`.
