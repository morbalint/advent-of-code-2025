def process_matrix(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    # Parse matrix rows (all lines except the last one)
    matrix = []
    for line in lines[:-1]:
        row = [int(x.strip()) for x in line.split()]
        matrix.append(row)

    # Parse operators from the last line
    operators = lines[-1].split()

    # Apply operators column-wise
    results = []
    num_cols = len(matrix[0])

    for col_idx in range(num_cols):
        # Extract column values
        column = [matrix[row_idx][col_idx] for row_idx in range(len(matrix))]

        # Apply operator
        if operators[col_idx] == '+':
            result = sum(column)
        elif operators[col_idx] == '*':
            result = 1
            for val in column:
                result *= val
        else:
            raise Exception(f"Invalid operator: {operators[col_idx]}")

        results.append(result)

    # Sum all results
    return sum(results)


if __name__ == "__main__":
    total = process_matrix('input.txt')
    print(f"Total: {total}")