# matrix

This project is a Rust library that implements two core classes: Vector and Matrix. This library is designed to simplify mathematical computations involving linear algebra. It offers robust, efficient, and easy-to-use abstractions for developers working with vectors and matrices in Rust.

This library was built using a Test-Driven Development (TDD) methodology, ensuring reliability and correctness from the start. All features were implemented with accompanying unit tests, making the codebase robust and well-tested.
All test files are organized within a dedicated tests folder. The project can be tested using Rust's built-in test runner with the command:

```bash
cargo test
```

## features

<!-- add sub scale -->

You can use :

Vector / Matrix Addition, Subtraction and Scalar Multiplication.

```rust
    let mut v1 = Vector::new([13, 4, 2]);
    let v2 = Vector::new([29, 1, 1]);
    v1.add(&v2);
```
```rust
    let mut v1 = Vector::new([13, 4, 2]);
    let v2 = Vector::new([12, 1, 1]);
    v1.sub(&v2);
```
```rust
    let mut v1 = Vector::new([13, 4, 2]);
    let x = 2;
    v1.scl(x);
```

<!--  surchare d'orateurs -->

Arithmetic Operators: addition (+), subtraction (-), multiplication (*), and division (/), equality operator (==), for both vectors and matrices.

```rust
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 + v2;
```
```rust
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 - v2;
```
```rust
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 * v2;
```
```rust
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    let v3 = v1 / v2;
```
```rust
    let v1 = Vector::new([2, 4]);
    let v2 = Vector::new([-2, -4]);
    if v1 == v2 {
		...
	}
	...
```

###linear interpolation
linear interpolation is a method of curve fitting using linear polynomials to construct new data points within the range of a discrete set of known data points.

```rust
    let x = 2.0;
    let y = 8.0;
    _ = lerp(x, y, 0.5);
```


```rust
    let m1 = Matrix::new([[2.0, 2.0], [4.0, 2.0]]);
    let m2 = Matrix::new([[4.0, 4.0], [3.0, 1.0]]);

    _ = lerp(m1.clone(), m2.clone(), 0.5);
```

###linear combination
linear combination or superposition is an expression constructed from a set of terms by multiplying each term by a constant and adding the results

```rust
    let v1 = Vector::new([5.0, 2.0, 13.0]);
    let v2 = Vector::new([1.0, 4.0, 3.0]);
    let v3 = Vector::new([6.0, 10.0, 2.0]);

    let x1 = 0.5;
    let x2 = 0.2;
    let x3 = 0.8;

    let v4 = vector::linear_combination(&[v1, v2, v3], &[x1, x2, x3]);
```

###Dot product
the dot product or scalar product is an algebraic operation that takes two equal-length sequences of numbers (usually coordinate vectors), and returns a single number.

```rust
    let v1 = Vector::new([0.0, 0.0]);
    let v2 = Vector::new([1.0, 1.0]);
    _ = v1.dot(&v2);
```

more features to come...
