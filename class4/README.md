# 泛型、特型与生命周期

> 泛型

- 将类型作为参数，变成泛型枚举类型
  - 考虑以下类似标准库的枚举类型

~~~rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
~~~

- T 和 E 是泛型类型

> 泛型实现

- 为泛型结构体或枚举类型定义实现时，在 impl 代码段的开头声明泛型类型

~~~rust
impl <T, E> Result<T, E> {
    fn is_ok(&self) -> bool {
        match *self {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
~~~

> 泛型函数

~~~rust
fn foo<T, U>(x: T, y: U) {
    // ...
}
~~~

> 特型共性的需求

- 一些类型具有共性。例如，支持美观打印，判断相等，比较大小等功能
- 针对每种类型进行实现是可行的，但是缺乏结构性。

~~~rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn format(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    fn equals(&self, other: Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
~~~

- 为了抽象类型共性机制，Rust 使用**特型(trait)** 的概念。
- 使用 **trait** 代码段来定义特型，列出特型所需的方法。
  - 与 impl 代码段不同。
  - 大多数方法只列出方法的签名，不包含定义。

~~~rust
trait PrettyPrint {
    fn format(&self) -> String;
}
~~~

> 实现特型

- 使用 impl Trait for Type 代码段来实现特型
  - 所有特型所指定的方法都必须实现
- 对于一种类型实现一种特型，要匹配一个对应的 impl 代码段
- 在特型的 impl 代码段中，同样可以使用 self/&self 参数

~~~rust
struct Point {
    x: i32,
    y: i32,
}

impl PrettyPrint for Point {
    fn format(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
~~~

> 对类型参数的约束要求

- 在使用泛型的场景中，有时候需要对泛型的类型参数做一定的约束（也就是满足一定的条件）。
  - 回忆之前提到的 C++ 在 C++20 之前由于没有对泛型的类型参数进行约束而产生的编译时问题。

> 使用特型约束的泛型

~~~rust
fn cloning_machine<T: Clone>(t: T) -> (T, T) {
    (t.clone(), t.clone())
}

fn cloning_machine_2<T>(t: T) -> (T, T) 
    where T: Clone {
    (t.clone(), t.clone())
}
~~~

> 多种特型约束

- 目前还不能指定反向类型约束（比如不能指定一种类型 T 不支持 Clone 类型）

~~~rust
fn clone_and_compare<T: Clone + Ord>(t1: T, t2: T) -> bool {
    t1.clone() > t2.clone()
}
~~~

> 特型约束的泛型结构化数据类型

- 需要在定义结构体或枚举类型的代码段头部和 impl 代码段头部都声明泛型类型
- 只有 impl 代码段的特型约束是必须指定的
  - 可以对同一泛型类型实现不同特型约束的 impl

~~~rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}

trait PrettyPrint {
    fn format(&self) -> String;
}

impl<T: PrettyPrint, E: PrettyPrint> PrettyPrint for Result<T, E> {
    fn format(&self) -> String {
        match *self {
            Ok(t) => format!("Ok({})", t.format()),
            Err(e) => format!("Err({})", e.format()),
        }
    }
}
~~~

> 示例: 相等关系

~~~rust
enum Result<T, E> { Ok(T), Err(E) }

trait Equals {
    fn equals(&self, other: &Self) -> bool;
}

impl<T: Equals, E: Equals> Equals for Result<T, E> {
    fn equals(&self, other: &Self) -> bool {
        match (*self, *other) {
            Ok(t1), Ok(t2) => t1.equals(t2),
            Err(e1), Err(e2) => e1.equals(e2),
            _ => false
        }
    }
}
~~~

> 特型的继承

- 特型之间存在逻辑上的先后关系
  - 例如，Eq 需要先实现 PartialEq，Copy 需要先实现 Clone
- 下面的代码表示实现 Child 特型要先实现 Parent 特型。

~~~rust
trait Parent {
    fn foo(&self) {
        // ...
    }
}

trait Child: Parent {
    fn bar(&self) {
        self.foo();
        // ...
    }
}
~~~

> 默认方法

- 特型可以指定默认的方法实现
  - 用于避免重复实现那些具有一般意义下常见方式的方法。
- 当某个方法在特型中提供默认实现时，特型的实现中就不用再定义这个方法
- 定义默认实现的方式是在 trait 方法段写出方法的实现。

~~~rust
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

trait Eq: PartialEq<Self> {}
~~~

- 一些特型实现起来比较直观，编译器可以自动完成。
- 是否 #[derive(...)] 属性让编译器完成响应特型的自动实现。
- 这样做可以避免重复手动实现诸如 Clone 这样的特型

~~~rust
#[derive(Eq, PartialEq, Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E)
}
~~~

> 特型自动获得的限制

- 只能自动获得下列核心特型:
  - Clone, Copy, Debug, Default, Eq.
  - Hash, Ord, PartialEq, PartialOrd
- 可以使用宏来完成自定义特型的自动获得
- 注意: 特型的自动获得需要满足下列条件:
  - 类型的所有成员都能自动获得指定的特型
  - 例如，Eq 不能在包含 f32 的结构体类型上自动获得，因为 f32 不是 Eq 的。

> 核心特型

- 有必要了解下列 Rust 的核心特型:
  - Clone, Copy
  - Debug
  - Default
  - Eq, PartialEq
  - Hash
  - Ord, PartialOrd

> Clone

~~~rust
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { /** ... **/  }
} 
~~~

- Clone 特型定义了如何复制 T 类型的一个值。
- 解决所有权问题的另一种方法。
  - 克隆一个对象，而不是获得所有权或者借用所有权。

> Clone 示例

~~~rust
#[derive(Clone)]
struct Foo {
    x: i32,
}

#[derive(Clone)]
struct Bar {
    x: Foo,
}
~~~

> Copy

~~~rust
pub trait Copy: Clone {  }
~~~
- Copy 特型表示一种类型是**拷贝语义**, 而不是 Rust 默认的**移动语义**
- 类型必须可以通过位拷贝来进行拷贝 (memcpy)。
  - 包含引用的类型不能实现 Copy 特型。
- 标记特型: 没有实现任何方法，只有标记行为。
- 一般来说，如果一种类型可以拷贝，就应该实现 Copy 特型。

> Debug

~~~rust
pub trait Debug {
    fn fmt(&self, &mut Formmatter) -> Result;
}
~~~

- 定义能够使用 {:?} 格式选项进行输出。
- 产生的是用于调试的输出信息，不是美观的输出格式。
- 一般来说，Debug 特型应该通过自动获得的方式实现。

> Default

~~~rust
pub trait Default: Sized {
    fn default() -> Self;
}
~~~

> Eq 与 PartialEq

~~~rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool { ... }
}

pub trait Eq: PartialEq<Self> {}
~~~

- 定义通过 == 操作符判断相等关系的特型

> Eq 与 PartialEq 的解释

- PartialEq 表示**部分等价关系(partial equivalence relation)**
  - 传递性: 若 a == b, 则 b == a
  - 传递性: 若 a == b 且 b == c, 则 a == c
- ne 具有使用 eq 的默认实现。
- Eq 表示**等价关系(equivalence relation)**
  - 除对称性和传递性外，还需要满足**自反性**。
  - 自反性: a == a
- Eq 没有定义更多的方法，也是一种标记特型

> Hash

~~~rust
pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
        where Self: Sized { ... }
}
~~~

- 表示可哈希的类型
- H 类型参数是抽象的哈希状态，用于计算哈希值。
- 如果同时实现了 Eq 特型，需要满足如下重要性质

k1 == k2 -> hash(k1) == hash(k2)

> PartialOrd

~~~rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool { ... }
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}
~~~

- 表示 (可能) 可以进行比较的特型。

> PartialOrd 的解释

- 对所有的 a, b, c, 比较操作必须满足:
  - 反对称性: 若 a < b, 则 !(a > b); 若 a > b, 则 !(a < b)。
  - 传递性: 若 a < b 且 b < c, 则 a < c; 对 == 和 > 同性成立。
- lt，le，gt，ge 具有基于 partial_cmp 的默认实现。

> Ord

~~~rust
pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;
}
~~~

