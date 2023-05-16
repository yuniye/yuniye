<div align="center" style="display:grid;place-items:center;">
<p>
    <a href="https://vlang.io/" target="_blank"><img width="80" src="https://github.com/yuniye/yuniye/assets/53147395/1d4efbc6-0748-43b9-9fbf-9d8fb72f8606?sanitize=true" alt="Yuniye logo"></a>
</p>
<h1>The Yuniye Programming Language</h1>

[yuniye.github.io](https://yuniye.github.io) | [Docs](https://github.com/yuniye/yuniye/blob/main/doc/docs.md) | [Changelog](https://github.com/yuniye/yuniye/blob/main/CHANGELOG.md) | [Speed](https://github.com/yuniye/yuniye/actions?query=workflow%3ABenchmark) | [Contributing](https://github.com/yuniye/yuniye/blob/main/CONTRIBUTING.md)
</div>

<div align="center" style="display:grid;place-items:center;">

[![Sponsor](https://camo.githubusercontent.com/da8bc40db5ed31e4b12660245535b5db67aa03ce/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d53706f6e736f72266d6573736167653d254532253944254134266c6f676f3d476974487562)](https://github.com/sponsors/divineniiquaye)
[![Twitter](https://img.shields.io/badge/follow-%40Yuniye_Official-1DA1F2?logo=twitter&style=flat&logoColor=white&color=1da1f2)](https://twitter.com/Yuniye-Official)
![PR Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)
![License](https://img.shields.io/github/license/yuniye/yuniye)
</div>

## Features

- **Zero-cost abstractions**: Yuniye allows you to write high-level, expressive code without sacrificing performance.

- **Move semantics**: This enables better performance and avoids unnecessary copying.

- **Guaranteed memory safety**: Ensuring that your code is safe from common memory-related errors, such as null pointer dereferences and memory leaks.

- **Threads without data races**: Ensuring thread safety without the need for locks or explicit synchronization primitives.

- **Generics**: Powerful generic programming capabilities, allowing you to write reusable and type-safe code that can operate on different data types.

- **Pattern matching**: Enabling concise and expressive handling of complex data structures and control flow.

- **Type inference**: A strong type inference system that automatically deduces the types of expressions, reducing the need for explicit type annotations.

- **Highly embeddable**: Designed to be easily embedded within existing applications, making it suitable for scripting, extensibility, and creating domain-specific languages.

- **Efficient bindings**: Yuniye offers efficient and seamless bindings to C++, Rust, and Python, allowing you to leverage existing libraries and take advantage of their performance and functionality.

- **High Performance**: Yuniye aims to deliver excellent performance, surpassing popular languages like PHP, Python, Ruby, and Java. It utilizes efficient runtime mechanisms and optimization techniques to achieve optimal execution speed.

## Getting Started

To start using Yuniye, follow the instructions below:

1. **Installation**: Download the latest release of Yuniye for your operating system from the [our GitHub Release Page](https://github.com/yuniye/yuniye/releases).

2. **Hello, World!**: Write your first Yuniye program by creating a new file, `hello.yy`, and entering the following code:

   ```php
   print("Hello, World!");
   ```

3. **Compile and Run**: Open a terminal and navigate to the directory containing your `hello.yy` file. Run the following command to execute the program:

   ```shell
   yy hello.yy
   ```
   or
   ```shell
   yuniye hello.yy
   ```

   You should see the output: `Hello, World!`.

4. **Explore**: Familiarize yourself with the Yuniye language by checking out the [Yuniye language reference](https://github.com/yuniye/yuniye/blob/main/doc/docs.md).

## Documentation

- [Language Reference](https://github.com/yuniye/yuniye/blob/main/doc/reference.md): Learn about Yuniye's syntax, features, and standard library. (Not Beginner friendly)

- [Standard Library](https://github.com/yuniye/yuniye/blob/main/doc/modules.md): Explore the functionality provided by Yuniye's standard library modules.

- [Embedding Guide](https://github.com/yuniye/yuniye/blob/main/doc/embedding.md): Discover how to embed Yuniye within your existing applications and projects.

- [Contributing](https://github.com/yuniye/yuniye/blob/main/CONTRIBUTING.md): Contribute to the development of Yuniye and join our vibrant community.

## Examples

Yuniye's flexibility and expressive power can be demonstrated through the following code snippets:

**Example 1: Fibonacci Sequence**

```rust
fibonacci: fn(n: int): int -> return n < 2 ? n : fibonacci(n - 1) + fibonacci(n - 2);
result = fibonacci(10);
print(result);
```

**Example 2: Pattern Matching**

```rust
Fruit: enum {
    Apple(int),
    Banana,
    Orange,
}

print_fruit: fn(fruit: Fruit) {
    match fruit {
        .Apple(seeds) => print("Apple with ", seeds, " seeds"),
        .Banana => print("Banana"),
        .Orange => print("Orange"),
    }
}

apple = Fruit::Apple(10);
banana = Fruit::Banana;
orange = Fruit::Orange;

// This is handled as if you wrote it as:
// print_fruit(apple);
// print_fruit(banana);
// print_fruit(orange);
std::format::repeat(print_fruit, apple, banana, orange);
```

## Community and Support

Join the Yuniye community to get support, share your experiences, and collaborate with fellow developers:

- [GitHub Discussions](https://github.com/yuniye/yuniye/discussions): Participate in discussions, ask questions, and share knowledge with the community.

- [GitHub Repository](https://github.com/yuniye/yuniye): Contribute to the development of Yuniye by reporting issues or submitting pull requests.

- [Twitter](https://twitter.com/Yuniye_Official): Follow Yuniye on Twitter to stay updated with the latest news and announcements.

## License

Yuniye is released under the [MIT License](LICENSE), granting you the freedom to use, modify, and distribute the language.

## Acknowledgments

Yuniye stands on the shoulders of giants. We would like to express our gratitude to the developers and contributors behind the programming languages that have inspired Yuniye's design and features.

Currently more 75% of the codebase was written by GPT-4. Thanks to OpenAI for developing such a capable AI model.

*Note: The Yuniye logo is subject to copyright and is provided here for illustrative purposes only.*
