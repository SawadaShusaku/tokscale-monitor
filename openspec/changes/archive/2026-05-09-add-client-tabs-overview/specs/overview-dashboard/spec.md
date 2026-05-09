## ADDED Requirements

### Requirement: Compact progress bar component
The system SHALL provide a compact progress bar component that displays a single model's usage rate with minimal vertical space.

#### Scenario: Progress bar rendering
- **WHEN** the component receives a model name, cost, limit, and usage rate
- **THEN** it SHALL render a horizontal bar with the filled portion proportional to usage rate
- **AND** display the model name and formatted "cost / limit" text beside or within the bar

#### Scenario: Warning state
- **WHEN** the usage rate exceeds 80%
- **THEN** the filled portion SHALL change to the warning color

### Requirement: Scrollable model list
The system SHALL render the Overview model list in a scrollable container to accommodate many models without excessive page height.

#### Scenario: Many models
- **WHEN** the number of models with usage exceeds the available vertical space
- **THEN** the container SHALL become scrollable
- **AND** the scrollbar SHALL follow the SmartHR Design System styling

### Requirement: Overview auto-refresh
The system SHALL refresh Overview data automatically by polling all supported clients every 60 seconds.

#### Scenario: Periodic overview refresh
- **WHEN** 60 seconds have elapsed
- **THEN** the system SHALL fetch data for all supported clients
- **AND** recompute and redisplay all progress bars
