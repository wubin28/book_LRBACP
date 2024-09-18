package org.example;

import java.util.Optional;

public class OptionalMisuseExample {
  public static Optional<String> getName(int id) {
    // 模拟数据库查询
    if (id == 1) {
      return Optional.of("Alice");
    } else {
      return Optional.empty();
    }
  }

  public static void printName(int id) {
    Optional<String> name = getName(id);
    // 错误使用：直接调用get()而不检查是否存在值
    System.out.println("Name: " + name.get());
  }

  public static void main(String[] args) {
    printName(1); // 这会正常工作
    printName(2); // 这会抛出 NoSuchElementException
  }
}
// Output of './gradlew run':
// > Task :app:run FAILED
// Exception in thread "main" Name: Alice
// java.util.NoSuchElementException: No value present
//         at java.base/java.util.Optional.get(Optional.java:143)
//         at org.example.OptionalMisuseExample.printName(OptionalMisuseExample.java:18)
//         at org.example.OptionalMisuseExample.main(OptionalMisuseExample.java:23)
// 
// FAILURE: Build failed with an exception.