- 实现该特型的类型形成全序关系 (total order)。
- 全序关系需要满足的性质除反对称性和传递性外，还需要满足完全性:
  - 对所有的 a 和 b, 有 a <= b 或 b <= a 成立
- 此特型可以保证类型的值能够按字典序排序。

> 关联类型的需求

- 考虑如下的 Graph 特型:

~~~rust
trait Graph<N, E> {
    fn edges(&self, &N) -> Vec<E>;
}
~~~

- 这里，N和E是泛型类型参数，但是它们和 Graph 之间的联系不明确。
- 如果有函数要使用 Graph 时，它必须也是 N 和 E 的泛型。

~~~rust
fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) 
    -> u32 {
    /* ... */
}
~~~

> 关联类型的需求

- 考虑如下的 Graph 特型

~~~rust
trait Graph<N, E> {
    fn edges(&self, &N) -> Vec<E>;
    // etc
}
~~~

- 这里，N和E是泛型类型参数，但是它们和 Graph 之间的联系不明确。
- 如果有函数要使用 Graph 时，它必须也是 N 和 E 的泛型。

~~~rust
fn distance<N, E, G: Graph<N,E>>(graph: &G, start: &N, end: &N) -> u32 { /* ... */ }
~~~

> 关联类型

- 使用关联类型来反映这种设计上的逻辑。
- 使用特型代码段里的 type 定义来表示特型关联的泛型类型。
- 特型在实现时来指定关联类型实际指代的类型。

