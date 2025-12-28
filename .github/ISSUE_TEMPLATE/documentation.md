name: Documentation Update
description: Improve or clarify documentation
labels: ["documentation"]
assignees: []

body:
  - type: markdown
    attributes:
      value: |
        Help us improve Audio Ninja documentation!

  - type: dropdown
    id: doc_type
    attributes:
      label: Documentation Type
      options:
        - User Guide
        - API Reference
        - Architecture Guide
        - Example/Tutorial
        - Troubleshooting Guide
        - Configuration Guide
        - Build Instructions
        - Other
    validations:
      required: true

  - type: textarea
    id: current
    attributes:
      label: Current Documentation
      description: Link or describe the current documentation
      placeholder: "File: docs/API_USAGE.md or section..."
    validations:
      required: true

  - type: textarea
    id: improvement
    attributes:
      label: Proposed Improvement
      description: What should be added, clarified, or changed?
      placeholder: |
        - Add example for multi-zone configuration
        - Clarify the calibration workflow
        - Fix typo in daemon setup guide
    validations:
      required: true

  - type: checkboxes
    id: checklist
    attributes:
      label: Checklist
      options:
        - label: I have checked existing documentation
          required: true
        - label: This improves clarity or completeness
          required: true
