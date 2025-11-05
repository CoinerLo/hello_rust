(module
    (import "logger" "loggerJS" (func $logger (param i32)))
    (func $getSum (param $a i32) (param $b i32) (result i32)
        (local $result i32)
        local.get $a
        (call $logger (local.get $a))
        local.get $b
        (call $logger (local.get $b))
        (local.set $result
            (i32.add (local.get $a) (local.get $b))
        )
        (call $logger (local.get $result))
        (local.get $result)
    )
    (export "getSum" (func $getSum))
)
