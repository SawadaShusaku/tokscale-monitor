## ADDED Requirements

### Requirement: Settings gear icon
The system SHALL display a settings gear icon in the header that opens a configuration panel.

#### Scenario: Gear icon display
- **WHEN** the application loads
- **THEN** a gear icon SHALL be visible in the top-right of the header
- **AND** clicking it SHALL open the settings panel

### Requirement: Settings panel with provider toggles
The system SHALL provide a settings panel that allows users to toggle the visibility of each provider tab.

#### Scenario: Opening settings panel
- **WHEN** the user clicks the gear icon
- **THEN** a slide-in drawer SHALL appear from the right side
- **AND** it SHALL display a list of all available providers with toggle switches

#### Scenario: Toggling provider visibility
- **WHEN** the user toggles a provider OFF
- **THEN** that provider's tab SHALL disappear from the tab bar
- **AND** its data SHALL be excluded from the Overview dashboard

#### Scenario: Toggling provider visibility ON
- **WHEN** the user toggles a provider ON
- **THEN** that provider's tab SHALL appear in the tab bar
- **AND** its data SHALL be included in the Overview dashboard

### Requirement: Settings persistence
The system SHALL persist the user's provider visibility settings across app restarts.

#### Scenario: App restart
- **WHEN** the application is closed and reopened
- **THEN** the previously selected provider visibility settings SHALL be restored
- **AND** the tab bar SHALL reflect the restored settings

### Requirement: Default settings
The system SHALL default all providers to visible (ON) for new users.

#### Scenario: First launch
- **WHEN** a user opens the app for the first time
- **THEN** all provider tabs SHALL be visible by default
- **AND** the settings panel SHALL show all toggles in the ON position
