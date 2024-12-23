# Leetcode exercises with Rust

## [Add Binary](https://leetcode.com/problems/add-binary/description/)
- Given two binary strings _a_ and _b_, return their sum as a binary string.

### Approach

#### 1. **Input Preparation**
- Convert the input strings `a` and `b` into vectors of characters (`array1` and `array2`) in reverse order.
    - This allows processing the binary strings from the least significant bit (rightmost bit) to the most significant bit (leftmost bit).
- Initialize a `carry` variable to `false`, representing whether there's a carry-over during addition.

#### 2. **Equalizing Lengths**
- Determine the maximum length of the two binary strings.
- Resize both vectors to this length, padding with `'0'` if one string is shorter than the other.
    - This ensures that both vectors have the same number of bits, simplifying the addition logic.

#### 3. **Bit-by-Bit Addition**
- Iterate through the vectors (`array1` and `array2`) element by element (from the least significant bit to the most significant bit).
- For each pair of bits, perform the binary addition, taking the `carry` into account:
    - If one bit is `'0'` and the other is `'1'`, the result is `'1'` unless there's a carry, in which case it becomes `'0'`.
    - If both bits are `'1'`, the result is `'0'` if there was no carry, or `'1'` if there was a carry. In either case, a new carry is generated.
    - If both bits are `'0'`, the result is `'1'` only if there's a carry; otherwise, it remains `'0'`.
    - If one bit is `'1'` and the other is `'0'`, the result is determined by whether there's a carry.
- Update `array1[i]` with the computed result for each bit.

#### 4. **Handle Final Carry**
- After iterating through all bits, if there's a carry left, append `'1'` to `array1` to account for the final carry.

#### 5. **Output the Result**
- Reverse `array1` to restore the correct order (most significant bit to the least significant bit).
- Convert `array1` to a `String` and return it as the result.

---

### Key Points

#### **Binary Addition Logic**
The addition of binary digits and the carry follows these rules:
- `0 + 0 = 0` (carry remains 0)
- `1 + 0 = 1` or `0 + 1 = 1` (carry remains 0)
- `1 + 1 = 0` with a carry of `1`
- When a carry is added, it behaves as a third input to the addition.

#### **Reverse Processing**
By reversing the input strings, the solution processes the binary digits starting from the least significant bit, which simplifies handling the carry propagation.

#### **Efficiency**
The solution processes each bit once, resulting in a time complexity of \(O(\max(n, m))\), where \(n\) and \(m\) are the lengths of the input strings.

---

## [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/description/)
- You are climbing a staircase. It takes _n_ steps to reach the top. 
- Each time you can either climb _1_ or _2_ steps. In how many distinct ways can you climb to the top?

### Approach

#### 1. **Base Cases**
- If there is only 1 step (`n == 1`), there is only 1 way to climb it.
- If there are 2 steps (`n == 2`), there are 2 ways to climb it:
    - Take two 1-step moves: `(1, 1)`
    - Take one 2-step move: `(2)`

#### 2. **Dynamic Programming (DP) Optimization**
- For larger values of `n`, use a dynamic programming approach to calculate the number of ways iteratively:
    - Define `prev2` as the number of ways to climb `n-2` steps (initially 1).
    - Define `prev1` as the number of ways to climb `n-1` steps (initially 2).
- For each step `i` from 3 to `n`:
    - Compute the current number of ways as the sum of `prev1` and `prev2`:
      ```
      current = prev1 + prev2;
      ```
    - Update `prev2` to hold the value of `prev1`.
    - Update `prev1` to hold the value of `current`.

#### 3. **Output the Result**
- After iterating through all steps, the variable `prev1` will hold the total number of ways to climb `n` steps.

---

### Key Points

#### **Relation to Fibonacci Sequence**
The problem has a recursive nature similar to the Fibonacci sequence:
- \( f(n) = f(n-1) + f(n-2) \), where \( f(n) \) represents the number of ways to climb `n` steps.
- By storing the last two computed values (`prev1` and `prev2`), the solution avoids recomputing values and reduces time complexity.

#### **Efficiency**
- **Time Complexity**: \(O(n)\), where \(n\) is the number of steps. The algorithm iterates once through the steps.
- **Space Complexity**: \(O(1)\), since only two variables (`prev1` and `prev2`) are used to store intermediate results.

---
