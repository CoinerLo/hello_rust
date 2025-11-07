(module
    ;; импорт из файла
    (import "env" "logStart" (func $log_start))
    (import "env" "logOperation" (func $log_operation (param i32)))
    (import "env" "logResult" (func $log_result (param i32)))
    ;; импорт 
    (func $get_sum (param $a i32) (param $b i32) (result i32)
        (local $result i32)
        (call $log_start)
        local.get $a
        (call $log_operation (local.get $a))
        local.get $b
        (call $log_operation (local.get $b))
        i32.add
        local.set $result
        (call $log_result (local.get $result))
        (local.get $result)
    )
    (export "getSum" (func $get_sum))
)
