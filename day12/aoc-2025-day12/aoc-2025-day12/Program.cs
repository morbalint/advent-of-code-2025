// Advent of Code 2025 - Day 12
using System.Text.RegularExpressions;

var inputFile = args.Length > 0 ? args[0] : "input.txt";
var lines = File.ReadAllLines(inputFile);

// Parse shapes
var shapes = new Dictionary<int, List<(int x, int y)>>();
var puzzles = new List<(int width, int height, int[] shapeIndices)>();

int i = 0;
while (i < lines.Length)
{
    var line = lines[i].Trim();
    
    // Skip empty lines
    if (string.IsNullOrWhiteSpace(line))
    {
        i++;
        continue;
    }
    
    // Check if it's a shape definition (starts with number and colon)
    var shapeMatch = Regex.Match(line, @"^(\d+):$");
    if (shapeMatch.Success)
    {
        int shapeIndex = int.Parse(shapeMatch.Groups[1].Value);
        var coordinates = new List<(int x, int y)>();
        
        // Read the shape lines
        i++;
        int y = 0;
        while (i < lines.Length && !string.IsNullOrWhiteSpace(lines[i]) && !lines[i].Contains(':'))
        {
            var shapeLine = lines[i];
            for (int x = 0; x < shapeLine.Length; x++)
            {
                if (shapeLine[x] == '#')
                {
                    coordinates.Add((x, y));
                }
            }
            y++;
            i++;
        }
        
        shapes[shapeIndex] = coordinates;
    }
    // Check if it's a puzzle line (format: "widthxheight: counts...")
    else if (line.Contains(':') && line.Contains('x'))
    {
        var puzzleMatch = Regex.Match(line, @"^(\d+)x(\d+):\s*(.+)$");
        if (puzzleMatch.Success)
        {
            int width = int.Parse(puzzleMatch.Groups[1].Value);
            int height = int.Parse(puzzleMatch.Groups[2].Value);
            var counts = puzzleMatch.Groups[3].Value
                .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                .Select(int.Parse)
                .ToArray();
            
            puzzles.Add((width, height, counts));
        }
        i++;
    }
    else
    {
        i++;
    }
}

// Debug: Display parsed shapes
Console.WriteLine("=== PARSED SHAPES ===");
foreach (var (index, coords) in shapes.OrderBy(kvp => kvp.Key))
{
    Console.WriteLine($"Shape {index}: {coords.Count} cells");
}
Console.WriteLine();

// Solve puzzles in parallel
int canFitCount = 0;
var lockObj = new object();
var results = new List<(int puzzleNum, int width, int height, int shapeCount, bool canFit, double elapsed)>();

var parallelOptions = new ParallelOptions 
{ 
    MaxDegreeOfParallelism = 16 
};

Console.WriteLine($"Starting parallel processing with {parallelOptions.MaxDegreeOfParallelism} threads...\n");

Parallel.ForEach(puzzles.Select((puzzle, index) => (puzzle, index)), parallelOptions, item =>
{
    var ((width, height, counts), index) = item;
    int puzzleNum = index + 1;
    
    // Expand counts into actual shape indices to place
    var shapesToPlace = new List<int>();
    for (int shapeIdx = 0; shapeIdx < counts.Length; shapeIdx++)
    {
        for (int count = 0; count < counts[shapeIdx]; count++)
        {
            shapesToPlace.Add(shapeIdx);
        }
    }
    
    var startTime = DateTime.Now;
    var canFit = CanFitShapesDLX(width, height, shapesToPlace.ToArray(), shapes);
    var elapsed = (DateTime.Now - startTime).TotalMilliseconds;
    
    lock (lockObj)
    {
        if (canFit) canFitCount++;
        results.Add((puzzleNum, width, height, shapesToPlace.Count, canFit, elapsed));
        
        // Print progress every 10 puzzles
        if (results.Count % 10 == 0)
        {
            Console.WriteLine($"Progress: {results.Count}/{puzzles.Count} puzzles completed...");
        }
    }
});

// Sort and display results
Console.WriteLine("\n=== RESULTS ===");
foreach (var result in results.OrderBy(r => r.puzzleNum))
{
    Console.WriteLine($"Puzzle {result.puzzleNum}: {result.width}x{result.height}, {result.shapeCount} shapes - {(result.canFit ? "CAN FIT" : "CANNOT FIT")} ({result.elapsed:F0}ms)");
}

