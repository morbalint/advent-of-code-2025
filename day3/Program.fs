// For more information see https://aka.ms/fsharp-console-apps
open System.Collections.Generic

printfn "Hello from F#"

let NumDigits = 12

let rec UpdateState (state: LinkedList<decimal>) (newJolt: decimal) (currentNode: LinkedListNode<decimal>): LinkedList<decimal> =
    if currentNode = null then
        state
    else
        if currentNode.Previous.Value < currentNode.Value then
            state.Remove(currentNode.Previous)
            state.AddLast(newJolt) |> ignore
            state
        else if currentNode.Next = null && newJolt > currentNode.Value then
            state.Remove(currentNode)
            state.AddLast(newJolt) |> ignore
            state
        else
            UpdateState state newJolt currentNode.Next

let nextBestJolt (numDigits: int) (currentBest: LinkedList<decimal>) (newJolt: decimal) : LinkedList<decimal> =
    if currentBest.Count < numDigits then
        currentBest.AddLast(newJolt) |> ignore
        currentBest
    else
        UpdateState currentBest newJolt currentBest.First.Next

let rec getBestJolt (chars: char list) (currentBest: LinkedList<decimal>) : LinkedList<decimal> =
    match chars with
    | [] -> currentBest
    | head :: tail ->
        let newJolt = decimal (int head - int '0')
        let updatedBest = nextBestJolt NumDigits currentBest newJolt
        getBestJolt tail updatedBest

let rec sumBestJolts (lines: string list) (sum: decimal) : decimal =
    match lines with
    | [] -> sum
    | head :: tail ->
        let chars = head |> Seq.toList
        let bestJolts = getBestJolt chars (LinkedList<decimal>())
        let bestJolt = bestJolts |> Seq.reduce (fun a b -> (a * (decimal 10)) + b)
        sumBestJolts tail (sum + bestJolt)

[<EntryPoint>]
let main argv =
    let lines = System.IO.File.ReadAllLines("input.txt")
    let sum = sumBestJolts (List.ofArray lines) (decimal 0)
    printfn $"Sum: {sum}"
    0 // return an integer exit code
