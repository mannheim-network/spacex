// Copyright (C) 2019-2021 Mannheim Network Technologies Ltd.
// This file is part of Mannheim.

use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .import_memory()
        .export_heap_base()
        .build()
}