Console.WriteLine($"\nTotal: {canFitCount} out of {puzzles.Count} puzzles can fit");


static bool CanFitShapesDLX(int gridWidth, int gridHeight, int[] shapeIndices, Dictionary<int, List<(int x, int y)>> shapes)
{
    // Early exit: if total cells needed exceeds grid size, it's impossible
    int totalNeeded = shapeIndices.Sum(idx => shapes[idx].Count);
    if (totalNeeded > gridWidth * gridHeight)
    {
        return false;
    }
    
    var dlx = new DancingLinks();
    
    // Create columns: one for each shape instance (PRIMARY) + one for each grid cell (SECONDARY)
    for (int i = 0; i < shapeIndices.Length; i++)
    {
        dlx.AddColumn($"shape_{i}", isPrimary: true);  // Must place each shape exactly once
    }
    
    for (int y = 0; y < gridHeight; y++)
    {
        for (int x = 0; x < gridWidth; x++)
        {
            dlx.AddColumn($"cell_{x}_{y}", isPrimary: false);  // Each cell can be used at most once
        }
    }
    
    // Generate rows: each row represents a possible placement
    int rowId = 0;
    int totalRows = 0;
    for (int shapeInstanceIdx = 0; shapeInstanceIdx < shapeIndices.Length; shapeInstanceIdx++)
    {
        int shapeIdx = shapeIndices[shapeInstanceIdx];
        var shape = shapes[shapeIdx];
        var transformations = GetAllTransformations(shape);
        
        for (int transformationIdx = 0; transformationIdx < transformations.Count; transformationIdx++)
        {
            var transformed = transformations[transformationIdx];
            
            // Try all possible positions
            for (int startY = 0; startY < gridHeight; startY++)
            {
                for (int startX = 0; startX < gridWidth; startX++)
                {
                    // Check if shape fits at this position
                    var absoluteCells = transformed.Select(p => (x: startX + p.x, y: startY + p.y)).ToList();
                    if (absoluteCells.All(p => p.x >= 0 && p.x < gridWidth && p.y >= 0 && p.y < gridHeight))
                    {
                        // This placement covers: the shape instance + all cells it occupies
                        var columns = new List<string> { $"shape_{shapeInstanceIdx}" };
                        columns.AddRange(absoluteCells.Select(p => $"cell_{p.x}_{p.y}"));
                        
                        dlx.AddRow(rowId++, columns);
                        totalRows++;
                    }
                }
            }
        }
    }
    
    return dlx.Solve();
}

static List<List<(int x, int y)>> GetAllTransformations(List<(int x, int y)> shape)
{
    var transformations = new HashSet<string>();
    var results = new List<List<(int x, int y)>>();
    
    // Generate all 8 transformations (4 rotations x 2 reflections)
    for (int rotation = 0; rotation < 4; rotation++)
    {
        var rotated = Rotate(shape, rotation);
        var normalized = Normalize(rotated);
        var key = GetShapeKey(normalized);
        if (transformations.Add(key))
        {
            results.Add(normalized);
        }
        
        var reflected = Reflect(rotated);
        normalized = Normalize(reflected);
        key = GetShapeKey(normalized);
        if (transformations.Add(key))
        {
            results.Add(normalized);
        }
    }
    
    return results;
}

static List<(int x, int y)> Rotate(List<(int x, int y)> shape, int times)
{
    var result = shape.ToList();
    for (int i = 0; i < times; i++)
    {
        result = result.Select(p => (-p.y, p.x)).ToList();
    }
    return result;
}

static List<(int x, int y)> Reflect(List<(int x, int y)> shape)
{
    return shape.Select(p => (-p.x, p.y)).ToList();
}

static List<(int x, int y)> Normalize(List<(int x, int y)> shape)
{
    if (!shape.Any()) return shape;
    
    var minX = shape.Min(p => p.x);
    var minY = shape.Min(p => p.y);
    return shape.Select(p => (p.x - minX, p.y - minY)).ToList();
}

static string GetShapeKey(List<(int x, int y)> shape)
{
    return string.Join(";", shape.OrderBy(p => p.y).ThenBy(p => p.x).Select(p => $"{p.x},{p.y}"));
}

// Dancing Links (Algorithm X) implementation
class DancingLinks
{
    private class Node
    {
        public Node Left, Right, Up, Down;
        public ColumnNode Column;
        public int RowId;
        
        public Node()
        {
            Left = Right = Up = Down = this;
        }
    }
    
