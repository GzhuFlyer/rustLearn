#第7章，第十三章，工作空间

```
包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
Crates ：一个模块的树形结构，它形成了库或二进制项目。
模块（Modules）和 use： 允许你控制作用域和路径的私有性。
路径（path）：一个命名例如结构体、函数或模块等项的方式

包中所包含的内容由几条规则来确立。一个包中至多 只能 包含一个库 crate(library crate)；包中
可以包含任意多个二进制 crate(binary crate)；包中至少包含一个 crate，无论是库的还是二进制
的。
```

#使用父模块将两个具有相同名称的类型引入同一作用域
```
use std::fmt; use std::io; fn function1() -> fmt::Result { // --snip-- }fn function2() -> io::Result<()> { // --snip-- }

use std::fmt::Result; use std::io::Result as IoResult; fn function1() -> Result { // --snip-- }fn function2() -> IoResult<()> { // --snip-- }
```

use std::io; use std::io::Write;
等于下面
use std::io::{self, Write};
