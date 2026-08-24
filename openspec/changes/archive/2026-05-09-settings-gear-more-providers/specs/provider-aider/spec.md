## ADDED Requirements

### Requirement: Aider provider
The system SHALL provide an `aider` client that reads usage data from Aider.

#### Scenario: Aider data read
- **WHEN** the frontend invokes `get_unified_messages` with `"aider"`
- **THEN** the backend SHALL read Aider's local data files
- **AND** return assistant interactions in UnifiedMessage format

## MODIFIED Requirements

### Requirement: Unified message endpoint expansion
The system SHALL support `"aider"` as a valid client name in `get_unified_messages`.