    private class ColumnNode : Node
    {
        public int Size;
        public string Name;
        public bool IsPrimary;  // Primary columns must be covered, secondary are optional
        
        public ColumnNode(string name, bool isPrimary = true) : base()
        {
            Size = 0;
            Name = name;
            IsPrimary = isPrimary;
            Column = this;
        }
    }
    
    private readonly ColumnNode _header;
    private readonly List<List<int>> _solutions;
    private bool _solutionFound;
    
    public DancingLinks()
    {
        _header = new ColumnNode("header");
        _solutions = new List<List<int>>();
        _solutionFound = false;
    }
    
    public void AddColumn(string name, bool isPrimary = true)
    {
        var col = new ColumnNode(name, isPrimary);
        col.Left = _header.Left;
        col.Right = _header;
        _header.Left.Right = col;
        _header.Left = col;
    }
    
    public void AddRow(int rowId, List<string> columnNames)
    {
        Node firstInRow = null;
        Node lastInRow = null;
        
        foreach (var colName in columnNames)
        {
            var col = FindColumn(colName);
            if (col == null) continue;
            
            var node = new Node
            {
                RowId = rowId,
                Column = col
            };
            
            // Insert into column
            node.Up = col.Up;
            node.Down = col;
            col.Up.Down = node;
            col.Up = node;
            col.Size++;
            
            // Insert into row
            if (firstInRow == null)
            {
                firstInRow = node;
                lastInRow = node;
            }
            else
            {
                node.Left = lastInRow;
                node.Right = firstInRow;
                lastInRow.Right = node;
                firstInRow.Left = node;
                lastInRow = node;
            }
        }
    }
    
    private ColumnNode FindColumn(string name)
    {
        for (var col = _header.Right as ColumnNode; col != _header; col = col.Right as ColumnNode)
        {
            if (col.Name == name) return col;
        }
        return null;
    }
    
    public bool Solve()
    {
        _solutionFound = false;
        Search(new List<int>());
        return _solutionFound;
    }
    
    private void Search(List<int> solution)
    {
        if (_solutionFound) return;
        
        // Check if all primary columns are covered
        bool allPrimaryCovered = true;
        for (var col = _header.Right as ColumnNode; col != _header; col = col.Right as ColumnNode)
        {
            if (col.IsPrimary)
            {
                allPrimaryCovered = false;
                break;
            }
        }
        
        if (allPrimaryCovered)
        {
            _solutionFound = true;
            _solutions.Add(new List<int>(solution));
            return;
        }
        
        // Choose column with smallest size (heuristic)
        var col2 = ChooseColumn();
        if (col2 == null || col2.Size == 0) return; // No solution
        
        Cover(col2);
        
        for (var row = col2.Down; row != col2 && !_solutionFound; row = row.Down)
        {
            solution.Add(row.RowId);
            
            // Cover all columns in this row
            for (var j = row.Right; j != row; j = j.Right)
            {
                Cover(j.Column);
            }
            
            Search(solution);
            
            if (!_solutionFound)
            {
                solution.RemoveAt(solution.Count - 1);
                
                // Uncover all columns in this row
                for (var j = row.Left; j != row; j = j.Left)
                {
                    Uncover(j.Column);
                }
            }
        }
        
        Uncover(col2);
    }
    
    private ColumnNode ChooseColumn()
    {
        var minSize = int.MaxValue;
        ColumnNode minCol = null;
        
        for (var col = _header.Right as ColumnNode; col != _header; col = col.Right as ColumnNode)
        {
            if (col.IsPrimary && col.Size < minSize)
            {
                minSize = col.Size;
                minCol = col;
            }
        }
        
        return minCol;
    }
    
    private void Cover(ColumnNode col)
    {
        col.Right.Left = col.Left;
        col.Left.Right = col.Right;
        
        for (var row = col.Down; row != col; row = row.Down)
        {
            for (var j = row.Right; j != row; j = j.Right)
            {
                j.Down.Up = j.Up;
                j.Up.Down = j.Down;
                j.Column.Size--;
            }
        }
    }
    
    private void Uncover(ColumnNode col)
    {
        for (var row = col.Up; row != col; row = row.Up)
        {
            for (var j = row.Left; j != row; j = j.Left)
            {
                j.Column.Size++;
                j.Down.Up = j;
                j.Up.Down = j;
            }
        }
        
        col.Right.Left = col;
        col.Left.Right = col;
    }
}

