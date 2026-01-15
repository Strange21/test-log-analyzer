# Zero-Copy Log Analyzer

## Summary

You have been tasked with writing a **high-performance log analyzer** in Rust.

The input is a log file that may be **several gigabytes** in size. Each line in
the file follows a structured format. The goal is to analyze the file
efficiently without loading it entirely into memory and without unnecessary
allocations.

The solution should emphasize **streaming processing**, **memory efficiency**,
and **robust error handling**.


## Log Format

Each line in the log file follows this format:

```
<timestamp>|<level>|<service>|<message>
```

Example:
```
2025-01-01T12:00:00Z|ERROR|auth|invalid token
```
Where:
* `timestamp` is an ISO-8601 timestamp
* `level` is one of `INFO`, `WARN`, or `ERROR`
* `service` is an alphanumeric service name
* `message` is free-form text

## Requirements

### Core Functionality

* Read the log file in a **streaming** manner
* Do not load the entire file into memory
* Parse each log line and extract the log level
* Count occurrences of each log level:
  * `INFO`
  * `WARN`
  * `ERROR`
* Print a summary report after processing the entire file

Example output:
```
INFO: 120394
WARN: 23941
ERROR: 4821
```

## Performance Constraints

* Must use buffered IO
* Must not read the full file into memory
* Avoid allocating new `String`s per log line where possible
* Prefer **zero-copy parsing** using string slices
* The solution must scale with increasing file size

## Error Handling

* Malformed lines must not cause the program to crash
* Invalid lines may be skipped, logged, or counted separately
* No panics on invalid input

## Additional Requirements

* Your source should contain unit tests where appropriate
* All code must be formatted using the standard formatting tool
* Code must compile without clippy errors
* The solution must use safe Rust only

## Design & Reasoning (Required)

Along with the code, include a document (for example `DESIGN.md`) explaining:

* How the file is read and processed
* How parsing is performed without unnecessary allocations
* Where allocations are unavoidable
* Performance trade-offs made
* How the solution behaves with very large files

Submissions without a design explanation will not be reviewed.

## Submission

Please fork this repository to your own GitHub account and submit a pull request
to your own repository.

Your pull request should include:

* A clear description of your approach
* Any assumptions or trade-offs made
* Instructions on how to run the program and tests

A link to the pull request can be submitted once it is ready for review.

## Bonus

* Parallel processing of log file chunks
* Support for configurable log formats
* Separate reporting for malformed lines
* Benchmark results or performance notes
