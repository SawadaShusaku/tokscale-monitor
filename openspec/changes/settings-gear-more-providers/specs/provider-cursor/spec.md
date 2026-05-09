## ADDED Requirements

### Requirement: Cursor IDE provider
The system SHALL provide a `cursor` client that reads usage data from Cursor IDE's local files.

#### Scenario: Cursor data read
- **WHEN** the frontend invokes `get_unified_messages` with `"cursor"`
- **THEN** the backend SHALL read Cursor's local data files
- **AND** return assistant interactions in UnifiedMessage format

#### Scenario: Cursor data source
- **GIVEN** Cursor IDE stores session data locally
- **WHEN** the parser runs
- **THEN** it SHALL scan known Cursor data directories
- **AND** extract model usage and timestamps

## MODIFIED Requirements

### Requirement: Unified message endpoint expansion
The system SHALL support `"cursor"` as a valid client name in `get_unified_messages`.
