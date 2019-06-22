# Integration Tests

This is a sort of kitchen sink approach to running integration tests. Just run
`cargo test` and it will make several HTTP requests to the running Osgood
instance. Some endpoints intentionally cause errors, this is to make sure the
process doesn't crash.
