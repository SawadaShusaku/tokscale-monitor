## ADDED Requirements

### Requirement: Tab-aware window metrics
The system SHALL compute and display window metrics for the currently selected client tab.

#### Scenario: Client-specific metrics
- **WHEN** the user switches to the "Claude Code" tab
- **THEN** the donut charts SHALL display Claude Code's rolling window metrics
- **AND** the PlanConfig SHALL be the one configured for Claude Code

#### Scenario: Auto-refresh per tab
- **WHEN** auto-refresh triggers while a client tab is active
- **THEN** the system SHALL fetch data only for that client
- **AND** update the dashboard for that client

## MODIFIED Requirements

### Requirement: Auto-refresh data
The system SHALL automatically refresh the displayed data by polling the backend every 60 seconds.

#### Scenario: Periodic refresh
- **WHEN** 60 seconds have elapsed since the last successful fetch
- **THEN** the frontend SHALL invoke `get_unified_messages` for the active client
- **AND** recompute and redisplay all metrics

#### Scenario: Manual refresh
- **WHEN** the user clicks a refresh button
- **THEN** the frontend SHALL immediately invoke `get_unified_messages` for the active client
- **AND** reset the 60-second timer
