## ADDED Requirements

### Requirement: Dynamic tab generation
The system SHALL generate the tab bar dynamically based on the user's settings, showing only enabled providers.

#### Scenario: Tab bar with all providers enabled
- **WHEN** all providers are enabled in settings
- **THEN** the tab bar SHALL display: Overview, OpenCode, Claude Code, Codex, Cursor, Gemini, Windsurf, Copilot, Aider

#### Scenario: Tab bar with some providers disabled
- **WHEN** some providers are disabled
- **THEN** only the enabled providers SHALL appear in the tab bar
- **AND** Overview SHALL always remain visible

#### Scenario: No provider tabs enabled
- **WHEN** all providers except Overview are disabled
- **THEN** only the Overview tab SHALL be displayed
- **AND** a subtle message MAY indicate that providers can be enabled in settings

### Requirement: Overview filtered by settings
The system SHALL filter the Overview dashboard to include only enabled providers.

#### Scenario: Overview with filtered providers
- **WHEN** a provider is disabled in settings
- **THEN** that provider's models SHALL NOT appear in the Overview dashboard
- **AND** the provider's cost SHALL NOT contribute to any aggregate metrics

### Requirement: Active tab fallback
The system SHALL handle the case where the currently active tab is disabled.

#### Scenario: Active tab becomes disabled
- **WHEN** the user disables the currently active provider in settings
- **THEN** the active tab SHALL switch to Overview automatically
- **AND** the disabled tab SHALL be removed from the tab bar
