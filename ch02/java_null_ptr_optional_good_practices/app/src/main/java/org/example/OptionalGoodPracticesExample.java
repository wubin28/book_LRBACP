package org.example;

import java.util.Optional;

public class OptionalGoodPracticesExample {
  public static Optional<String> getName(int id) {
    // 模拟数据库查询
    if (id == 1) {
      return Optional.of("Alice");
    } else {
      return Optional.empty();
    }
  }

  public static void printName(int id) {
    getName(id)
        .ifPresentOrElse(
            name -> System.out.println("Name: " + name),
            () -> System.out.println("No name found for id: " + id));
  }

  public static void main(String[] args) {
    printName(1); // 这会打印名字
    printName(2); // 这会打印未找到名字的消息
  }
}
// Output:
// Name: Alice
// No name found for id: 2
