/**
# Closure
`它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值`

```
rust_course::advanced::functional_programming::learn_closure();
```
 */
pub fn learn_closure() {
    let x = 4;
    // sum，它拥有一个入参 y，同时捕获了作用域中的 x 的值，因此调用 sum(3) 意味着将 3（参数 y）跟 4（x）进行相加
    let sum = |y| x + y;

    assert_eq!(sum(3), 7);
}
