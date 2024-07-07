// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut x = 100;
    {
      let y = &mut x;
      *y += 100;
    }
    //你需要确保在任何给定时间，只有一个可变引用存在
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
