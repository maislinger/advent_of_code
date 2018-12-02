import java.io.File

fun countMultiples(input: List<String>, value: Int): Int {
    return input.filter {
        val counts = it.groupingBy{it}.eachCount()
        counts.containsValue(value)
    }.count()
}

fun computeSolutionPartOne(input: List<String>): Int {
    val countTwo = countMultiples(input, 2)
    val countThree = countMultiples(input, 3)
    return countTwo * countThree
}

fun countDifferentLetters(a: String, b: String): Int {
    return a.zip(b).filter {
        (x, y) -> x != y
    }.count()
}

fun sameLetters(a: String, b: String): String {
    return a.zip(b).filter {
        (x, y) -> x == y
    }.map {
        (x, _) -> x
    }.joinToString(separator = "")
}

fun computeSolutionPartTwo(input: List<String>): String {
    var result = ""
    loop@ for (i in 0..(input.size - 1)) {
        for (j in (i+1)..(input.size - 1)) {
            val a = input[i]
            val b = input[j]
            if (a.length != b.length) {
                continue
            }
            if (countDifferentLetters(a, b) == 1) {
                result = sameLetters(a, b)
                break@loop
            }
        }
    }
    return result
}

fun main(args: Array<String>) {
    val fileName = args[0]
    val input = File(fileName).readLines(Charsets.UTF_8)
    val solutionPartOne = computeSolutionPartOne(input)
    val solutionPartTwo = computeSolutionPartTwo(input)
    println("solution 1 = $solutionPartOne")
    println("solution 2 = $solutionPartTwo")
}