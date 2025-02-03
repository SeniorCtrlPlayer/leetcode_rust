# leetcode_rust

## String

- `` 在进行字串对比时，可以直接用切片与String进行比较，但是是否支持单个字符的切片还不确定。#String
- `bedc0bb0` 如果在遍历Vec时，有in-place修改，应该使用索引遍历，而不是通过迭代器遍历，因为迭代器会直接拿走vec的所有权，而不仅仅是vec[i]。#Vec
- `dec59f9a` let Some语句定义的变量，也属于let语句所在的作用域。take()在拿走所有权时，会将原有的变量设置为None，所以原有变量必须是Option<>。
- `57200987` 通过.chars()遍历String数组，因为这样才能与字符进行比较，String.pop()方法抛出的类型为Some('[')。#String
- `34a4ecb7` 如何多次遍历Vec，因为for循环默认使用的是.into_iter()会转移所有权，所以不能多次遍历，想要多次遍历某个Vec时，应该使用.iter()。#Vec
- `ff5977a0` 如何遍历访问String数组，这里需要通过String的nth方法来访问索引对应的字符，并且还需要使用unwrap。比c++使用[]操作符访问麻烦很多。unwrap的作用暂不明确。#String