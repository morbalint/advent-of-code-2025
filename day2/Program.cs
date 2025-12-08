using System.Globalization;

// part1 solution: 23039913998

Console.WriteLine("Hello, World!");
string inputTxt = File.ReadAllText("input.txt");
// const string inputTxt = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
string[] rangesString = inputTxt.Split(',');
var ranges = rangesString.Select(x => x.Split('-')).Select(x => 
    new { start = decimal.Parse(x[0]), end = decimal.Parse(x[1]) }
).ToArray();
decimal sum = ranges.Sum(range =>
{
    decimal partialSum = 0;
    for (decimal id = range.start; id <= range.end; id++)
    {
        if (IsFakeIdV2(id))
        {
            partialSum += id;
        }
    }
    return partialSum;
});
Console.WriteLine(sum);

return;

bool IsFakeId(decimal id)
{
    string idStr = id.ToString(CultureInfo.InvariantCulture);
    if (idStr.Length % 2 == 1)
    {
        return false;
    }

    ulong dividend = 1;
    for (int i = 0; i < (idStr.Length / 2); i++)
    {
        dividend *= 10;
    }

    dividend += 1;
    
    return id % dividend == 0;
}

bool IsFakeIdV2(decimal id)
{
    string idStr = id.ToString(CultureInfo.InvariantCulture);

    for (int patterLength = 1; patterLength <= idStr.Length / 2; patterLength++)
    {
        if (idStr.Length % patterLength != 0)
        {
            continue;
        }
        string pattern = idStr[..patterLength];
        string rebuiltId = string.Concat(Enumerable.Repeat(pattern, idStr.Length / patterLength));
        if (rebuiltId == idStr)
        {
            return true;
        }
    }
    return false;
}