# Extended Euclidean Algorithm

In modular arithmetic, Extended Euclidean Algorithm is a way to find the multiplicative inverse of an element from the prime field. It is a common method to find inverse in cryptography.

However, in a field/group, not every element has it's multiplicative inverse such that x\*x^-1=1. The element need to fulfill the condition such that `gcd(x,n)=1`, where x is an element in the field/group, n is the modulo of the field. Given such a condition, if n is a prime number, then the factor of n must be either n or 1. Hence, if we choose n as a prime number, the element in field/group must satisfy the condition. Therefore, in cryptography, prime field is used and the prime number is often very large in order to achieve security based on some assumption. (For more details, can check out Discrete Logarithm Assumption)

## Algorithm explained

In **Euclidean Algorithm**, we know that the gcd of two integers `x`,`n` can be written in the form of following equation: `n = a*x + b `, where `a`,`b` is also an integer, by doing repeated division algorithm.
By checking if the `gcd(n,x)=1` condition is safistied, we can make sure that `x` indeed has inverse in mod `n` operation. Hence, based on that foundation, we record the element `a` from `n = a*x + b `in every step of Euclidean Algorithm, and applying the formula `p_i = p_(i-2) - p_(i-1)*a_(i-2)` to compute `p_i`, given `p_0 = 0`,`p_1=1`.  
If we need `s` step to compute Euclidean Algorithm, then `s-1` step is required to compute the inverse of `x`.

Reference: http://www-math.ucdenver.edu/~wcherowi/courses/m5410/exeucalg.html

## Try it locally

### Clone the Repo

Open your terminal, and run

```
git clone https://github.com/zengzengzenghuy/cryptography-with-Rust.git
cd Extended-Euclidean-Algorithm
```

### Run the program

```
cargo build
cargo run
```

Enter the modulus and the number which you want to find the inverse for.
If the number has an inverse, it will be shown on terminal; else `"No multiplicative Inverse!"` will be shown on terminal.  
**Sample**

```
Enter the modulus:
26
Enter the Number:
15
inverse is 7
```
