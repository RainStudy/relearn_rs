# 标准库

> 字符串

- `&str` 是字符串切片，是切片的一种
- 形如 “string literal" 的字符串是 `&str` 类型的
- 不能用方括号来做形如 `some_str[i]` 的索引，因为每个 Unicode 字符可能有多个字节。
- 正确的做法是在 `chars()` 中迭代:

~~~rust
for c in "1234".chars() { ... }
~~~

> String

- String 是分配在堆上的，可以动态增长
  - 和 Vec 类似，实际上就是在 `Vec<u8>` 外面包了一层。
- 也不能用下标来索引。
  - 可以通过 `s.nth(i)` 来访问某个字符。
- 通过取引用的方式可以获得 &str (&String 就是 &str)。

~~~rust
let s0: String = String::new();
let s1: String = "foo".to_string();
let s2: String = String::from("bar");
let and_s: &str = &s0;
~~~

> str

- 如果 `&str` 是一种字符串类型，那么 `str` 到底是什么
- `str` 是一种 `Unsized` 的类型，也就是编译时大小未知
    - 不能直接绑定 `str`，只能通过引用的形式来使用

> 字符串连接操作和实现

- 可以用 + 连接一个 `String` 和一个 `&str` 类型的字符串 (注意顺序):
  
~~~rust
let a = String::from("hello");
let b = String::from(" world");
let c = a + &b;
// `a` is moved and can no longer be used here.
~~~

- 如果想保留第一个 `String`, 需要做一份克隆(clone):
  
~~~rust
let a = String::from("hello");
let b = String::from(" world");
let c = a.clone() + &b;
// `a` is still valid here.
~~~

- 如果要连接两个 `&str`, 需要把第一个转换成 `String`:

~~~rust
let a = "hello";
let b = " world";
let c = a.to_string() + b;
~~~

- 连接操作的实现代码:

~~~rust
fn add(mut self, other: &str) -> String {
  self.push_str(other);
  self
}
~~~

> `String` 与 `&str` 并存的设计原因

- `&str` 能够提供 `String` 的一个视图，正如切片之于 `Vec` 向量那样。
- 拷贝 `String` 的代价非常高昂，而且借用的时候并不一定需要整个字符串
- `&str` 提供了一种低开销传递部分 `String` 字符串内容的方法，而且节约内存
- 一般而言，如果要处理字符串，使用 String，同时可以用 `&str` 来借用其中的内容

> Option::unwarp()

~~~rust
fn unwrap<T>(self) -> T {
  match self {
    None => panic!("Called `Option::unwarp()` on a `None` value"),
    Some(value) => value,
  }
}

let x = foo().unwarp();
let y = bar(x);
// ...
~~~

- `Option::unwrap()` 在遇到 `None` 时会恐慌并输出固定的内容
- 更好的做法是调用 `expect(self, msg: String) -> T`。
  - 它可以在遇到 `None` 时以指定的信息执行恐慌。

~~~rust
impl<T> Option<T> {
  fn unwrap_or<T>(self, default: T) -> T {
    match self {
      None => default,
      Some(value) => value,
    }
  }
}
~~~

> Option::unwarp_or()

- 如果对于空值的情况有合理的默认值，可以用 `unwrap_or` 提供

> `Result<T, E>` 的处理原则

- 对于返回结果是 Result 的函数，一定要显式进行处理
  - 可以使用 unwarp/expect，也可以通过匹配合理地处理 Ok/Err 状态
  - 如果不处理，编译器会发出警告
  - 不正确处理可能会带来潜在问题，导致意想不到的情况。

> 自定义 Result 别名

- 一种常见的做法是在自己编写的库使用自定义的错误类型，并定义 Result 的别名

~~~rust
use std::io::Error;

type Result<T> = Result<T, Error>;
~~~

- 除了固定 E = Error 以外与 std::Result 是等价的
- 使用的时候要注意名字空间

~~~rust
use std::io;

fn foo() -> io::Result {
  // ...
}
~~~

> ? 操作符

- 配合 Result 类型

~~~rust
fn read_username_from_file() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username)?;
  Ok(username)
}
~~~

- 配合 Option 类型

~~~rust
fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}
~~~

> ? 操作符的原理

- 作业: 提前传播错误
- 场合: 返回值是 Result 或者 Option 函数中
- 对于 Result 类型
  - 如果是 Err 则提前返回，当前函数立刻返回该错误
  - 否则，从 Ok 中取出返回值作为 ? 操作符的结果
- 对于 Option 类型
  - 如果是 None 则提前返回，当前函数立即返回 None
  - 否则，从 Some 中取出返回值作为 ? 操作符的结果

> `Vec<T>`

- 连续空间，可增长的序列，末尾可以高效增删
- 会发生增长和收缩
- 最常用的容器

> `VecDeque<T>`

- 双端向量，两端可以高效增删
- 用环状缓冲区实现

> `LinkedList<T>`

- 双向链表
- 不能随机索引

> `HashMap<K,V>/BTreeMap<K,V>`

- 映射/字典类型
- 一般使用 `HashMap<K, V>`
  - 需要满足 K: Hash + Eq
  - 使用哈希表实现，没有顺序，效率较高，O(1) 的访存复杂度
- 需要有序的时候用 `BTreeMap<K, V>`
  - 需要满足 K: Ord
  - 使用 B 树实现，有序，效率相对低一些，O(logn) 的访存复杂度

~~~rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 5);

let team_name = String::from("Blue");
let score = scores.get(&team_name);

for (key, value) in &scores {
  println!("{}: {}", key, value)
}
~~~

> 哈希表和所有权

