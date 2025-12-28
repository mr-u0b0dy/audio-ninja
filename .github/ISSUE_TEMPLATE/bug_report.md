name: Bug Report
description: Report a bug or unexpected behavior
labels: ["bug"]
assignees: []

body:
  - type: markdown
    attributes:
      value: |
        Thanks for reporting a bug! Please fill out the form below to help us understand and fix the issue.

  - type: dropdown
    id: component
    attributes:
      label: Component
      description: Which component does this affect?
      options:
        - Core Library (audio-ninja)
        - Daemon (audio-ninja-daemon)
        - CLI (audio-ninja-cli)
        - GUI (audio-ninja-gui)
        - Documentation
        - CI/CD
        - Other
    validations:
      required: true

  - type: textarea
    id: description
    attributes:
      label: Description
      description: A clear and concise description of the bug
      placeholder: What is the bug about?
    validations:
      required: true

  - type: textarea
    id: steps_to_reproduce
    attributes:
      label: Steps to Reproduce
      description: How can we reproduce this bug?
      placeholder: |
        1. Start daemon with...
        2. Run command...
        3. Observe error...
    validations:
      required: true

  - type: textarea
    id: expected_behavior
    attributes:
      label: Expected Behavior
      description: What should happen instead?
      placeholder: The command should succeed and return...
    validations:
      required: true

  - type: textarea
    id: actual_behavior
    attributes:
      label: Actual Behavior
      description: What actually happens?
      placeholder: Error message, output, or incorrect behavior...
    validations:
      required: true

  - type: textarea
    id: environment
    attributes:
      label: Environment
      description: Environmental details
      placeholder: |
        - OS: Ubuntu 24.04 / macOS 14.0 / Windows 11
        - Rust version: 1.75
        - Audio Ninja version: 0.1.0
        - Audio device: USB, ALSA, PulseAudio, etc.
    validations:
      required: true

  - type: textarea
    id: logs
    attributes:
      label: Relevant Logs or Error Messages
      description: Include any error messages or relevant logs
      render: bash
      placeholder: |
        error: failed to decode audio
        thread 'audio-renderer' panicked at ...

  - type: textarea
    id: additional_context
    attributes:
      label: Additional Context
      description: Any other relevant information
      placeholder: Configuration files, speaker setup, network details, etc.

  - type: checkboxes
    id: checklist
    attributes:
      label: Checklist
      options:
        - label: I have searched for existing issues
          required: true
        - label: I have checked the documentation
          required: true
        - label: I have tried to reproduce with latest version
          required: false
