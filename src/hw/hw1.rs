/*
    Домашнее задание 1
*/

// принимает 32-х битное целое беззнаковое число и возвращает 32-х битное целое беззнаковое число, равное удвоенному входному
pub fn double_int32(a: u32) -> u32 {
    2 * a
}

// принимает 32-х битное целое беззнаковое число и возвращает 64-х битное целое беззнаковое число, равное удвоенному входному
pub fn double_int64(a : u32) -> u64 {
    (2 * a).into()
}

// принимает 32-х битное число с плавающей точкой и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному
pub fn double_float32(a : f32) -> f32{
    2 as f32 * a
}

// принимает 32-х битное число с плавающей точкой и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному
pub fn double_float64(a : f32) -> f64 {
    (2 as f32 * a).into()
}

// принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное число с плавающей точкой, равное сумме входных
pub fn int_plus_float_to_float(a : u32, b : f32) -> f64 {
    (a as f32 + b).into()
}

// принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное целое беззнаковое число, равное сумме входных
pub fn int_plus_float_to_int(a :u32, b :f32) -> u64{
    (a as f32 + b) as u64
}

// принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже
pub fn tuple_sum(a :(i32, i32)) -> i32{
    a.0 + a.1
}

// принимает массив из трёх целых чисел. Возвращает целое число, равное сумме чисел во входном массиве
pub fn array_sum(numbers :[i32;3]) -> i32{
    numbers[0] + numbers[1] + numbers[2]
}

// сумма элементов массива (цикл)
pub fn array_sum_for(numbers :[i32;3]) -> i32{
    let mut sum = 0;
    for number in numbers {
        sum += number
    }
    sum
}

// сумма элементов массив (итерратор)
pub fn array_sum_iter(numbers :[i32;3]) -> i32{
    numbers.iter().sum()
}