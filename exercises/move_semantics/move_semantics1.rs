// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

/**
 * 소유권 규칙
 * 1. Rust가 다루는 각각의 값은 소유자(owner) 라고 부르는 변수를 가지고 있다.
 * 2. 특정 시점에 값의 소유자는 단 하나 뿐이다.
 * 3. 소유자가 범위를 벗어나면 그 값은 제거된다.
 * 
 */

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
