uniffi_macros::build_foreign_language_testcases!(
    [
        "../guid/src/guid.udl",
        "../../../examples/custom-types/src/custom-types.udl",
        "../uniffi-one/src/uniffi-one.udl",
        "src/ext-types-lib.udl",
    ],
    [
        "tests/bindings/test_imported_types.kts",
        "tests/bindings/test_imported_types.py",
    ]
);
