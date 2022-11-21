# 2.所有权与结构化数据

## 大作业 Wordle Game

[大作业一：Wordle](https://lab.cs.tsinghua.edu.cn/rust/projects/wordle/)

只上了两节课就要做这种难度的作业我是没想到的。对应了一下课表，上到这里他们才刚接触Rust三天，这就是清华爷吗

对我来说写起来还是没有很大难度（毕竟之前也算学过 Rust)

### 库

- colored 用于打印彩色文字
- clap 用于解析命令行参数

clap 这个库挺让我惊喜的，它能以一种相当简洁优雅的方式构建 cli 工具，而且也让我见识到了宏带来的魔法——这种高自定义度的语法也是可以允许的吗？

~~~rust
use clap::{command, arg};

let matches = command!()
        .arg(arg!(--word <VALUE> "the target word to guess"))
        .get_matches();

let word = if let Some(str) = matches.get_one::<String>("word") {
        str
    } else {
        "APPLE"
    };
~~~

## 

> 移动语义

~~~rust
let v1 = vec![1, 2, 3];

// Ownership of the Vec object moves to v2
let v2 = v1;

println!("{}", v1[2]); // error: use of moved value `v1`
~~~

- 拷贝数据代价高昂，不希望默认这样做
- 数据不能有多个所有者
- 解决方案: 把向量的所以权移交给v2, 并将v1置于无效的状态
- Rust 能在编译时发现使用了无效变量绑定的问题，抛出编译错误

> 借用

- 如果每需要用到一个值就把所有权交给其他函数，其他函数用完再传回来，操作会非常繁琐
- 这种情况下，与其移交所有权，不如进行**借用**(borrow)
- 可以通过对变量取引用来借用变量中的数据的所有权，此时所有权未发生变化
    - 当引用超过作用域，借用也随之结束
    - 原来的变量依然拥有对数据的所有权

~~~rust
let v = vec![1, 2, 3];

let v_ref = &v;

assert_eq!(v[1], v_ref[1]);
~~~

- 带来的问题: 会给原来的变量增加限制
- 当一个变量有引用存在时，不能移交它所绑定的数据的所有权
  - 因为所有权移交会导致引用失效

~~~rust
let v = vec![1, 2, 3];
let v_ref = &v;
let v_new = v;
println!("{:?}", v_ref);
~~~

非词法生命周期

~~~rust
fn main() {
    let v = vec![1, 2, 3];
    let v_ref = &v;
    let v_new = v;
}
~~~

这段代码可以通过编译，因为我们后续没有再使用 v_ref 了，它的生命周期可以提前结束

~~~rust
fn push2(vec_ref: &mut Vec<i32>, x: i32) {
    // error: cannot move out of borrowed content
    let vector = *vec_ref;
    vector.push(x);
}

fn main() {
    let mut vector = vec![];
    push2(&mut vector);
}
~~~

不能通过解引用然后绑定给变量，这样做会引起引用的转移 (同时引用还没有失效)

- Rust 在大多数情况会自动解引用，但有些情况需要显式解引用
  - 往解引用后的结果写入内容
  - 其他可能会引起歧义的情况
  
~~~rust
let mut a = 5;
let ref_a = &mut a;
*ref_a = 4;
println!("{}", *ref_a + 4);
// ===> 8
~~~

> Copy 类型

- Rust定义了 Copy 特征 (trait), 表示一种类型可以拷贝，而不是用默认的移动语义。
    - 通常这样的类型都是轻量级的，拷贝行为是按位进行的（联系C++中默认的拷贝构造行为）。
    - 大多数基本类型是 Copy 类型 (i32, f64, char, bool 等等)
    - 包含引用的类型不是 Copy 类型 (例如，Vec, String)

~~~rust
let x: i32 = 12;
let y = x; // `i32` is `Copy`, so it's not moved :D
println!("x still works: {}, and so does y: {}", x, y);
~~~

- 不能在某个对象不存在后继续保留它的引用
- 一个对象可以同时存在多个不可变引用 (&T)
- 或者仅有一个可变引用 (&mut T)
- 以上两者不能同时存在

> 借用的作用

- 考虑迭代器的场景：在修改集合的同时进行迭代访问会引起迭代器失效
- 但是这种代码在 C++ 和 Java 等语言中都是可以写出来的
    - 但是会引发错误，比如在Java中会抛出 `ConcurrentModificationException`

~~~rust
let mut vs = vec![1, 2, 3, 4];
for v in &vs {
    vs.pop();
    // ERROR: cannot borrow `vs` as mutable because
    // it is also borrowed as immutable
}
~~~

