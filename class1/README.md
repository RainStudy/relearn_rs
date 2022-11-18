# 1.课程介绍与基本语法

## 新建项目

> cargo new \<project-name\> && cd ./\<project-name\>

## 编译并运行项目

> cargo run

程序入口为 src/main.rs 中的main函数

## 编译项目

> cargo build

## 以前没有注意到的知识点

> loop表达式可以通过break返回值 (while和for是不可以的)

~~~rust
// 5
let value = loop {
    break 5;
};
~~~

> 基本类型

- 布尔值 `bool`
- 字符 `char`
- 数值 分为整数和浮点数，有不同的大小和符号属性
    - i8, i16, i32, i64, isize 整数
    - u8, u16, u32, u64, usize
    - f32, f64 浮点数 (对应float，double)
    - isize, usize是指针大小的整数，他们的大小与机器架构有关 (32位，64位)
    - 字面值 (literals) 写为 10i8, 10u16, 10.0f32, 10usize 等
    - 字面值如果不指定类型，则默认整数为 i32, 浮点数为 f64。
- 数组 (arrays), 切片 (slices), str 字符串 (strings), 元组 (tuples)
- 函数 (函数是一等公民)

> 创建含有多个重复元素的数组

~~~rust
// 创建一个含有32个`2`的数组
let arr = [2; 32];
~~~

> 数组切片

- 切片类型的形式为`&[T]`，例如`&[i32]`。
- 切片表示引用数组中的异步伏尔加所形成的视图。
- 切片不能直接创建，需要从别的变量借用（borrow）。
- 切片可以是可变的，也可以是不可变的。

~~~rust
let arr = [0, 1, 2, 3, 4, 5];
// 取一个数组的切片,这个切片包含数组的所以元素
let total_slice = &slice;
// 同上，完整写法
let total_slice = &slice[..];
// 取数组中第三个到第五个元素的切片
let partial_slice = &arr[2..5];
~~~

> 字符串

- Rust有两种字符串: `String` 和 `&str`
- `String` 是在堆上分配空间，可以增长的字符序列
- `&str` 是 `String` 的切片类型
- 形如 `"foo"` 的字符串字面值都是 `&str` 类型的

> str是没有大小的类型，编译期不知道大小，因此无法独立存在

~~~rust
let s: &str = "galaxy";
let s2: String = "galaxy".to_string();
let s3: String = String::from("galaxy");
let s4: &str = &s3;
~~~

> 元组

- 元组是固定大小的，有序的，异构的列表类型
- 可以通过下标来访问元组的分量，例如 `foo.0`
- 可以使用 `let` 绑定来解构

~~~rust
let foo: (i32, char, f64) = (72, 'H', 5.1);
let (x, y, z) = (72, 'H', 5.1);
let (a, b, c) = foo;
~~~

> 向量 `Vec<T>`

- 标准库提供的类型，可直接使用
- Vec是分配在堆上的，可增长的数组
    - 类似C++中的 `std::vector` , Java 中的 `java.util.ArrayList`
- `<T>` 表示泛型，使用时代入实际的类型
    - 例如，元素是 `i32` 类型的 Vec 写作 `Vec<i32>`。
- 使用 `Vec::new()` 或 `vec!` 宏来创建 `Vec`。

~~~rust
let v0: Vec<i32> = Vec::new();

let mut v1 = Vec::new();
v1.push(1);
v2.push(2);
v3.push(3);

let v2 = vec![1, 2, 3];

// 4个0
let v3 = vec![0; 4];
~~~

- 向量可以像数组一样使用 `[]` 来访问元素。
    - 在 Rust 中不能用 i32/i64 等类型的值作为下标访问元素。
    - 必须使用 `usize` 类型的值，因为 `usize` 保证和指针是一样长度的。
    - 其他类型要显式转换成 usize;

~~~rust
let i: i8 = 2;
let y = v2[i as usize];
~~~

> 类型转换

- 用 `as` 进行类型转换 (cast)
- 只能在可以安全转换的类型之间进行转换操作
    - 例如，`[i16; 4]` 不能转换为 `char` 类型
    - 有不安全的机制可以做这样的事情，但代价是编译器就无法确保安全性。

> 引用

- 在类型前面写 `&` 表示引用类型: &i32。
- 用 `&` 取引用
- 用 `*` 解引用
- 在 Rust 中，引用保证是合法的
    - 合法性要通过编译期检查
- 因此，Rust中引用和一般意义的指针是不一样的

~~~rust
let x = 12;
let ref_x = &x;
println!("{}", *ref_x); // 12
~~~

> for循环

完全抛弃传统写法，使用迭代器语法，类似kotlin

- `n..m` `n..=m` 这种语法就是创建一个迭代器

~~~rust
for i in 1..=100 {
    println!("{}", i);
}

let arr = [0, 1, 2, 3, 4];

for x in &arr {
    println!("{}", x);
}
~~~

> match 模式匹配

~~~rust
let x = 3;
match x {
    1 => println!("one fish"),
    2 => {
        println!("two fish");
        println!("two fish");
    },
    _ => println!("no fish for you"),
}
~~~

- 匹配语句由一个表达式 (x) 和一组 value => expression 的分支语句组成
- 整个匹配语句被视为一个表达式来求值
    - 与 if 类似，所以分支都必须是相同的类型
- 下划线 `(_)` 用于捕捉所有情况。

~~~rust
let x = 3;
let y = -3;
match (x, y) {
    (1, 1) => println!("one"),
    (2, j) => println!("two, {}", j),
    (_, 3) => println!("three"),
    (i, j) if i > 5 && j < 0 => println!("On guard!"),
    (_, _) => println!(":<"),
}
~~~

- 匹配的表达式可以是任意表达式，包括元组和函数调用。
    - 构成模式 (patterns)
    - 匹配可以绑定变量，_用来忽略不需要的部分。
- 为了通过编译，必须写穷尽的匹配模式。
- 可以用 if 来限制匹配的条件。

