## 首先第一步，认识运算符

* ### & 
(与)：对每个二进制位进行逻辑与运算。如果两个相应位都是1，则该位的结果为1，否则为0。

```
010 , 110  -> 010
```

* ### |
(或)：对每个二进制位进行逻辑或运算。如果两个相应位都是0，则该位的结果为0，否则为1。

```
010 , 110  -> 001
```

* ### ^
(异或)：对每个二进制位进行异或运算。如果两个相应位值不同，则该位结果为1，否则为0。

```
010 , 110  -> 101
```

* ### ~
(取反)：对每个二进制位取反。0变成1，1变成0。

```
010  -> 101
```

* ### >>
(右移)：将二进制数的每个位向右移动n位。

```
010 >> 1 ->  001
```

* ### <<
(左移)：将二进制数的每个位向左移动n位。

```
010 << 1 ->  100
```

## 去接任务 [Missions](./mission_list.md)

## 进阶解析运算符应用

在 & 和 | 是一对，他们各自对 0 和 1 上的偏好选择

在 ^ 和 ~ 是一对，他们对 0 和 1 上没有逻辑上的偏好

## 掩码 

是用 ! 或者 ~ 来取得

```
 !(1 << 2)
 
 100 -> 011
```

而 ! 是逻辑非运算符，它通常用于比较运算中，如 !(x == 0) 就相当于 x != 0。