- pop 需要以可变方式借用 vs 来修改数据
- 但是vs正在以不可变的方式被循环借用

这样从编译期就避免了不安全的代码

~~~rust
let y: &i32;
{
    let x = 5;
    y = &x; // error: `x` does not live long enough
}
println!("{}", *y);
~~~

也不能将借用绑定给自己生命周期之外的变量

> 例子: 向量

~~~rust
let mut vs = vec![0, 1, 2, 3, 4, 5, 6];

// Borrow immutably
for v in &vs { // Can also write `for v in vs.iter()`
    println!("I'm borrowing {}.", v);
}

// Borrow mutably
for v in &mut vs { // Can also write `for v in vs.iter_mut()`
    *v = *v + 1;
    println!("I'm mutably borrowing {}.", v);
}

// take ownership of vs
for v in vs {
    println!("I now own {}! AHAHAHAHA!", v);
}

// `vs` is no longer valid
~~~

> 切片

- 切片是一种特殊形态的引用，表示引用序列中的一个片段
- 切片构造的语法 `&x[s..t]`, 其中 s 和 t 还可以根据情况省略
    - 也可以使用 `&x[s..=t]` 的语法
- 可变性以及引用的约束条件对切片同样适用

~~~rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
~~~

> 结构体与可变性

- 结构体没有域级的可变性控制
- 可变性是变量绑定的属性，跟类型无关
- 域级的可变性（内部可变性）可以通过 Cell 类型来实现

~~~rust
struct Point {
    x: i32,
    mut y: i32 // Illegal!
}
~~~

> 结构体的访问权限

- 结构体在它所在模块的名字空间里
    - Point 的完整名字是 foo::Point
- 结构体的域是可私有的
  - 可以通过 pub 关键字变成公有
- 私有域只能在结构体所在模块内访问

~~~rust
mod foo {
    pub struct Point {
        pub x: i32,
        y: i32,
    }
}

fn main() {
    let b = foo::Point { x: 12, y: 12 }; // error: y is private
}
~~~

> 结构体的更新语法

解构就完事了，但结构出来是拷贝还是移动？

规则是一样的，如果实现了 Copy trait 就是拷贝，没有默认就是移动

跟js不同的是，解构的结构体类型必须一致，从不同类型的结构体获得相同类型的域也是不行的

~~~rust
struct Foo { a: i32, b: i32, cz; i32, d: i32, e: i32 }

let mut x = Foo { a: 1, b: 1, c: 2, d: 2, e: 3 };
let x2 = Foo { e: 4, ..x };

// Useful to update multiple fields of the same struct:
x = Foo { a: 2, b: 2, e: 2, .. x };
~~~

> 元组结构体

- 元组结构体 (tuple struct) 是结构体的一种形态，有结构体名字，但没有域的名字
- 可以像元组那样通过数字来访问域，例如 x.0，x.1 等等
- 也可以通过match来匹配

~~~rust
struct Color(i32, i32, i32);

let mut c = Color(0, 255, 255);
c.0 = 255;

match c {
    Color(r, g, b) => println!("({}, {}, {})", r, g, b)
}
~~~

- 可以用来创建新的类型，而不仅仅是一个别名
    - 被称为 “新类型” 模式（“newtype” pattern)
- 两种类型在结构上是相同的，但并不等价 (不是同一种类型)

~~~rust
//  Not equatable
struct Meters(i32);
struct Yards(i32);

// May be compared using `==`, added with `+`, etc.
type MetersAlias = i32;
type YardsAlias = i32;
~~~

> 单位元结构体 (零大小的类型)

- 可以声明零大小的结构体
  - 这样的结构体没有域
- 这种结构体也是可以实例化的
- 通常被用来作为其他数据结构的标记类型
  - 例如，可以用来指示一个容器保存的数据的类型

~~~rust
struct Unit;

let u = Unit;
~~~

> 枚举

- 枚举 (enum)，是**和类型**(sum type),用来表示可以是多选一的数据
  - 相对地，结构体和元组都是**积类型**(product type)
- Rust 的枚举比 C/C++, Java 等语言中的枚举要强很多。
- 枚举的每种变体 (variant) 可以:
    - 没有数据 (单位元变体)
    - 有命名的数据域 (结构体变体)
    - 有不命名的有序数据域 (元组变体)

~~~rust
enum Resultish {
    Ok,
    Warning { code: i32, message: String },
    Err(String)
}
~~~

> 递归类型

