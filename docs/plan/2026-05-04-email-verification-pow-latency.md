# Email Verification PoW Latency

- [x] Identify frontend worker hashing loop as the latency source.
- [x] Reduce default challenge difficulty to match the current worker implementation.
- [x] Improve worker progress reporting.
- [x] Run backend and frontend validation.
- [x] Remove worker dependency on `crypto.subtle`.
- [x] Restore PoW difficulty after worker hashing is reliable.
