namespace aoc_2025_day10;

public struct Machine(uint desiredConfig, List<uint> buttons, List<List<int>> buttonsAsIndices, List<int> jolts)
{
    public readonly uint DesiredConfig = desiredConfig;
    public readonly List<uint> Buttons = buttons;
    public readonly List<List<int>> ButtonsAsIndices = buttonsAsIndices;
    public readonly List<int> Jolts = jolts;
    
    public static Machine ParseLine(string line)
    {
        var elements = line.Split(" ");
        if (elements.Length < 3) throw new Exception("Invalid line");
        var lights = elements[0][1..^1];
        uint desiredConfig = 0;
        for (var i = 0; i < lights.Length; i++)
        {
            if (lights[i] == '#') desiredConfig |= (1u << i);
        }
        var jolts = elements[^1][1..^1].Split(',').Select(int.Parse).ToList();
        var buttons = new List<uint>();
        var indices = new List<List<int>>();
        for (var i = 1; i < elements.Length - 1; i++)
        {
            var btnIdx = elements[i][1..^1].Split(',').Select(int.Parse).ToList();
            indices.Add(btnIdx);
            var button = btnIdx.Aggregate(0u, (acc, item) => acc | (1u << item));
            buttons.Add(button);
        
        }
        return new Machine(desiredConfig, buttons, indices, jolts);
    }
};
