// See https://aka.ms/new-console-template for more information

using aoc_2025_day11;

var graph = Graph.Parse("input.txt");
            
// Calculate paths for both orderings: svr->dac->fft->out and svr->fft->dac->out
var svrToDac = Graph.CountAllPaths(graph, "svr", "dac");
Console.WriteLine($"Paths svr->dac: {svrToDac}");
var dacToFft = Graph.CountAllPaths(graph, "dac", "fft");
Console.WriteLine($"Paths dac->fft: {dacToFft}");
var fftToOut = Graph.CountAllPaths(graph, "fft", "out");
Console.WriteLine($"Paths fft->out: {fftToOut}");
Console.WriteLine($"Routes via svr->dac->fft->out: {svrToDac * dacToFft * fftToOut}");

Console.WriteLine();
var svrToFft = Graph.CountAllPaths(graph, "svr", "fft");
Console.WriteLine($"Paths svr->fft: {svrToFft}");
var fftToDac = Graph.CountAllPaths(graph, "fft", "dac");
Console.WriteLine($"Paths fft->dac: {fftToDac}");
var dacToOut = Graph.CountAllPaths(graph, "dac", "out");
Console.WriteLine($"Paths dac->out: {dacToOut}");
Console.WriteLine($"Routes via svr->fft->dac->out: {svrToFft * fftToDac * dacToOut}");
Console.WriteLine();
            
long totalRoutes = (svrToDac * dacToFft * fftToOut) + (svrToFft * fftToDac * dacToOut);
Console.WriteLine($"Total routes from 'svr' to 'out' via 'dac' and 'fft' (any order): {totalRoutes}");

namespace aoc_2025_day11
{
    using System.Collections.Generic;
    using System.IO;

    internal static class Graph
    {
        public static Dictionary<string, List<string>> Parse(string filePath)
        {
            var graph = new Dictionary<string, List<string>>();
            foreach (var line in File.ReadLines(filePath))
            {
                var trimmed = line.Trim();
                if (string.IsNullOrEmpty(trimmed)) continue;
                var parts = trimmed.Split(':');
                if (parts.Length != 2) continue;
                var src = parts[0].Trim();
                var dests = parts[1].Split([' '], StringSplitOptions.RemoveEmptyEntries);
                graph[src] = new List<string>(dests);
            }
            return graph;
        }

        public static long CountAllPaths(Dictionary<string, List<string>> graph, string start, string end)
        {
            var memo = new Dictionary<string, long>();
            
            long Dfs(string node)
            {
                if (node == end)
                    return 1;
                
                if (memo.TryGetValue(node, out var dfs))
                    return dfs;
                
                long count = 0;
                if (graph.TryGetValue(node, out var value))
                {
                    foreach (var next in value)
                    {
                        count += Dfs(next);
                    }
                }
                
                memo[node] = count;
                return count;
            }
            
            return Dfs(start);
        }
    }
}