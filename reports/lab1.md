# 简单总结你实现的功能（200字以内，不要贴代码）

完成了`sys_task_info`系统调用

通过在`syscall`下添加了全局系统调用记录数组来记录系统调用发生的次数

通过在进程控制块内添加记录第一次运行时间的变量，来实现获得当前进程运行时间的统计

# 完成问答题

* ### 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 [三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2024S/tree/master/src/bin) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

这样的程序会触发异常，并被操作系统结束

```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

使用的sbi为`RustSBI-QEMU Version 0.2.0-alpha.2`

* ### 深入理解 [trap.S](https://github.com/LearningOS/rCore-Tutorial-Code-2024S/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

* L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

`__restore`是从`__switch`进入的，这时的`a0`代表了` current_task_cx_ptr: *mut TaskContext`

其他情况下，`a0`保存了返回值(来自`trap_handle`)

`__restore` 第一，可以使用在用于任务第一次进入时，从内核跳转到用户程序，第二，可以使用在`trap_handle`处理完毕后返回用户程序

* L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

```
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```

特别处理了`sstatus`，`sepc`，`sscratch`这三个控制状态寄存器，他们的作用分别是

`sstatus` 是一个重要的状态寄存器，用于控制和反映处理器的状态，此处用于返回正确的特权级

`sepc` 寄存器存储了异常发生时的程序计数器（PC）的值，此处用于返回正确的用户程序地址

`sscratch` 寄存器通常用于在异常处理过程中暂存一个临时值，此处保存内核栈指针

* L50-L56：为何跳过了 `x2` 和 `x4`？

```
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```

因为它们分别是`Stack pointer`和`Thread pointer`，这些寄存器会被用于其他功能并被妥善处理

* L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

```
csrrw sp, sscratch, sp
```

该指令使得 `sp` 寄存器的内容和 `sscratch` 寄存器的内容交换，此处，用于切换到用户栈指针

* `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

发生在`sert`指令

这时`riscv`硬件规定的功能，使用后会根据`sstatus`和`sepc`跳转到正确的特权级和地址，在`rcore`中，通过修改这两个控制状态寄存器来进入用户态和正确的程序位置

* L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

```
csrrw sp, sscratch, sp
```

该指令使得 `sp` 寄存器的内容和 `sscratch` 寄存器的内容交换，此处，用于切换到内核栈指针

* 从 U 态进入 S 态是哪一条指令发生的？

`ecall`或触发中断和其他异常

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > *无*

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

# 其他
