mod infra;

// Your tests go here!
success_tests! {
    add1: "73",
}

failure_tests! {
    unbound_id: "Unbound variable identifier x",
    duplicate_binding: "Duplicate binding",
}
