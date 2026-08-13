### Goldenfile Testing

### How to Run and Update

* Run normally: cargo test (Fails if actual_output differs from tests/testdata/user_report.txt).
* Update the baseline: UPDATE_GOLDENFILES=1 cargo test (Overwrites the file with the new output).
