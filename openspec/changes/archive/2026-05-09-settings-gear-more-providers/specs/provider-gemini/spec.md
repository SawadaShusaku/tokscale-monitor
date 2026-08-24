## ADDED Requirements

### Requirement: Gemini CLI/App provider
The system SHALL provide a `gemini` client that reads usage data from Gemini CLI or desktop app.

#### Scenario: Gemini data read
- **WHEN** the frontend invokes `get_unified_messages` with `"gemini"`
- **THEN** the backend SHALL read Gemini's local data files
- **AND** return assistant interactions in UnifiedMessage format

## MODIFIED Requirements

### Requirement: Unified message endpoint expansion
The system SHALL support `"gemini"` as a valid client name in `get_unified_messages`.
