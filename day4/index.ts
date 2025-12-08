import * as fs from 'fs';
import * as path from 'path';

function loadGrid(filename: string): boolean[][] {
    const filePath = path.join(__dirname, filename);
    const content = fs.readFileSync(filePath, 'utf-8');
    return content.trim().split('\n').map(line =>
        line.split('').map(char => char === '@')
    );
}

function countFreeNeighbors(grid: boolean[][], row: number, col: number): number {
    const directions = [
        [-1, -1], [-1, 0], [-1, 1],
        [0, -1],           [0, 1],
        [1, -1],  [1, 0],  [1, 1]
    ];

    let freeCount = 0;
    for (const [dr, dc] of directions) {
        const newRow = row + dr;
        const newCol = col + dc;
        // Out of bounds counts as free, or check if cell is free (false)
        if (newRow < 0 || newRow >= grid.length ||
            newCol < 0 || newCol >= grid[0].length ||
            !grid[newRow][newCol]) {
            freeCount++;
        }
    }
    return freeCount;
}

function printMap(grid: boolean[][]): void {
    let counter = 0;
    let previousCounter = 0;
    let nextGrid = grid.map(row => [...row]);  // deep copy
    let iteration = 0;
    do {
        previousCounter = counter;
        for (let row = 0; row < grid.length; row++) {
            let line = '';
            for (let col = 0; col < grid[row].length; col++) {
                if (!grid[row][col]) {
                    line += '.';  // free
                    nextGrid[row][col] = false;
                } else if (countFreeNeighbors(grid, row, col) >= 5) {
                    line += 'x';  // occupied with at least 5 free neighbors
                    counter++;
                    nextGrid[row][col] = false;
                } else {
                    line += '@';  // fully occupied
                    nextGrid[row][col] = true;
                }
            }
            console.log(line);
        }
        console.log('---');
        console.log("Iteration: " + ++iteration);
        console.log("Count: " + counter);
        grid = nextGrid.map(row => [...row]); // deep copy
    } while (counter !== previousCounter);
}

const grid = loadGrid('input.txt');
printMap(grid);
