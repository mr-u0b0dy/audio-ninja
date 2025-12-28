# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of Audio Ninja seriously. If you believe you have found a security 
vulnerability, please report it to us responsibly.

### How to Report

**Please DO NOT file a public issue.** Instead:

1. Email security details to: [INSERT EMAIL ADDRESS]
2. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if available)

### What to Expect

- **Acknowledgment**: Within 48 hours of your report
- **Initial Assessment**: Within 7 days
- **Status Updates**: Every 7 days until resolution
- **Disclosure**: Coordinated disclosure after fix is available

### Security Best Practices

When using Audio Ninja:

1. **Network Security**
   - Use VLANs to isolate audio traffic
   - Enable encryption for sensitive environments
   - Validate speaker identity before pairing

2. **Access Control**
   - Restrict BLE pairing to authorized devices
   - Use secure WiFi passwords
   - Implement firewall rules for UDP/RTP ports

3. **Updates**
   - Keep dependencies up to date
   - Monitor security advisories
   - Test updates in non-production environments first

4. **Configuration**
   - Use strong authentication for control APIs
   - Limit network exposure of management interfaces
   - Validate user inputs

### Known Security Considerations

1. **Network Transport**
   - UDP/RTP traffic is unencrypted by default
   - Consider IPsec or VPN for sensitive audio
   - Packet injection could affect audio quality

2. **BLE Control**
   - Pairing uses standard BLE security
   - Physical proximity required for initial pairing
   - Monitor for unauthorized connection attempts

3. **Room Calibration**
   - Calibration sweeps can be loud
   - Verify speaker configuration before measurement
   - Store calibration data securely

### Disclosure Policy

- Security issues will be disclosed publicly after a fix is available
- Credit will be given to reporters (unless anonymity is requested)
- CVE IDs will be obtained for significant vulnerabilities

## Security Hall of Fame

We appreciate responsible disclosure. Contributors who report valid security issues will be acknowledged here (with permission).

---

Thank you for helping keep Audio Ninja and our users safe!