尝试创建一个链表

~~~rust
// error: recursive type `List` has infinite size
enum List {
    Nil,
    Cons(i32, List),
}
~~~

- 编译时会出现无穷大小的问题。
- 结构体和枚举默认情况下是内联储存的,因此不能递归
    - 他们的元素正常情况下不使用引用来存储，但可以显式指定

> Box

- `Box<T>` 是指向堆上对象的指针，作为对象的唯一所有者
    - Box 唯一拥有它的数据(T 类型)，不能创建别名
- Box 在超过作用域时会自动销毁
- 通过 Box::new() 来创建 Box。

~~~rust
let boxed_five = Box::new(5);

enum List {
    Nil,
    Cons(i32, Box<List>), // OK!
}
~~~

> 方法

~~~rust
impl Point {
    pub fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.distance();
}
~~~

- 结构体和枚举的方法可以实现在 impl 代码块里。
- 和域相同，方法也通过点记号进行访问。
- 可以用 pub 将方法声明为公开的，impl 代码块本身不需要是 pub 的。
- 对枚举和对结构体是一样的。

> 方法与所以权

方法的第一个参数 (名字为 self) 决定这个方法需要的所有权种类

- &self: 方法借用对象的值
    - 一般情况下尽量使用这种方式，类似于 C++ 中的常成员函数
- &mut self: 方法可变地借用对象的值
    - 在方法需要修改对象时使用，类似于 C++ 中的普通成员函数
- self: 方法获得对象的所以权
    - 方法会消耗掉对象，同时可以返回其他的值

> 关联函数

~~~rust
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y:  y }
    }
}

fn main() {
    let p = Point::new(1, 2);
}
~~~

- 关联函数与方法类似，但是没有 self 参数
  - 调用时使用名字空间语法: `Point::new()`, 而不是 `Point.new()`
  - 类似 C++ 中的静态成员函数
- 一般会创建一个名为 new 的关联函数起到构造函数的作用。
  - Rust 没有内置的构造函数语法，也不会自动构造

- 方法，关联函数和函数不能重载
  - 例如，Vec::new() 和 Vec::with_capacity(capacity: uszie) 都是 Vec 的构造函数。
- 方法不能被继承。
  - Rust 中结构体和枚举用的是合成(compose)的方法
  - 特型(trait)具有基本的继承功能

> 模式匹配

- 一些结构体匹配的技巧

~~~rust
match p {
    Point { y: y1, x: x1 } => println!("({}, {})", x1, y1)
}

match p {
    Point { y, .. } => println!("{}", y)
}
~~~

- 匹配时域不一定要按照结构体声明时的顺序
- 将结构体的域列出来，和对应的变量名做绑定
  - 可以使用 struct_field: new_var_binding 的语法来改变绑定的变量名。
- 忽略部分域: 使用 .. 忽略所有没有提到名字的域

- 注意 match 可能会递交所有权
- 使用 ref 可以在匹配时获得一个变量的引用 (否则是绑定，直接获得所有权)

~~~rust
let mut x = 17;

match x {
    ref r => println!("Of type &i32: {}", r),
}

match x {
    ref if x == 5 => println!("{}", r),
    ref mut r => *r = 5
}
~~~

> if let 语句

~~~rust
// 使用match
match make_request() {
    Resultish::Err(_) => println!("Total and utter failure."),
    _ => println!("ok")
}

// 使用 if let
if let Resultish::Err(s) = make_request() {
    println!("Total and utter failure: {}", s);
} else {
    println!("ok.");
}
~~~

> while let 语句

~~~rust
let v = vec![0, 1, 2];
while let Some(x) = v.pop() {
    println!("{}", x);
}
~~~

> 内部绑定

- 对于更复杂的数据结构，可以用 @ 创建内部元素的变量绑定

~~~rust
#[derive(Debug)]
enum A { None, Some(B) }
#[derive(Debug)]
enum B { None, Some(i32) }

fn foo(x: A) {
    match x {
        a @ A::None => println!("a is A::{:?}", a),
        ref a @ A::Some(B::None) => println!("a is A::{:?}", *a),
        A::Some(b @ B::Some(_)) => println!("b is B::{:?}", b),
    }
}
~~~

> 模式匹配的穷尽性

- match 的所有分支对于模式来说必须是穷尽的

~~~rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1)
            // error: not exhaustive
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
~~~

> 函数参数也可以通过模式匹配解构

~~~rust
fn tuple_add((a, b): (i32, i32)) -> i32 {
    a + b
}
~~~

