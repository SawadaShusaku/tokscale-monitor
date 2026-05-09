# plan-limit-monitor Specification

## Purpose
Monitor and display rolling window cost metrics against configured plan limits, with per-client tab support.

## Requirements

### Requirement: Filter sessions by target models
The system SHALL include only sessions whose model name matches one of the configured Target Models in PlanConfig.

#### Scenario: Target model session
- **WHEN** a session uses a model listed in PlanConfig.targetModels (e.g., "GLM-5.1")
- **THEN** the session SHALL be included in rolling window calculations

#### Scenario: Non-target model session
- **WHEN** a session uses a model not listed in PlanConfig.targetModels
- **THEN** the session SHALL be excluded from rolling window calculations

### Requirement: Calculate rolling window costs
The system SHALL compute the total cost of included sessions within three rolling windows relative to the current local time: 5 hours, 7 days, and 30 days.

#### Scenario: 5-hour window
- **WHEN** the current time is T
- **THEN** the system SHALL sum costs of all included sessions where T - 5 hours <= session.date < T

#### Scenario: Weekly window
- **WHEN** the current time is T
- **THEN** the system SHALL sum costs of all included sessions where T - 7 days <= session.date < T

#### Scenario: Monthly window
- **WHEN** the current time is T
- **THEN** the system SHALL sum costs of all included sessions where T - 30 days <= session.date < T

### Requirement: Display remaining budget and usage rate
The system SHALL display the remaining budget and usage percentage for each window, and visually indicate when usage exceeds 80%.

#### Scenario: Normal usage
- **WHEN** a window's usage rate is 80% or less
- **THEN** the donut chart SHALL use the primary chart color

#### Scenario: Warning usage
- **WHEN** a window's usage rate exceeds 80%
- **THEN** the donut chart SHALL use the warning color

### Requirement: Predict next free slot for the 5-hour window
The system SHALL estimate when the 5-hour window will next free up by identifying the oldest included session and adding 5 hours.

#### Scenario: Window is partially or fully utilized
- **WHEN** there is at least one session within the 5-hour window
- **THEN** the system SHALL find the oldest such session
- **AND** compute oldestSession.date + 5 hours as the next free slot time
- **AND** display the remaining minutes until that time from the current time

#### Scenario: Window is empty
- **WHEN** there are no sessions within the 5-hour window
- **THEN** the system SHALL indicate that the window is already free

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
