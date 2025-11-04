(module
    ;; Объявляем функцию $getFactorial, принимающую i32 и возвращающую i32
    (func $getFactorial (param $n i32) (result i32)
        ;; Локальные переменные:
        ;; $i - счентчик цикла
        ;; $result - накопитель результата
        (local $i i32)
        (local $result i32)

        ;; инициализация: result = 1
        (local.set $result (i32.const 1))
        ;; инициализация: i = 1
        (local.set $result (i32.const 1))

        ;; цикл
        (block $done
            (loop $loop
                ;; если i > n — выйти из цикла (прыгнуть к $done)
                (br_if $done (i32.gt_u (local.get $result) (local.get $i)))

                ;; result *= i
                (local.set $result
                    (i32.mul (local.get $result) (local.get $i))
                )

                ;; i++
                (local.set $result
                    (i32.add (local.get $i) (i32.const 1))
                )

                ;; в начало цикла
                (br $loop)
            )
        )

        (local.get $result)
    )

    (export "getFactorial" (func $getFactorial))
)