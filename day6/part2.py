def process_matrix(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    operations = lines[-1].split()

    max_line_length = max(len(line.rstrip()) for line in lines)
    matrix_lines = [line.rstrip().ljust(max_line_length) for line in lines[:-1]]
    matrix = [[char for char in line] for line in matrix_lines]

    print("Original matrix:")
    for row in matrix:
        print(row)

    # Transpose the matrix
    matrix = [[matrix[row][col] for row in range(len(matrix))] for col in range(len(matrix[0]))]

    print("Transposed matrix:")
    for row in matrix:
        print(row)

    # Convert each row to a string
    transposed_strings = [''.join(row) for row in matrix]

    print("Transposed strings:")
    for s in transposed_strings:
        print(s)

    # Group strings using empty strings as separators
    groups = []
    current_group = []
    for s in transposed_strings:
        if s.strip() == '':  # Empty string separator
            if current_group:
                groups.append(current_group)
                current_group = []
        else:
            current_group.append(s)
    if current_group:  # Don't forget the last group
        groups.append(current_group)

    print("Groups:")
    for i, group in enumerate(groups):
        print(f"Group {i + 1}: {group}")

    # Parse each string in every group as integers
    parsed_groups = []
    for group in groups:
        parsed_group = [int(s) for s in group]
        parsed_groups.append(parsed_group)

    print("Parsed groups:")
    for i, group in enumerate(parsed_groups):
        print(f"Group {i + 1}: {group}")

    # Aggregate each group with its corresponding operator
    results = []
    for i, (group, op) in enumerate(zip(parsed_groups, operations)):
        if op == '+':
            result = sum(group)
        elif op == '*':
            result = 1
            for num in group:
                result *= num
        else:
            raise ValueError(f"Unknown operator: {op}")

        results.append(result)
        print(f"Group {i + 1} with operator '{op}': {result}")

    return sum(results)


if __name__ == "__main__":
    total = process_matrix('input.txt')
    print(f"Total: {total}")