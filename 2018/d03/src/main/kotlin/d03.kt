import java.io.File

data class Coordinate(val x: Int, val y: Int)

data class Claim(val id: Int, val leftEdge: Int, val topEdge: Int, val width: Int, val height: Int) {
    fun coordinates(): Sequence<Coordinate> {
        return generateSequence(Coordinate(this.leftEdge, this.topEdge)) {
            var x = it.x + 1
            var y = it.y
            if (x > this.leftEdge + this.width - 1) {
                x = this.leftEdge
                y += 1
            }
            if (y > this.topEdge + this.height - 1) {
                return@generateSequence null
            } else {
                return@generateSequence Coordinate(x, y)
            }
        }
    }
}

data class Fabric(var claimedFields: HashMap<Coordinate, Int> = HashMap()) {
    fun add(seq: Sequence<Coordinate>) {
        seq.forEach {
            val value = this.claimedFields.getOrDefault(it, 0)
            this.claimedFields.put(it, value + 1)
        }
    }

    fun multipleClaims(): Int {
        return this.claimedFields.values.map {
            if (it >= 2) {
                return@map 1
            } else {
                return@map 0
            }
        }.sum()
    }

    fun uniqueClaim(seq: Sequence<Coordinate>): Boolean {
        seq.forEach {
            val value = this.claimedFields.getOrDefault(it, 0)
            if (value > 1) {
                return false
            }
        }
        return true
    }
}

fun parseInput(input: List<String>): List<Claim> {
    val pattern = """#(\d+) @ (\d+),(\d+): (\d+)x(\d+)""".toRegex()
    return input.mapNotNull {
        val found = pattern.find(it)
        val id = found?.groups?.get(1)?.value?.toInt()
        val leftEdge = found?.groups?.get(2)?.value?.toInt()
        val topEdge = found?.groups?.get(3)?.value?.toInt()
        val width = found?.groups?.get(4)?.value?.toInt()
        val height = found?.groups?.get(5)?.value?.toInt()
        if (id == null || leftEdge == null || topEdge == null || width == null || height == null) {
            return@mapNotNull null
        } else {
            return@mapNotNull Claim(id, leftEdge, topEdge, width, height)
        }
    }.toList()
}

fun computeSolutionPartOne(input: List<String>): Int {
    val claims = parseInput(input)
    var fabric = Fabric()
    claims.forEach {
        fabric.add(it.coordinates())
    }
    return fabric.multipleClaims()
}

fun computeSolutionPartTwo(input: List<String>): Int {
    val claims = parseInput(input)
    var fabric = Fabric()
    claims.forEach {
        fabric.add(it.coordinates())
    }
    claims.forEach {
        if (fabric.uniqueClaim(it.coordinates())) {
            return it.id
        }
    }
    throw Exception("No Solution!")
}

fun main(args: Array<String>) {
    val fileName = args[0]
    val input = File(fileName).readLines(Charsets.UTF_8)
    val solutionPartOne = computeSolutionPartOne(input)
    val solutionPartTwo = computeSolutionPartTwo(input)
    println("solution 1 = $solutionPartOne")
    println("solution 2 = $solutionPartTwo")
}
