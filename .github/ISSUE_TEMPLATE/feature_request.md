name: Feature Request
description: Suggest an idea for Audio Ninja
labels: ["enhancement"]
assignees: []

body:
  - type: markdown
    attributes:
      value: |
        Thanks for suggesting a feature! Please describe your idea clearly.

  - type: dropdown
    id: area
    attributes:
      label: Area
      description: What area does this feature affect?
      options:
        - Spatial Audio Rendering
        - Transport & Networking
        - Calibration & DSP
        - Device Control (BLE/WiFi)
        - API & Integration
        - Documentation
        - Testing & CI
        - Other
    validations:
      required: true

  - type: textarea
    id: problem
    attributes:
      label: Problem Statement
      description: What problem does this feature solve?
      placeholder: |
        Currently, there is no way to...
        This would help users who...
    validations:
      required: true

  - type: textarea
    id: solution
    attributes:
      label: Proposed Solution
      description: How would you like this feature to work?
      placeholder: |
        Add a new endpoint POST /api/v1/...
        Or implement a new command: audio-ninja ...
    validations:
      required: true

  - type: textarea
    id: alternatives
    attributes:
      label: Alternatives Considered
      description: Are there alternative approaches?
      placeholder: We could also...

  - type: textarea
    id: use_cases
    attributes:
      label: Use Cases
      description: What real-world use cases would benefit?
      placeholder: |
        - Multi-zone audio playback
        - Wireless synchronization
        - Automated calibration

  - type: checkboxes
    id: checklist
    attributes:
      label: Checklist
      options:
        - label: I have searched for existing feature requests
          required: true
        - label: I have checked the roadmap
          required: false
        - label: This aligns with Audio Ninja's scope
          required: true