~~~rust
trait Graph {
    type N;
    type E;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

impl Graph for MyGraph {
    type N = MyNode;
    type E = MyEdge;
    fn edges(&self, n: &MyNode) -> Vec<MyEdge> { /* ... */ }
}
~~~

> 特型的作用域规则示例: Display

~~~rust
pub trait Display {
    fn fmt(&self, &mut Formatter) -> Result<(), Error>;
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {}, {}", self.x, self.y)
    }
}
~~~

> Drop

~~~rust
pub trait Drop {
    fn drop(&mut self);
}
~~~

- Drop特型提供drop方法，用于将对象销毁，会由编译器自动生成，不能显式调用。

- 一般情况下，不需要实现Drop。
  - 默认的实现可以正常工作
  - 也不需要用自动获得功能去产生
- 什么时候需要手动实现？
  - 如果在对象销毁时有特殊行为
  - 例如，Rust的引用计数指针类型 `Rc<T>` 就有特殊的 Drop 规则: 当引用计数大于1时，drop 只对计数器 -1；但是当 -1 之后引用计数器降为0时，要真的删除这个对象

> Sized 和 ?Sized

- Sized 表示一种类型在编译时就可以知道是固定的大小
- 而 ?Sized 表示一种类型的大小可能是不固定的
- 默认情况下，所有类型都是 Sized，而指定 ?Sized 可以撤销这一规定
  - 例如，像 [T] 和 str (没有 &) 都是 ?Sized 的
- 一般来说，跟指针相关的泛型的类型参数里的特型约束会出现 ?Sized，例如 `Box<T>` 就有 T: ?Sized。
- 很少直接使用这两种特型，一般都是在特型约束中出现的。

> 动态分发

静态分发就是使用泛型，一个类型参数生成一个对应的函数

- Rust 也可以通过**特征对象**进行**动态分发**
- 特型对象要用像 `Box<dyn Foo>` 或 `&dyn Foo` 的形式来使用 （相当于C++中的对象指针）
- 背后的数据类型要实现 Foo 特型
- 当使用动态分发时，特型背后的数据类型被抹去，无法获得

> 特型对象的实现规则

- 使用特型对象是，只能在运行时进行方法的分发
  - 编译器不知道实际类型，类型信息已经抹去
- 这样做会带来一定的运行时开销，但在处理一些情况时会有用（例如动态大小的类型）
  - 实际上特型对象只能通过指针的方式来使用，会增加指向方法的 vtable

> 对象安全性

- 不是所有的特型都可以以特型对象的形式安全地使用
- 例如创建 &dyn Clone 会引起编译错误，因为 Clone 不是对象安全的
- 特型是对象安全的，需要满足以下条件
  - 所有超特型也都是对象安全的
  - 不能以 Sized 为超特型
  - 所有关联函数能够从特型对象进行分发
    - 不带任何类型参数
    - 方法接受方(receiver)外，其他地方不能使用 Self 类型
    - 接收方是引用或者某种指针形式的类型，如 `&Self`，`&mut Self`，`Box<Self>` 等。
    - 没有 where Self: Sized 子句

> 生命周期的显式表示

- 通常情况下，引用具有隐式的生命周期，不需要额外关注。

~~~rust
fn foo(x: &i32) {
    // ...
}
~~~

- 在必要的时候，也可以显式指定生命周期

~~~rust
fn bar<'a>(x: &'a i32) {
    // ...
}
~~~

> 多个生命周期

~~~rust
fn borrow_x_or_y(x: &'a str, y: &'a str) -> &'a str;

fn borrow_p<'a, 'b>(p: &'a str, q: &'b str) -> &'a str;
~~~

> 结构体相关的生命周期

- 结构体和结构体成员也可以具有生命周期
  
~~~rust
struct Pizza(Vec<i32>);
struct PizzaSlice<'a> {
    pizza: &'a Pizza,
    index: u32,
}

let p1 = Pizza(vec![1, 2, 3, 4]);
{
    let s1 = PizzaSlice { pizza: &p1, index: 2 };
}
~~~

> 生命周期之间的关系

- 如果结构体或枚举类型的成员是引用，则必须显式指定生命周期

~~~rust
struct Foo<'a, 'b> {
    v: &'a Vec<i32>,
    s: &'b str,
}
~~~

- 可以指定生命周期之间的关系（“活得比你久”）
  - 词法与泛型中的特型约束相同 <'b: 'a> ('b 生命周期需要涵盖 'a 生命周期)
  
> impl代码段中的生命周期

~~~rust
impl<'a, 'b> Foo<'a, 'b> {
    fn new(v: &'a Vec<i32>, s: &'b str) -> Foo<'a, 'b> {
        Foo {
            v: v,
            s: s
        }
    }
}
~~~

> 静态生命周期

- 'static 表示整个程序的生命周期，拥有 'static 生命周期的引用在整个程序运行时都有效

