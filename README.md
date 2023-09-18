# Financial-analyzer
Rust program that calculates the present value of a series of future cash flows, which is a common financial calculation used in various financial applications like investment valuation and bond pricing. This program takes into account cash flows occurring at different time periods and discounts them to their present value at a given discount rate

## Features

- Accepts user input for future cash flows.
- Allows users to specify the discount rate.
- Calculates the present value of cash flows using the time value of money concept.
- Provides a user-friendly interface for interactive input.

## How to Use

1. Make sure you have Rust installed. If not, you can download it from [Rust's official website](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine:

   ```sh
   git clone https://github.com/divya-r-b/financial-analyzer.git

## Example 
```plaintext
Finance Analyzer

Enter the cash flow (or 'q' to finish):
100
Enter the cash flow (or 'q' to finish):
150
Enter the cash flow (or 'q' to finish):
200
Enter the cash flow (or 'q' to finish):
250
Enter the cash flow (or 'q' to finish):
q
Enter the discount rate (e.g., 0.1 for 10%):
0.1
Present value of cash flows: $415.30
