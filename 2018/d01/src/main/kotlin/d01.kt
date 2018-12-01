import java.io.File

fun parseInput(input: String): Array<Int> {
    return input.split("\n").mapNotNull{it.toIntOrNull()}.toTypedArray()
}

fun computeSolutionPartOne(input: String): Int {
    val changes = parseInput(input)
    return changes.sum()
}

fun computeSolutionPartTwo(input: String): Int {
    val changes = parseInput(input)
    var index = 0
    var frequency = 0
    var visited = HashSet<Int>()
    while (true) {
        if (visited.contains(frequency)) {
            return frequency
        }
        visited.add(frequency)
        frequency += changes[index]
        index += 1
        if (index >= changes.size) {
            index = 0
        }

    }
}

fun main(args: Array<String>) {
    val fileName = args[0]
    val input = File(fileName).readText(Charsets.UTF_8)
    val solutionPartOne = computeSolutionPartOne(input)
    val solutionPartTwo = computeSolutionPartTwo(input)
    println("solution 1 = $solutionPartOne")
    println("solution 2 = $solutionPartTwo")
}
