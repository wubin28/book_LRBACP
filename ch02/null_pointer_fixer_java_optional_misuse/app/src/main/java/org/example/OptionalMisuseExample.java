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
    try {
      printName(1); // 这会正常工作
      printName(2); // 这会抛出 NoSuchElementException
    } catch (Exception e) {
      System.out.println(
          "An exception occurred: " + e.getClass().getSimpleName() + " - " + e.getMessage());
    }
  }
}
// Output:
// Name: Alice
// An exception occurred: NoSuchElementException - No value present
