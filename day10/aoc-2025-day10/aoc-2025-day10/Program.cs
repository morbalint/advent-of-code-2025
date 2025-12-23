// See https://aka.ms/new-console-template for more information

using aoc_2025_day10;
using Google.OrTools.LinearSolver;

Console.WriteLine("Hello, World!");

var fileName = "input.txt";
var lines = File.ReadAllLines(fileName);
var machines = lines.Select(Machine.ParseLine).ToList();

int sum = 0;
foreach (var machine in machines)
{
    var solution = SolveJoltage(machine);
    Console.WriteLine($"Solution for {machine.DesiredConfig} is {solution}");
    sum += solution;
}
Console.WriteLine($"Total number of button presses is {sum}");

return;

int SolveJoltage(Machine machine)
{
    var numJolts = machine.Jolts.Count;
    var numButtons = machine.ButtonsAsIndices.Count;
    
    // Create coefficient matrix with all buttons as columns
    int[,] coefficients = new int[numJolts, numButtons];
    for (int i = 0; i < numJolts; i++)
    {
        for (int j = 0; j < numButtons; j++)
        {
            coefficients[i, j] = 0;
        }
    }
    
    // Fill in the coefficient matrix
    foreach ((int buttonIdx, List<int> button) in machine.ButtonsAsIndices.Index())
    {
        foreach (int joltIdx in button)
        {
            coefficients[joltIdx, buttonIdx] = 1;
        }
    }

    var solution = SolveGaussianILP(coefficients, machine.Jolts.ToArray());
    
    if (solution == null)
    {
        throw new Exception("Not solvable");
    }
    
    return solution.Sum();
}


int[]? SolveGaussianILP(int[,] coefficients, int[] results)
{
    int numEquations = results.Length;
    int numVariables = coefficients.GetLength(1);
    
    // Create the linear solver with CBC backend
    Solver solver = Solver.CreateSolver("CBC");
    if (solver == null)
        return null;

    // Create integer variables (non-negative) for all buttons
    Variable[] vars = new Variable[numVariables];
    for (int i = 0; i < numVariables; i++)
    {
        vars[i] = solver.MakeIntVar(0.0, double.PositiveInfinity, $"x{i}");
    }

    // Add constraints (one for each equation/jolt)
    for (int i = 0; i < numEquations; i++)
    {
        Constraint constraint = solver.MakeConstraint(results[i], results[i]);
        for (int j = 0; j < numVariables; j++)
        {
            constraint.SetCoefficient(vars[j], coefficients[i, j]);
        }
    }

    // Minimize sum of all button presses
    Objective objective = solver.Objective();
    for (int i = 0; i < numVariables; i++)
    {
        objective.SetCoefficient(vars[i], 1);
    }
    objective.SetMinimization();

    // Solve
    Solver.ResultStatus resultStatus = solver.Solve();
    
    if (resultStatus != Solver.ResultStatus.OPTIMAL && 
        resultStatus != Solver.ResultStatus.FEASIBLE)
        return null;

    return vars.Select(v => (int)v.SolutionValue()).ToArray();
}


int[]? SolveGaussian(int[,] coefficients, int[] results)
{
    int n = results.Length;
    double[,] matrix = new double[n, n + 1];

    // Copy to augmented matrix
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            matrix[i, j] = coefficients[i, j];
        }
        matrix[i, n] = results[i];
    }

    int[] pivotCol = new int[n]; // Track which column is used as pivot for each row
    for (int i = 0; i < n; i++) pivotCol[i] = -1;

    // Forward elimination
    int currentRow = 0;
    for (int col = 0; col < n && currentRow < n; col++)
    {
        // Find pivot in current column
        int maxRow = -1;
        double maxVal = 0;
        for (int k = currentRow; k < n; k++)
        {
            if (Math.Abs(matrix[k, col]) > maxVal)
            {
                maxVal = Math.Abs(matrix[k, col]);
                maxRow = k;
            }
        }

        // Skip this column if all zeros
        if (maxVal < 1e-10)
        {
            continue;
        }

        // Swap rows
        for (int k = 0; k < n + 1; k++)
        {
            (matrix[maxRow, k], matrix[currentRow, k]) = (matrix[currentRow, k], matrix[maxRow, k]);
        }

        pivotCol[currentRow] = col;

        // Eliminate column
        for (int k = currentRow + 1; k < n; k++)
        {
            double factor = matrix[k, col] / matrix[currentRow, col];
            for (int j = col; j < n + 1; j++)
            {
                matrix[k, j] -= factor * matrix[currentRow, j];
            }
        }

        currentRow++;
    }

    // Check for inconsistency (non-zero in result column with all-zero row)
    for (int i = currentRow; i < n; i++)
    {
        if (Math.Abs(matrix[i, n]) > 1e-10)
        {
            return null; // Inconsistent system
        }
    }

    // Back substitution
    double[] solution = new double[n];
    for (int i = currentRow - 1; i >= 0; i--)
    {
        int col = pivotCol[i];
        solution[col] = matrix[i, n];
        for (int j = col + 1; j < n; j++)
        {
            solution[col] -= matrix[i, j] * solution[j];
        }
        solution[col] /= matrix[i, col];
    }

    // Convert to integers (round to nearest)
    return solution.Select(x => (int)Math.Round(x)).ToArray();
}

uint Solve(Machine machine)
{
    if (machine.Buttons.Any(b => b == machine.DesiredConfig))
    {
        return 1;
    }
    List<uint> combinations = [..machine.Buttons];
    for (uint depth = 1; depth < machine.Buttons.Count; depth++)
    {
        List<uint> nextCombinations = [];
        foreach (var pressed in combinations.SelectMany(combination => machine.Buttons.Select(button => combination ^ button)))
        {
            if (pressed == machine.DesiredConfig)
            {
                return depth + 1u;
            }
            nextCombinations.Add(pressed);
        }
        combinations = nextCombinations;
    }

    return uint.MaxValue;
}
