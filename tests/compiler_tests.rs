mod common;
use common::*;

include!(concat!(env!("OUT_DIR"), "/parsing_tests.rs"));
include!(concat!(env!("OUT_DIR"), "/semantic_tests.rs"));
include!(concat!(env!("OUT_DIR"), "/integration_tests.rs"));