- 对于 Copy 类型，拷贝进哈希表
- 对于非 Copy 类型，移动进哈希表，哈希表拥有所有权

~~~rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point
~~~

> 更新哈希表

~~~rust
// 改写
scores.insert(String::from("Blue"), 10);
// 不存在时添加
scores.entry(String::from("Blue")).or_insert(50);
// 基于旧值更新
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.spilt_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}
~~~

> `HashSet<T>`/`BTreeSet<T>`

看来跟java一样

- 集合，元素是唯一的
- `HashSet<T>` 和 `BTreeSet<T>` 就是在 `HashMap<T, ()>` 和 `BTreeMap<T, ()>` 上包了一层
- 需求和表现跟相应的 Map 相同

> `BinaryHeap<T>`

- 用二叉最大堆实现的优先级队列
- 弹出元素时返回目前堆中的最大值

> 迭代器的定义

~~~rust
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
  // More fields omitted
}
~~~

- 迭代器特型包含一个相关的类型 Item，以及会产生该类型对象的方法 next。
- 其他方法(包括消费器和适配器) 有使用 next 的默认实现版本。
  
> 迭代器与所有权

- 有三种迭代类型:
  - `into_iter()`, 产生 T 类型
  - `iter()`, 产生 &T 类型
  - `iter_mut()`, 产生 &mut T
- 集合可以提供部分或全部接口。

> 迭代器提供 for 循环的语法糖

~~~rust
let values = vec![1, 2, 3, 4, 5];
{
  let result = match values.into_iter() {
    mut iter => loop {
      match iter.next() {
        Some(x) => { /* loop body */ }
        None => break,
      }
    }
  };
  result
}
~~~

> 迭代器的类型转换操作

- 迭代器的处理器接收 Iterator，返回其他类型
  - 例如，map 返回 Map，filter 返回 Filter。
- 这些类型也是实现了Iterator的结构体。
  - 不用太纠结它们的内部结构
- 类型转换主要用于确保类型安全

一系列跟函数式编程范式相性很好的流式操作符

> collect

- collect() 把(惰性的)迭代器变成一个实际的集合。
- collect() 有时候需要提供类型提示来通过编译。
  - 结果可以是任何的集合 （或容器）
  
~~~rust
fn collect<B>(self) -> B where B: FromIterator<Self::Item>

let vs = vec![1,2,3,4];
let set: HashSet<_> = vs.iter().collect();
let set = vs.iter().collect::<HashSet<_>>();
~~~

> fold

~~~rust
fn fold<B, F>(self, init: B, f: F) -> B 
  where F: FnMut(B, Self::Item) -> B;

let vs = vec![1, 2, 3, 4, 5];
let sum = vs.iter().fold(0, |acc, &x| acc + x);
assert_eq!(sum, 15);
~~~

> filter

~~~rust
fn filter<P>(self, predicate: P) -> Filter<Self, P>
  where P: FnMut(&Self::Item) -> bool;
~~~

- filter 接受一个谓词函数 P, 把不符合谓词的元素都去掉
- filter 返回 `Filter<Self, P>`, 需要用 collect 获得集合

> find 和 position

~~~rust
fn find<P>(&mut self, predicate: P) -> Option<Self::Item> 
  where P: FnMut(Self::Item) -> bool;

fn position<P>(&mut self, predicate: P) -> Option<usize>
  where P: FnMut(Self::Item) -> bool;
~~~

- 找出迭代器中第一个满足谓词函数 predicate 的项目
  - find 返回项目本身
  - position 返回项目的索引
- 没找到都返回 None。

> zip

~~~rust
fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
  where U : IntoIterator;
~~~

- 把两个迭代器逐项合并成一个新的迭代器
- 调用形式: a.iter().zip(b.iter())
  - 返回项目的形式: (ai, bi)
- 当一个输入迭代器结束，整个 zip 输出的迭代器结束。

> enumerate

~~~rust
fn enumerate(self) -> Enumerate<Self>;
~~~

- 用于迭代集合同时需要项目和索引。
- 返回 (index, value) 的迭代器，index 是 usize 类型的索引。

> 迭代器适配器

- 适配器(adapters)操作一个迭代器，返回另一个迭代器
- 适配器通常是**惰性的**: 除非说不得不做，不然先不去求值。
- 必须显式使用或者用 for 循环迭代才会去求值。

> map

~~~rust
fn map<B, F>(self, f: F) -> Map<Self, F> 
  where F: FnMut(Self::Item) -> B;

let vs = vec![1, 2, 3, 4, 5];
let twice_vs: Vec<_> = vs.iter().map(|x| x * 2).collect();
~~~

- map 接受一个函数，创建一个迭代器，在每个元素上调用这个函数。
- 完成从集合 `Collection<A>` 和 `A -> B` 函数得到 `Collection<B>` 的操作。
  - 这里，Collection 不是一种实际的类型。

> take 和 take_while

~~~rust
fn take(self, n: usize) -> Take<Self>;

fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
  where P: FnMut(&Self::Item) -> bool;
~~~

- take 创建一个迭代器，返回前 n 个元素。
- take_while 接受一个谓词，迭代直到谓词返回 false。
- 可以在无限范围上使用得到有限序列

~~~rust
for i in (0..).take(5) {
  println!("{}", i); // Prints 0 1 2 3 4
}
~~~

> cloned

~~~rust
fn cloned<'a, T>(self) -> Cloned<Self>
  where T: 'a + Clone, Self: Iterator<Item=&'a T>;
~~~

- 创建一个迭代器，在每个元素上调用 clone 方法
- 相当于 vs.iter().map(|v| v.clone()) 
- 在目前有 &T 迭代器，想有 T 迭代器的时候使用

