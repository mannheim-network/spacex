// Copyright (C) 2019-2021 Mannheim Network Technologies Ltd.
// This file is part of Mannheim.

pub use sc_executor::NativeExecutor;
use sc_executor::native_executor_instance;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
native_executor_instance!(
    pub Executor,
    spacex_runtime::api::dispatch,
    spacex_runtime::native_version,
    frame_benchmarking::benchmarking::HostFunctions,
);