#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mikan_os_rust::test_runner_handler)]
#![reexport_test_harness_main = "test_main"]

use mikan_os_rust::test_template;

test_template!();